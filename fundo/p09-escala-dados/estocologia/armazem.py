"""Armazém colunar do H2 (agent-f2-data): Parquet particionado por ambiente, consultado via DuckDB.

Regra de escala do PLAN (§5.1): "um modelo que exige uma varredura completa por previsão não escala".
Toda leitura de agente:
1. nomeia as colunas (SELECT * é recusado) e, quando pode, os ambientes (poda de partição);
2. tem os bytes estimados pelos metadados do Parquet (só as colunas e partições pedidas) e cobrados de
   um orçamento por agente;
3. passa pelo EXPLAIN do DuckDB, que precisa mostrar a projeção exata e a poda de arquivos;
4. fica no registro de varredura, com bytes varridos contra bytes da tabela inteira.

Os ambientes do cofre ficam trancados até o agent-f3-lifecycle destrancá-los com o hash do pré-registro.
"""

from __future__ import annotations

import re
from pathlib import Path

import duckdb
import pandas as pd
import pyarrow as pa
import pyarrow.parquet as pq


class Recusa(PermissionError):
    pass


def _seguro(v: str) -> str:
    s = re.sub(r"[^0-9A-Za-z_.-]", "_", str(v))
    return s or "_"


class Armazem:
    def __init__(self, raiz: Path, orcamento_por_agente: int = 400_000_000):
        self.raiz = Path(raiz)
        self.orcamento = orcamento_por_agente
        self.gasto: dict[str, int] = {}
        self.registro: list[dict] = []
        self.cofres: dict[str, set] = {}
        self.destrancado: dict[str, str] = {}
        self.con = duckdb.connect()

    # ---------- ingestão ----------
    def ingerir(self, dominio: str, df: pd.DataFrame, cofre: list | tuple = ()) -> dict:
        """Grava df particionado pela coluna 'ambiente'. Idempotente: regrava a partição inteira."""
        if "ambiente" not in df.columns:
            raise ValueError("df precisa da coluna 'ambiente'")
        base = self.raiz / dominio
        for amb, parte in df.groupby("ambiente", sort=True):
            d = base / f"ambiente={_seguro(amb)}"
            d.mkdir(parents=True, exist_ok=True)
            tmp = d / "parte.parquet.tmp"
            pq.write_table(pa.Table.from_pandas(parte.drop(columns="ambiente"), preserve_index=False), tmp, row_group_size=50_000)
            tmp.replace(d / "parte.parquet")
        self.cofres[dominio] = {_seguro(a) for a in cofre}
        return {"arquivos": len(list(base.glob("*/*.parquet"))), "bytes_tabela": self.bytes_tabela(dominio),
                "linhas": int(len(df)), "ambientes": sorted(map(str, df["ambiente"].unique()))}

    def _arquivos(self, dominio: str, ambientes=None) -> list[Path]:
        todos = sorted((self.raiz / dominio).glob("ambiente=*/parte.parquet"))
        if ambientes is None:
            return todos
        quer = {_seguro(a) for a in ambientes}
        return [a for a in todos if a.parent.name.split("=", 1)[1] in quer]

    def bytes_tabela(self, dominio: str) -> int:
        return sum(self._bytes(a, None) for a in self._arquivos(dominio))

    @staticmethod
    def _bytes(arq: Path, colunas) -> int:
        m = pq.ParquetFile(arq).metadata
        tot = 0
        for g in range(m.num_row_groups):
            rg = m.row_group(g)
            for c in range(rg.num_columns):
                col = rg.column(c)
                if colunas is None or col.path_in_schema in colunas:
                    tot += col.total_compressed_size
        return tot

    # ---------- cofre ----------
    def destrancar(self, dominio: str, hash_preregistro: str, agente: str) -> None:
        if agente != "agent-f3-lifecycle":
            raise Recusa(f"{agente} não pode abrir o cofre de {dominio}")
        if not re.fullmatch(r"[0-9a-f]{64}", hash_preregistro or ""):
            raise Recusa("cofre só abre com o hash do pré-registro")
        self.destrancado[dominio] = hash_preregistro

    # ---------- leitura ----------
    def ler(self, dominio: str, colunas: list[str], agente: str, ambientes=None, excluir=()) -> pd.DataFrame:
        if not colunas or "*" in colunas:
            raise Recusa("projeção explícita obrigatória: SELECT * é recusado")
        arqs = self._arquivos(dominio, ambientes)
        excl = {_seguro(a) for a in excluir}
        arqs = [a for a in arqs if a.parent.name.split("=", 1)[1] not in excl]
        cofre = self.cofres.get(dominio, set())
        toca_cofre = [a for a in arqs if a.parent.name.split("=", 1)[1] in cofre]
        if toca_cofre and dominio not in self.destrancado:
            raise Recusa(f"{agente} tentou ler o cofre de {dominio} antes do pré-registro")
        if not arqs:
            raise Recusa("nenhuma partição sobrou depois da poda")
        fisicas = [c for c in colunas if c != "ambiente"]
        estimado = sum(self._bytes(a, set(fisicas)) for a in arqs)
        if self.gasto.get(agente, 0) + estimado > self.orcamento:
            raise Recusa(f"{agente} estouraria o orçamento de varredura ({self.orcamento} bytes)")
        lista = ", ".join(f"'{a.as_posix()}'" for a in arqs)
        sel = ", ".join(f'"{c}"' for c in colunas)
        sql = f"SELECT {sel} FROM read_parquet([{lista}], hive_partitioning=true)"
        plano = self.con.execute("EXPLAIN " + sql).fetchall()[0][1]
        proj = self._projecao(plano)
        explain_ok = set(proj) <= set(colunas) | {"ambiente"} and len(proj) > 0
        if not explain_ok:
            raise Recusa(f"EXPLAIN mostra projeção {proj}, pedida {colunas}")
        df = self.con.execute(sql).df()
        self.gasto[agente] = self.gasto.get(agente, 0) + estimado
        self.registro.append({"agente": agente, "dominio": dominio, "colunas": list(colunas),
                              "ambientes": sorted(a.parent.name.split("=", 1)[1] for a in arqs),
                              "arquivos": len(arqs), "bytes_varridos": int(estimado),
                              "bytes_tabela": int(self.bytes_tabela(dominio)), "explain_projecao": proj,
                              "toca_cofre": bool(toca_cofre)})
        return df

    @staticmethod
    def _projecao(plano: str) -> list[str]:
        linhas = [l.strip("│ ").strip() for l in plano.splitlines()]
        out, dentro = [], False
        for l in linhas:
            if l.startswith("Projections:"):
                resto = l.split(":", 1)[1].strip()
                if resto:                      # uma coluna só vem na mesma linha
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
