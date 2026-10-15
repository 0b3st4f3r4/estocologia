"""Armazém temporal do H3 (agent-f2-data): registro de eventos em Parquet, lido sob um relógio.

Cada fato entra como evento com `t_disp`, o instante (segundos UTC) em que ele passa a ser conhecível.
Um atraso de chegada só existe depois que o avião pousa; um feriado existe desde o calendário. A
leitura de um agente é sempre uma janela (desde, até] de t_disp, e o armazém recusa:
- SELECT * e projeção que o EXPLAIN não confirme (herança do H2);
- `ate` além do relógio (ler o futuro);
- qualquer janela que passe do corte do cofre fora do tempo antes do F3 apresentar o hash do
  pré-registro, e uma segunda abertura do cofre;
- gasto acima do orçamento do agente.

As tabelas são particionadas por mês de t_disp e ordenadas por t_disp, com grupos de linhas pequenos.
Os bytes cobrados são os dos grupos de linhas cujo intervalo [min, max] de t_disp cruza a janela, só
nas colunas pedidas: é a mesma poda que o DuckDB faz pelas estatísticas, e a regra de escala do PLAN
(§5.1) vira mensurável: o replay inteiro deve varrer cada tabela O(1) vezes, não uma vez por previsão.
"""

from __future__ import annotations

import re
from datetime import datetime, timezone
from pathlib import Path

import duckdb
import numpy as np
import pandas as pd
import pyarrow as pa
import pyarrow.parquet as pq


class Recusa(PermissionError):
    pass


class Relogio:
    """Relógio do replay. Só anda para a frente."""

    def __init__(self, t0: int):
        self.agora = int(t0)

    def avancar(self, t: int) -> None:
        if int(t) < self.agora:
            raise Recusa(f"relógio não volta: {t} < {self.agora}")
        self.agora = int(t)


def _mes(t: int) -> str:
    return datetime.fromtimestamp(int(t), tz=timezone.utc).strftime("%Y-%m")


class Armazem:
    def __init__(self, raiz: Path, relogio: Relogio, corte_cofre: int, orcamento_por_agente: int = 2_000_000_000):
        self.raiz = Path(raiz)
        self.relogio = relogio
        self.corte = int(corte_cofre)
        self.orcamento = orcamento_por_agente
        self.gasto: dict[str, int] = {}
        self.registro: list[dict] = []
        self.destrancado: str | None = None
        self.con = duckdb.connect()
        self._meta: dict[str, list] = {}      # tabela -> [(arquivo, mes, [(tmin, tmax, {col: bytes})])]
        self._explain_ok: dict[tuple, list] = {}

    # ---------- ingestão ----------
    def ingerir(self, tabela: str, df: pd.DataFrame, linhas_por_grupo: int = 256) -> dict:
        if "t_disp" not in df.columns:
            raise ValueError(f"{tabela}: falta t_disp")
        df = df.sort_values("t_disp", kind="stable").reset_index(drop=True)
        df["t_disp"] = df["t_disp"].astype("int64")
        base = self.raiz / tabela
        meses = df["t_disp"].map(_mes)
        for mes, parte in df.groupby(meses, sort=True):
            d = base / f"mes={mes}"
            d.mkdir(parents=True, exist_ok=True)
            tmp = d / "parte.parquet.tmp"
            pq.write_table(pa.Table.from_pandas(parte, preserve_index=False), tmp, row_group_size=linhas_por_grupo)
            tmp.replace(d / "parte.parquet")
        self._indexar(tabela)
        return {"tabela": tabela, "linhas": int(len(df)), "bytes_tabela": self.bytes_tabela(tabela),
                "grupos": sum(len(g) for _, _, g in self._meta[tabela]), "colunas": [c for c in df.columns]}

    def _indexar(self, tabela: str) -> None:
        out = []
        for arq in sorted((self.raiz / tabela).glob("mes=*/parte.parquet")):
            m = pq.ParquetFile(arq).metadata
            grupos = []
            for g in range(m.num_row_groups):
                rg = m.row_group(g)
                cols, tmin, tmax = {}, None, None
                for c in range(rg.num_columns):
                    col = rg.column(c)
                    cols[col.path_in_schema] = col.total_compressed_size
                    if col.path_in_schema == "t_disp":
                        tmin, tmax = int(col.statistics.min), int(col.statistics.max)
                grupos.append((tmin, tmax, cols))
            out.append((arq, arq.parent.name.split("=", 1)[1], grupos))
        self._meta[tabela] = out
        self._limites = getattr(self, "_limites", {})
        self._limites[tabela] = (np.array([min(g[0] for g in gs) for _, _, gs in out], dtype=np.int64),
                                 np.array([max(g[1] for g in gs) for _, _, gs in out], dtype=np.int64))

    def bytes_tabela(self, tabela: str) -> int:
        return sum(sum(c.values()) for _, _, gs in self._meta[tabela] for _, _, c in gs)

    # ---------- cofre ----------
    def destrancar(self, hash_preregistro: str, agente: str) -> None:
        if agente != "agent-f3-lifecycle":
            raise Recusa(f"{agente} não pode abrir o cofre")
        if not re.fullmatch(r"[0-9a-f]{64}", hash_preregistro or ""):
            raise Recusa("cofre só abre com o hash do pré-registro")
        if self.destrancado is not None:
            raise Recusa("o cofre abre uma vez só")
        self.destrancado = hash_preregistro

    # ---------- leitura ----------
    def ler(self, tabela: str, colunas: list[str], agente: str, desde: int, ate: int) -> pd.DataFrame:
        if not colunas or "*" in colunas:
            raise Recusa("projeção explícita obrigatória: SELECT * é recusado")
        if ate > self.relogio.agora:
            raise Recusa(f"{agente} pediu até {ate}, depois do relógio {self.relogio.agora}: é ler o futuro")
        if ate >= self.corte and self.destrancado is None:
            raise Recusa(f"{agente} pediu dados do cofre fora do tempo antes do pré-registro")
        cols = list(dict.fromkeys([*colunas, "t_disp"]))
        arqs, estimado, parcial = [], 0, False
        fmin, fmax = self._limites[tabela]
        a0 = int(np.searchsorted(fmax, desde, side="right"))      # primeiro arquivo com max > desde
        a1 = int(np.searchsorted(fmin, ate, side="right"))         # arquivos com min <= ate
        for arq, mes, grupos in self._meta[tabela][a0:a1]:
            toca = [(tmin, tmax, c) for tmin, tmax, c in grupos if tmax > desde and tmin <= ate]
            if toca:
                arqs.append(arq)
                estimado += sum(sum(v for k, v in c.items() if k in cols) for _, _, c in toca)
                parcial |= any(tmin <= desde or tmax > ate for tmin, tmax, _ in grupos)
        if self.gasto.get(agente, 0) + estimado > self.orcamento:
            raise Recusa(f"{agente} estouraria o orçamento de varredura ({self.orcamento} bytes)")
        if not arqs:
            vazio = pd.DataFrame({c: [] for c in cols})
            self._registrar(agente, tabela, colunas, desde, ate, 0, 0, [], None)
            return vazio
        sel = ", ".join(f'"{c}"' for c in cols)
        lista = ", ".join(f"'{a.as_posix()}'" for a in arqs)
        sql = f"SELECT {sel} FROM read_parquet([{lista}]) WHERE t_disp > {int(desde)} AND t_disp <= {int(ate)}"
        # A forma da consulta decide a projeção: o EXPLAIN roda na primeira leitura de cada forma e, para o
        # filtro, na primeira leitura parcial (quando as estatísticas provam que o arquivo inteiro cabe na
        # janela, o DuckDB elimina o filtro, corretamente).
        chave = (tabela, tuple(cols), parcial)
        if chave not in self._explain_ok:
            plano = self.con.execute("EXPLAIN " + sql).fetchall()[0][1]
            proj = self._projecao(plano)
            filtra = ("Filters:" in plano and "t_disp" in plano.split("Filters:", 1)[1]) or not parcial
            if not proj or not set(proj) <= set(cols) or not filtra:
                raise Recusa(f"EXPLAIN mostra projeção {proj} (pedida {cols}), filtro em t_disp: {filtra}")
            self._explain_ok[chave] = proj
        df = self.con.execute(sql).df()
        self.gasto[agente] = self.gasto.get(agente, 0) + estimado
        tmax = int(df["t_disp"].max()) if len(df) else None
        self._registrar(agente, tabela, colunas, desde, ate, estimado, len(df), self._explain_ok[chave], tmax)
        return df

    def _registrar(self, agente, tabela, colunas, desde, ate, estimado, linhas, proj, tmax):
        self.registro.append({"agente": agente, "tabela": tabela, "colunas": list(colunas), "desde": int(desde),
                              "ate": int(ate), "relogio": self.relogio.agora, "bytes_varridos": int(estimado),
                              "linhas": int(linhas), "explain_projecao": proj, "t_disp_max_lido": tmax,
                              "toca_cofre": bool(ate >= self.corte)})

    @staticmethod
    def _projecao(plano: str) -> list[str]:
        linhas = [l.strip("│ ").strip() for l in plano.splitlines()]
        out, dentro = [], False
        for l in linhas:
            if l.startswith("Projections:"):
                resto = l.split(":", 1)[1].strip()
                if resto:
                    return [resto.strip('"')]
                dentro = True
                continue
            if dentro:
                if not l or l.endswith(":") or l.startswith("~") or l.startswith("Scanning") or set(l) <= set("─┌┐└┘┬┴"):
                    if out:
                        break
                    continue
                out.append(l.strip('"'))
        return out

    def resumo_varredura(self) -> dict:
        por = {}
        for r in self.registro:
            d = por.setdefault(r["tabela"], {"leituras": 0, "bytes_varridos": 0})
            d["leituras"] += 1
            d["bytes_varridos"] += r["bytes_varridos"]
        for t, d in por.items():
            d["bytes_tabela"] = self.bytes_tabela(t)
            d["varreduras_equivalentes"] = d["bytes_varridos"] / max(d["bytes_tabela"], 1)
        vaz = [r for r in self.registro if r["t_disp_max_lido"] is not None and (r["t_disp_max_lido"] > r["ate"] or r["t_disp_max_lido"] > r["relogio"])]
        return {"por_tabela": por, "leituras": len(self.registro), "leituras_com_futuro": len(vaz),
                "gasto_por_agente": dict(self.gasto), "orcamento_por_agente": self.orcamento}
