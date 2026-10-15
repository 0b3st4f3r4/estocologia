"""Pacote de Domínio Padronizado (PLAN.md §3): o que o agent-f7-domain entrega para plugar uma área nova.

Um domínio é uma pasta `dominios/<nome>/` com dois arquivos:
- `pacote.yaml`: pergunta substantiva, alegação, glossário de 10 termos, fontes, unidade independente,
  ambientes (e quais ficam no cofre), DAG declarado, conjunto de ajuste, elicitação SHELF, decisão e escopo;
- `preparo.py`: `preparar(dados: Path) -> DataFrame` com as colunas `y`, `ambiente`, `unidade` e os
  candidatos declarados. Nada de modelagem aqui: só tradução de nomes, tipos e recortes.

O carregador valida o pacote contra este contrato antes de qualquer leitura de dados.
"""

from __future__ import annotations

import importlib.util
from dataclasses import dataclass, field
from pathlib import Path

import numpy as np
import pandas as pd
import yaml
from scipy import optimize, stats

CABECALHO = ("processo_gerador", "nao_linearidade", "unidade_e_margem", "fisica_do_ruido", "escada_causal")
OBRIGATORIOS = ("nome", "area", "titulo", "pergunta", "alegacao", "glossario", "fontes", "arquivos", "familia", "desfecho",
                "chave", "candidatos", "icp_candidatos", "ajuste", "dag", "ambientes", "unidade", "cabecalho", "shelf",
                "decisao", "termo_escopo")


class PacoteInvalido(ValueError):
    pass


@dataclass
class Pacote:
    raiz: Path
    meta: dict
    preparar: object = field(repr=False)

    def __getattr__(self, k):
        try:
            return self.meta[k]
        except KeyError as e:
            raise AttributeError(k) from e


def validar(meta: dict) -> list[str]:
    erros = [f"falta {k}" for k in OBRIGATORIOS if k not in meta]
    if erros:
        return erros
    if len(meta["glossario"]) < 10:
        erros.append(f"glossário com {len(meta['glossario'])} termos (mínimo 10)")
    if meta["familia"] not in ("gaussiana", "binomial"):
        erros.append(f"família inválida: {meta['familia']}")
    cand = meta["candidatos"]
    if meta["chave"] not in cand:
        erros.append("a chave precisa ser um candidato")
    for v in list(meta["icp_candidatos"]) + list(meta["ajuste"]):
        if v not in cand:
            erros.append(f"{v} não está entre os candidatos")
    if len(meta["icp_candidatos"]) > 10:
        erros.append("ICP com mais de 10 candidatos (2^10 subconjuntos é o teto)")
    for k in CABECALHO:
        if not str(meta["cabecalho"].get(k, "")).strip():
            erros.append(f"cabeçalho sem {k}")
    sh = meta["shelf"]
    for q in ("q05", "q25", "q50", "q75", "q95"):
        if q not in sh.get("quartis", {}):
            erros.append(f"SHELF sem {q}")
    if not sh.get("fonte"):
        erros.append("SHELF sem fonte")
    if meta["familia"] == "binomial" and "taxa_base" not in sh:
        erros.append("SHELF binomial sem taxa_base")
    for k in ("beneficio_por_unidade", "custo", "custo_pesquisa", "acao"):
        if k not in meta["decisao"]:
            erros.append(f"decisão sem {k}")
    for k in ("vale_para", "nao_vale_para"):
        if not meta["termo_escopo"].get(k):
            erros.append(f"termo de escopo sem {k}")
    if not meta["ambientes"].get("cofre"):
        erros.append("nenhum ambiente no cofre")
    nos_dag = {n for a in meta["dag"] for n in a}
    for v in ("y", meta["chave"]):
        if v not in nos_dag:
            erros.append(f"DAG não tem o nó {v}")
    return erros


def carregar(pasta: Path) -> Pacote:
    pasta = Path(pasta)
    meta = yaml.safe_load((pasta / "pacote.yaml").read_text())
    erros = validar(meta)
    if erros:
        raise PacoteInvalido(f"{pasta.name}: {erros}")
    spec = importlib.util.spec_from_file_location(f"preparo_{meta['nome']}", pasta / "preparo.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return Pacote(pasta, meta, mod.preparar)


def checar_tabela(p: Pacote, df: pd.DataFrame) -> list[str]:
    erros = [f"coluna {c} ausente" for c in ("y", "ambiente", "unidade", *p.candidatos) if c not in df.columns]
    if erros:
        return erros
    if df["y"].isna().any():
        erros.append("y com faltantes")
    if p.familia == "binomial" and not set(np.unique(df["y"])) <= {0, 1}:
        erros.append("y binomial fora de {0,1}")
    cofre = set(map(str, p.ambientes["cofre"]))
    amb = set(map(str, df["ambiente"].unique()))
    if not cofre <= amb:
        erros.append(f"ambientes do cofre ausentes: {sorted(cofre - amb)}")
    if len(amb - cofre) < 2:
        erros.append("menos de 2 ambientes de treino: ICP impossível")
    return erros


# ---------------- SHELF: da elicitação à priori ----------------
def ajustar_normal(q: dict) -> dict:
    """Normal ajustada aos quartis por mínimos quadrados nos quantis (como o SHELF faz); q05/q95 conferem a
    calibração: se o 90% elicitado for muito mais estreito que o implicado pelos quartis, há excesso de confiança."""
    alvo = np.array([q["q25"], q["q50"], q["q75"]], float)
    z = stats.norm.ppf([0.25, 0.5, 0.75])
    r = optimize.minimize(lambda t: np.sum((t[0] + np.exp(t[1]) * z - alvo) ** 2),
                          x0=[alvo[1], np.log(max((alvo[2] - alvo[0]) / 1.35, 1e-9))], method="Nelder-Mead",
                          options={"xatol": 1e-12, "fatol": 1e-16, "maxiter": 5000})
    m, s = r.x[0], float(np.exp(r.x[1]))
    impl = (m + s * stats.norm.ppf(0.05), m + s * stats.norm.ppf(0.95))
    larg_elic = q["q95"] - q["q05"]
    return {"media": float(m), "dp": s, "erro_max_quartis": float(np.max(np.abs(m + s * z - alvo))),
            "q05_q95_implicados": [float(impl[0]), float(impl[1])],
            "razao_largura_90_elicitada_sobre_implicada": float(larg_elic / (impl[1] - impl[0]))}


def priori_da_chave(p: Pacote) -> dict:
    """Priori normal do coeficiente da chave. Binomial: os quartis vêm como razão de chances e viram log."""
    q = dict(p.shelf["quartis"])
    if p.familia == "binomial" and p.shelf.get("escala", "razao_de_chances") == "razao_de_chances":
        q = {k: float(np.log(v)) for k, v in q.items()}
    return ajustar_normal(q)


def theta(p: Pacote, beta_chave: np.ndarray, taxa: float | None = None) -> np.ndarray:
    """Quantidade da decisão: o coeficiente (gaussiana) ou o efeito marginal médio aproximado β·p̄(1−p̄)."""
    if p.familia == "gaussiana":
        return beta_chave
    pb = p.shelf["taxa_base"] if taxa is None else taxa
    return beta_chave * pb * (1 - pb)


def utilidade(p: Pacote, acao: str, th: np.ndarray) -> np.ndarray:
    d = p.decisao
    if acao == "adotar":
        return d["beneficio_por_unidade"] * th - d["custo"]
    return np.zeros_like(th)
