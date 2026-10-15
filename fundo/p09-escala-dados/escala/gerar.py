"""Gera domínios automáticos (pacote.yaml + preparo.py) a partir do catálogo do Rdatasets pela regra R1 de DECLARACAO.md.

Uso:  python3 -I escala/gerar.py <pasta_dados> <pasta_saida_dominios>
Lê <pasta_dados>/rdatasets/{catalogo.csv,csv/...}; grava <saida>/<nome>/{pacote.yaml,preparo.py} e
<saida>/_selecao.json (cada conjunto do catálogo com o passo da regra em que parou, ou o domínio gerado).

A regra só olha nomes, tipos, contagens de níveis, faltantes e monotonia; não olha a relação entre colunas,
exceto o descarte de candidatos quase idênticos ao alvo (|r| ≥ 0,999), que é checagem de identidade.
"""

from __future__ import annotations

import hashlib
import json
import re
import sys
from pathlib import Path

import numpy as np
import pandas as pd
import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))
from baixar import filtro_metadados  # noqa: E402

PROIBIDO = re.compile(r"(^|[_.\s])(rownames|id|ids|code|codes|index|idx|row|obs|name|names|x)([_.\s]|$)", re.I)
MAX_CAND = 6
MIN_NIVEL = 30
MIN_LINHAS = 300
MAX_FALTA = 0.20


def _nome_seguro(s: str) -> str:
    return re.sub(r"[^0-9a-z_]", "_", s.lower())


def selecionar(df: pd.DataFrame) -> tuple[dict | None, str]:
    """Aplica R1 (passos 2 a 7). Devolve (escolha, motivo)."""
    cols = [c for c in df.columns if not PROIBIDO.search(str(c)) and str(c) != "rownames"]
    if len(cols) < 4:
        return None, "R1.2 menos de 4 colunas com nome permitido"
    # passo 3: ambiente = primeira coluna com 3 a 8 níveis, cada um com >= 30 linhas, e não decimal
    amb = None
    for c in cols:
        s = df[c].dropna()
        if pd.api.types.is_float_dtype(s) and not np.all(np.mod(s, 1) == 0):
            continue
        vc = s.astype(str).value_counts()
        if 3 <= len(vc) <= 8 and vc.min() >= MIN_NIVEL:
            amb = c
            break
    if amb is None:
        return None, "R1.3 nenhuma coluna de ambiente (3 a 8 níveis com >= 30 linhas)"
    num = [c for c in cols if c != amb and pd.api.types.is_numeric_dtype(df[c]) and not pd.api.types.is_bool_dtype(df[c])]
    num = [c for c in num if df[c].isna().mean() <= MAX_FALTA and df[c].std(skipna=True) > 0]
    # passo 4: alvo = primeira numérica com >= 10 valores distintos e não monótona na ordem das linhas
    alvo = None
    for c in num:
        s = df[c].dropna()
        if s.nunique() >= 10 and not (s.is_monotonic_increasing or s.is_monotonic_decreasing):
            alvo = c
            break
    if alvo is None:
        return None, "R1.4 nenhum alvo numérico (>= 10 valores, não monótono)"
    # passo 5: candidatos = numéricas seguintes na ordem do arquivo, até 6, sem cópias do alvo
    cand = []
    for c in num:
        if c == alvo:
            continue
        ok = df[[c, alvo]].dropna()
        if ok[c].nunique() < 2:
            continue
        r = np.corrcoef(ok[c].astype(float), ok[alvo].astype(float))[0, 1] if len(ok) > 2 else 1.0
        if not np.isfinite(r) or abs(r) >= 0.999:
            continue
        cand.append(c)
        if len(cand) == MAX_CAND:
            break
    if len(cand) < 2:
        return None, "R1.5 menos de 2 candidatos numéricos"
    # passo 6: casos completos; níveis com < 30 linhas saem; >= 3 níveis e >= 300 linhas
    d = df[[amb, alvo, *cand]].dropna()
    vc = d[amb].astype(str).value_counts()
    niveis = sorted(vc[vc >= MIN_NIVEL].index)
    d = d[d[amb].astype(str).isin(niveis)]
    if len(niveis) < 3 or len(d) < MIN_LINHAS:
        return None, "R1.6 depois dos casos completos sobram < 3 ambientes ou < 300 linhas"
    for c in cand:  # candidato constante dentro do treino quebra a padronização
        if d[c].std() == 0:
            return None, "R1.6 candidato constante depois dos casos completos"
    # passo 7: cofre = último nível na ordem alfabética
    cofre = niveis[-1]
    impressao = hashlib.sha256(np.sort(d[alvo].to_numpy(float).round(8)).tobytes()).hexdigest()[:16]
    return {"ambiente": amb, "alvo": alvo, "candidatos": cand, "chave": cand[0], "niveis": niveis, "cofre": cofre,
            "linhas": int(len(d)), "impressao": impressao}, "ok"


PREPARO = '''"""Gerado por escala/gerar.py (regra R1 de DECLARACAO.md). Rdatasets {pacote}::{item}.

y e a chave são padronizados com média e dp do TREINO (ambientes fora do cofre); os outros candidatos ficam na
escala original (o Desenho do funil os padroniza). Unidade = linha: nenhum agrupamento é declarado.
"""

from pathlib import Path

import pandas as pd

ARQ = {arq!r}
AMB, ALVO, CAND, COFRE, NIVEIS = {amb!r}, {alvo!r}, {cand!r}, {cofre!r}, {niveis!r}
NOVOS = {novos!r}


def preparar(dados: Path) -> pd.DataFrame:
    d = pd.read_csv(Path(dados) / ARQ, low_memory=False)
    d = d[[AMB, ALVO, *CAND]].dropna()
    d = d[d[AMB].astype(str).isin(NIVEIS)].reset_index(drop=True)
    tr = d[AMB].astype(str) != COFRE
    out = pd.DataFrame({{"ambiente": d[AMB].astype(str).to_numpy(), "unidade": range(len(d))}})
    for velho, novo in zip([ALVO, *CAND], ["y", *NOVOS]):
        x = d[velho].astype(float)
        if novo in ("y", NOVOS[0]):
            x = (x - x[tr].mean()) / x[tr].std()
        out[novo] = x.to_numpy()
    return out
'''


def pacote(pac: str, item: str, titulo: str, e: dict) -> tuple[dict, list]:
    nome = _nome_seguro(f"auto_{pac}_{item}")
    novos = []
    for c in e["candidatos"]:
        n = "x_" + _nome_seguro(str(c))
        while n in novos or n == "x_y":
            n += "_"
        novos.append(n)
    chave = novos[0]
    q = {"q05": -1.645, "q25": -0.674, "q50": 0.0, "q75": 0.674, "q95": 1.645}
    gloss = {"regra_R1": "seleção automática de ambiente, alvo e candidatos por nome, tipo e contagem (DECLARACAO.md)",
             "ambiente": f"coluna {e['ambiente']} do conjunto original ({len(e['niveis'])} níveis)",
             "cofre": f"nível {e['cofre']} da coluna {e['ambiente']}, nunca visto antes do pré-registro",
             "alvo": f"coluna {e['alvo']}, padronizada pelo treino",
             "chave": f"coluna {e['candidatos'][0]}, padronizada pelo treino",
             "priori_vaga": "N(0, 1) na inclinação padronizada, por regra; não é elicitação",
             "decisao_sintetica": "adotar se a inclinação padronizada for positiva; não representa decisão real",
             "unidade_linha": "cada linha é tratada como independente; nenhum agrupamento declarado",
             "dag_estrela": "todos os candidatos apontam para y; nenhuma aresta entre candidatos foi declarada",
             "sem_semantica": "nenhum humano leu a documentação deste conjunto para montar o pacote"}
    meta = {
        "nome": nome, "area": f"Automático (Rdatasets: {pac})",
        "titulo": f"{titulo} [{pac}::{item}]",
        "pergunta": f"A lei de {e['alvo']} dado {', '.join(map(str, e['candidatos']))} é a mesma entre os níveis de {e['ambiente']}?",
        "alegacao": f'"{e["alvo"]} responde a {e["candidatos"][0]} do mesmo jeito em qualquer nível de {e["ambiente"]}."',
        "glossario": gloss,
        "fontes": [f"Rdatasets {pac}::{item}, commit 1dcc2bf5f955cc1224a3e1307256e1fe86b68dae"],
        "arquivos": [f"rdatasets/csv/{pac}/{item}.csv"],
        "familia": "gaussiana", "desfecho": f"{e['alvo']} padronizado", "chave": chave,
        "candidatos": {n: {"tipo": "num", "descricao": f"coluna {c}"} for n, c in zip(novos, e["candidatos"])},
        "icp_candidatos": novos, "ajuste": novos[1:],
        "dag": [[n, "y"] for n in novos],
        "ambientes": {"descricao": f"níveis de {e['ambiente']} escolhidos pela regra R1; cofre = último em ordem alfabética",
                      "cofre": [e["cofre"]]},
        "unidade": "a linha (regra R1; nenhum agrupamento conhecido)",
        "cabecalho": {"processo_gerador": f"desconhecido: conjunto {pac}::{item} ({titulo}), pacote gerado sem leitura humana",
                      "nao_linearidade": "linear por regra", "unidade_e_margem": f"{e['linhas']} linhas tratadas como independentes",
                      "fisica_do_ruido": "não examinada", "escada_causal": "degrau desconhecido (sem saber se há intervenção)"},
        "shelf": {"quantidade": "inclinação de y padronizado por dp da chave", "fonte": "priori vaga por regra (R1), não elicitada",
                  "painel": "nenhum", "quartis": q},
        "decisao": {"acao": "agir como se a inclinação fosse positiva", "beneficio_por_unidade": 1.0, "custo": 0.0,
                    "custo_pesquisa": 0.01, "nota": "decisão sintética da regra R1"},
        "termo_escopo": {"vale_para": f"o conjunto {pac}::{item}", "nao_vale_para": "qualquer outra população"},
        "tentativas_de_especificacao": 1, "tem_titulares": False, "quase_ids": [], "gerado_por": "escala/gerar.py (R1)",
    }
    return meta, novos


def main(dados: Path, saida: Path):
    cat = filtro_metadados(pd.read_csv(dados / "rdatasets" / "catalogo.csv"))
    saida.mkdir(parents=True, exist_ok=True)
    sel, vistos = [], {}
    for r in cat.itertuples():
        arq = dados / "rdatasets" / "csv" / r.Package / f"{r.Item}.csv"
        reg = {"pacote": r.Package, "item": r.Item, "linhas_catalogo": int(r.Rows)}
        if not arq.exists():
            sel.append({**reg, "motivo": "R1.1 CSV não baixado"})
            continue
        try:
            df = pd.read_csv(arq, low_memory=False)
            e, motivo = selecionar(df)
        except Exception as ex:  # noqa: BLE001
            e, motivo = None, f"R1.2 CSV ilegível: {type(ex).__name__}"
        if e is None:
            sel.append({**reg, "motivo": motivo})
            continue
        if e["impressao"] in vistos:
            sel.append({**reg, "motivo": f"R1.8 duplicata de {vistos[e['impressao']]}"})
            continue
        meta, novos = pacote(r.Package, r.Item, str(r.Title), e)
        vistos[e["impressao"]] = meta["nome"]
        pasta = saida / meta["nome"]
        pasta.mkdir(exist_ok=True)
        (pasta / "pacote.yaml").write_text(yaml.safe_dump(meta, allow_unicode=True, sort_keys=False))
        (pasta / "preparo.py").write_text(PREPARO.format(pacote=r.Package, item=r.Item, arq=meta["arquivos"][0], amb=e["ambiente"],
                                                         alvo=e["alvo"], cand=e["candidatos"], cofre=e["cofre"],
                                                         niveis=e["niveis"], novos=novos))
        sel.append({**reg, "motivo": "ok", "dominio": meta["nome"], **{k: e[k] for k in ("ambiente", "alvo", "candidatos", "cofre", "linhas")}})
    (saida / "_selecao.json").write_text(json.dumps(sel, indent=1, ensure_ascii=False, default=str))
    cont = pd.Series([s["motivo"].split(" ")[0] if s["motivo"] != "ok" else "ok" for s in sel]).value_counts()
    print(cont.to_string())


if __name__ == "__main__":
    main(Path(sys.argv[1]), Path(sys.argv[2]))
