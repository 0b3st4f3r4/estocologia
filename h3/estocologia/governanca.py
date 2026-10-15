"""F5 (privacidade) e F6 (justiça) em operação contínua, sobre o fluxo de prisões.

F5. A operação publica, ao fim de cada ano, a taxa de liberação por cor da pele, e republica o
acumulado de todos os anos até ali. Assim cada pessoa entra em todas as publicações seguintes, e o ε
dela se soma (composição sequencial). Cada publicação usa ε = 0,5 por célula, com ruído de Laplace
nas contagens (sensibilidade 1). O orçamento por pessoa é 2,0.

O disjuntor de privacidade recusa a parte de uma publicação que levaria alguém acima do orçamento.
Na prática, as coortes antigas saem do acumulado e a publicação passa a ser só das coortes que
ainda têm saldo. Os bits perdidos por célula são I(K; K̃) contra H(K) (estat.bits_perdidos_laplace).

F6. Trilema da justiça (Kleinberg, Mullainathan & Raghavan 2016; Chouldechova 2017). Quando as
taxas-base dos grupos diferem e o previsor não é perfeito, três coisas não valem juntas:
- a calibração dentro de cada grupo;
- o equilíbrio da classe positiva (FNR iguais);
- o equilíbrio da classe negativa (FPR iguais).

O F6 mede as três por ano, com a decisão do contrato (marcar para revisão se p < 0,70). O desfecho
de interesse é não ser liberado (y = 0).

Bloco G, inversão da métrica. A banca adversarial do Φ4 pede inverter a métrica escolhida e mostrar quem
passaria a pagar. A métrica do F6 é a taxa de falso positivo do flag — P(marcado | não sofre o desfecho de
interesse) —, e a métrica invertida é a taxa de falso negativo, P(não marcado | sofre o desfecho). O grupo
menos favorecido é o de maior taxa-base do desfecho de interesse, e o pagador é o de maior taxa sob a métrica.
As duas contas saem por limiar de decisão, num grid declarado antes de rodar (LIMIARES_DECLARADOS), porque o
limiar move as duas taxas em sentidos opostos e pode trocar o pagador. Nada aqui entra no caminho oficial:
`f6_trilema` só acrescenta a chave `justica_invertida` quando recebe `por_limiar`.
"""

from __future__ import annotations

from collections import Counter

import numpy as np
import pandas as pd

from .estat import bits_perdidos_laplace, taxas_por_grupo

EPS_POR_PUBLICACAO = 0.5
ORCAMENTO_EPS = 2.0


def k_anonimato(df: pd.DataFrame, quase: list[str]) -> dict:
    c = Counter(map(tuple, df[quase].astype(str).to_numpy()))
    tam = np.array(list(c.values()))
    d2 = df.assign(age=(df["age"] // 5 * 5).astype(int))
    c2 = Counter(map(tuple, d2[quase].astype(str).to_numpy()))
    tam2 = np.array(list(c2.values()))
    return {"k": int(tam.min()), "fracao_unicos": float((tam == 1).sum() / len(df)),
            "k_idade_em_faixas_de_5": int(tam2.min()), "fracao_unicos_faixas": float((tam2 == 1).sum() / len(df))}


def f5_publicacoes(det: pd.DataFrame, y: pd.Series, rng) -> dict:
    """det: detenções com year e colour; y: liberação (0/1) alinhada. Simula as publicações anuais."""
    gasto = Counter()                      # ε gasto por coorte (ano)
    pubs, recusas = [], []
    anos = sorted(det["year"].unique())
    for ano in anos:
        candidatos = [a for a in anos if a <= ano]
        aceitas = [a for a in candidatos if gasto[a] + EPS_POR_PUBLICACAO <= ORCAMENTO_EPS + 1e-12]
        recusadas = [a for a in candidatos if a not in aceitas]
        if recusadas:
            recusas.append({"publicacao": int(ano), "coortes_recusadas": [int(a) for a in recusadas],
                            "motivo": f"orçamento ε = {ORCAMENTO_EPS} por pessoa esgotado"})
        for a in aceitas:
            gasto[a] += EPS_POR_PUBLICACAO
        sel = det["year"].isin(aceitas)
        cel = []
        for cor, g in det[sel].groupby("colour"):
            k, n = int(y[g.index].sum()), len(g)
            k_ruid = k + rng.laplace(0, 1 / EPS_POR_PUBLICACAO)
            n_ruid = n + rng.laplace(0, 1 / EPS_POR_PUBLICACAO)
            bits = bits_perdidos_laplace(min(n, 400), k / n, EPS_POR_PUBLICACAO, rng, m=40_000)
            cel.append({"cor": cor, "n": n, "taxa_verdadeira": k / n, "taxa_publicada": float(k_ruid / n_ruid),
                        "erro_absoluto": float(abs(k_ruid / n_ruid - k / n)), "bits_perdidos": bits["bits_perdidos"],
                        "H_bits": bits["H_bits"]})
        pubs.append({"publicacao": int(ano), "coortes": [int(a) for a in aceitas], "celulas": cel})
    return {"eps_por_publicacao": EPS_POR_PUBLICACAO, "orcamento_eps": ORCAMENTO_EPS,
            "eps_gasto_por_coorte": {int(k): v for k, v in gasto.items()},
            "orcamento_respeitado": all(v <= ORCAMENTO_EPS + 1e-12 for v in gasto.values()),
            "publicacoes": pubs, "recusas_do_disjuntor_de_privacidade": recusas}


def f6_trilema(diario: pd.DataFrame, limiar: float = 0.70, tol: float = 0.05, por_limiar=None) -> dict:
    """diario: previsões oficiais resolvidas de prisões, com 'grupo', 'ano', 'p' e 'y'.

    por_limiar: com None (o padrão) a saída é exatamente a de sempre, e é o que h3/portoes.py e os
    avaliar_h3c/h3d leem. Com uma sequência de limiares, cada ano ganha a chave 'justica_invertida'
    com a taxa de falso negativo do grupo menos favorecido e o pagador sob a métrica e sob a invertida.
    """
    por_limiar = [float(x) for x in por_limiar] if por_limiar is not None else None
    out = {}
    for ano, g in diario.groupby("ano"):
        g = g[g["grupo"].isin(["Black", "White"])]
        y0 = (g["y"] == 0).to_numpy()                     # desfecho de interesse: não liberado
        flag = (g["p"] < limiar).to_numpy()
        t = taxas_por_grupo(y0, flag, g["grupo"].to_numpy())
        cal = {grp: {"p_nao_liberacao_media": float(1 - gg["p"].mean()), "taxa_nao_liberacao": float(1 - gg["y"].mean())}
               for grp, gg in g.groupby("grupo")}
        dif_cal = max(abs(v["p_nao_liberacao_media"] - v["taxa_nao_liberacao"]) for v in cal.values())
        fpr = {k: v["fpr"][0] for k, v in t.items()}
        fnr = {k: v["fnr"][0] for k, v in t.items()}
        base = {k: v["taxa_base"][0] for k, v in t.items()}
        crit = {"calibracao_dentro_dos_grupos": dif_cal <= tol,
                "equilibrio_classe_negativa_FPR": abs(fpr["Black"] - fpr["White"]) <= tol,
                "equilibrio_classe_positiva_FNR": abs(fnr["Black"] - fnr["White"]) <= tol}
        out[int(ano)] = {"n": int(len(g)), "taxa_base_nao_liberacao": base, "calibracao": cal, "fpr": fpr, "fnr": fnr,
                         "criterios_satisfeitos": crit, "quantos_valem": int(sum(crit.values())),
                         "taxas_base_diferem": abs(base["Black"] - base["White"]) > tol,
                         "detalhe_wilson": t}
        if por_limiar is not None:
            out[int(ano)]["justica_invertida"] = justica_invertida_por_limiar(g, por_limiar, limiar_declarado=limiar)
    return out


# ---------- Bloco G: inversão da métrica (a pergunta do Φ4) ----------

LIMIARES_DECLARADOS = [round(0.05 * i, 2) for i in range(1, 20)]   # 0,05 a 0,95: declarado antes de rodar

METRICA_ESCOLHIDA = "taxa de falso positivo do flag: P(marcado | não sofre o desfecho de interesse)"
METRICA_INVERTIDA = "taxa de falso negativo do flag: P(não marcado | sofre o desfecho de interesse)"
REGRA_DO_GRUPO_MENOS_FAVORECIDO = ("maior taxa-base do desfecho de interesse no recorte; empate se os intervalos "
                                   "de Wilson de 95% das duas taxas-base se sobrepõem")
REGRA_DO_PAGADOR = "maior taxa sob a métrica; empate se os intervalos de Wilson de 95% se sobrepõem"
# v2 (declaração 2): o texto é o mesmo que já está gravado como declaração 2, e agora vale também para
# descrever o que os resultados fizeram — antes o bloco regras dos resultados repetia a regra da v1.
REGRA_DO_GRUPO_MENOS_FAVORECIDO_2 = ("maior taxa-base do desfecho de interesse no recorte, pela estimativa pontual; "
                                     "o empate dos intervalos de Wilson de 95% sai no campo taxa_base_empata_95")
CONVENCAO_DE_SINAL = "y = 1 é o desfecho NÃO adverso; o desfecho de interesse é y = 0; o flag é p < limiar"

FLUXOS_JUSTICA = {
    "prisoes": {"limiar": 0.70, "fatia": "diário resolvido inteiro, desenvolvimento e cofre (como em portoes.g6)",
                "transformacao": "p = probabilidade oficial do rótulo (a liberação); y já é a liberação",
                "pergunta": "P(liberação com intimação) do detido; o desfecho de interesse é não ser liberado"},
    "prisoes20": {"limiar": 0.70, "fatia": "diário resolvido inteiro (como em avaliar_h3c)",
                  "transformacao": "p = 1 − oficial (a não prisão); y = 1 − registro; ano × 10 + semestre",
                  "pergunta": "P(registro na delegacia em 24 h); o desfecho de interesse é o registro"},
    "prisoes_ri": {"limiar": 0.90, "fatia": "só o cofre, t_emissao ≥ corte (como em avaliar_h3d)",
                   "transformacao": "p = 1 − oficial (a não prisão); y = 1 − prisão",
                   "pergunta": "P(prisão do motorista na parada); o desfecho de interesse é a prisão"},
}


def _sobrepoem(a: list, b: list) -> bool:
    """Dois intervalos de Wilson de 95% [p, lo, hi] se sobrepõem."""
    return bool(a[1] <= b[2] and b[1] <= a[2])


def _maior_ou_empate(t: dict, chave: str, gs: list) -> str:
    x, y = gs
    if not np.isfinite(t[x][chave][0]) or not np.isfinite(t[y][chave][0]):
        return "indeterminado"
    if _sobrepoem(t[x][chave], t[y][chave]):
        return "empate"
    return x if t[x][chave][0] > t[y][chave][0] else y


def _maior_por_ponto(t: dict, chave: str, gs: list) -> str:
    """Nomeia sempre um grupo, pela estimativa pontual (o empate vai ao lado, em campo próprio)."""
    x, y = gs
    if not np.isfinite(t[x][chave][0]):
        return y
    if not np.isfinite(t[y][chave][0]):
        return x
    return x if t[x][chave][0] >= t[y][chave][0] else y


def _fin(x) -> float | None:
    """Grava null no lugar de NaN: uma razão 0/0 é indefinida, não um número (JSON válido)."""
    return float(x) if np.isfinite(x) else None


def _fins(vals: list) -> list:
    return [_fin(v) for v in vals]


def _sha_json(obj) -> str:
    import hashlib
    import json
    return hashlib.sha256(json.dumps(obj, sort_keys=True, ensure_ascii=False).encode()).hexdigest()


def justica_invertida_por_limiar(g: pd.DataFrame, limiares, limiar_declarado: float | None = None) -> dict:
    """Quem paga pelo erro com a métrica do F6 e com a métrica invertida, limiar por limiar.

    g: previsões de um recorte com 'p', 'y' e 'grupo' (Black/White). O sinal é o do f6_trilema:
    y = 1 é o desfecho não adverso e o flag é p < limiar. A métrica escolhida é o FPR do flag e a
    invertida é o FNR do flag, de modo que a inversão troca qual erro os grupos pagam. O grupo menos
    favorecido sai da maior taxa-base do desfecho de interesse, e não de um rótulo fixo.
    """
    g = g[g["grupo"].isin(["Black", "White"])]
    gs = sorted(g["grupo"].astype(str).unique().tolist())
    if len(gs) < 2:
        return {"n": int(len(g)), "grupos": gs, "erro": "menos de dois grupos no recorte"}
    y0 = (g["y"] == 0).to_numpy()
    p = g["p"].to_numpy(float)
    grp = g["grupo"].astype(str).to_numpy()
    base = taxas_por_grupo(y0, np.zeros(len(y0), bool), grp)     # a taxa-base não depende do limiar
    menos = _maior_por_ponto(base, "taxa_base", gs)              # sempre nomeia um grupo (declaração 2)
    empata_base = _sobrepoem(base[gs[0]]["taxa_base"], base[gs[1]]["taxa_base"])
    linhas = []
    for lim in limiares:
        t = taxas_por_grupo(y0, p < lim, grp)
        paga_esc, paga_inv = _maior_ou_empate(t, "fpr", gs), _maior_ou_empate(t, "fnr", gs)
        linhas.append({"limiar": float(lim),
                       "fnr_grupo_menos_favorecido": _fin(t[menos]["fnr"][0]),
                       "fnr_por_grupo": {x: _fin(t[x]["fnr"][0]) for x in gs},
                       "fpr_por_grupo": {x: _fin(t[x]["fpr"][0]) for x in gs},
                       "paga_metrica_escolhida": paga_esc, "paga_metrica_invertida": paga_inv,
                       "troca_de_pagador": bool(paga_esc != paga_inv and "empate" not in (paga_esc, paga_inv))})
    lref = float(limiar_declarado) if limiar_declarado is not None else float(limiares[0])
    t = taxas_por_grupo(y0, p < lref, grp)
    pag_esc, pag_inv = _maior_ou_empate(t, "fpr", gs), _maior_ou_empate(t, "fnr", gs)
    outro = gs[1] if menos == gs[0] else gs[0]
    detalhe = {"limiar": lref, "n": int(len(g)),
               "grupos": {x: {"n": int(t[x]["n"]), "taxa_base": _fins(t[x]["taxa_base"]),
                              "fpr_do_flag": _fins(t[x]["fpr"]), "fnr_do_flag": _fins(t[x]["fnr"])} for x in gs},
               "grupo_menos_favorecido": menos, "taxa_base_empata_95": bool(empata_base),
               "fnr_grupo_menos_favorecido": _fin(t[menos]["fnr"][0]),
               "fnr_outro_grupo": _fin(t[outro]["fnr"][0]),
               "paga_metrica_escolhida": pag_esc, "paga_metrica_invertida": pag_inv,
               "troca_de_pagador": bool(pag_esc != pag_inv and "empate" not in (pag_esc, pag_inv)),
               "pessoas_no_erro_do_flag": {x: {"falso_positivo_do_flag": int(((p < lref) & (y0 == 0) & (grp == x)).sum()),
                                               "falso_negativo_do_flag": int(((p >= lref) & (y0 == 1) & (grp == x)).sum())}
                                           for x in gs}}
    trocas = [l["limiar"] for l in linhas if l["troca_de_pagador"]]
    fnr_mf = [l["fnr_grupo_menos_favorecido"] for l in linhas if l["fnr_grupo_menos_favorecido"] is not None]
    return {"n": int(len(g)), "grupos": gs, "grupo_menos_favorecido": menos,
            "taxa_base_empata_95": bool(empata_base),
            "taxa_base": {x: _fins(base[x]["taxa_base"]) for x in gs},
            "regras": {"convencao": CONVENCAO_DE_SINAL, "metrica_escolhida": METRICA_ESCOLHIDA,
                       "metrica_invertida": METRICA_INVERTIDA,
                       "grupo_menos_favorecido": REGRA_DO_GRUPO_MENOS_FAVORECIDO_2,
                       "grupo_menos_favorecido_na_declaracao_1": REGRA_DO_GRUPO_MENOS_FAVORECIDO,
                       "regra_aplicada": "declaracao_2", "pagador": REGRA_DO_PAGADOR},
            "resumo": {"limiares_em_que_o_pagador_troca": trocas,
                       "fracao_dos_limiares_com_troca": len(trocas) / len(linhas),
                       "pagadores_sob_a_metrica_escolhida": sorted({l["paga_metrica_escolhida"] for l in linhas}),
                       "pagadores_sob_a_metrica_invertida": sorted({l["paga_metrica_invertida"] for l in linhas}),
                       "fnr_do_grupo_menos_favorecido_min": (min(fnr_mf) if fnr_mf else None),
                       "fnr_do_grupo_menos_favorecido_max": (max(fnr_mf) if fnr_mf else None)},
            "por_limiar": linhas, "no_limiar_declarado": detalhe}


def declaracao_justica_invertida(fluxo: str, cfg: dict) -> dict:
    """O texto que fica gravado antes de rodar. Só muda se alguém editar governanca.py."""
    return {"escopo": "Bloco G do plano: auditoria de justiça, sem modelo novo",
            "fluxo": fluxo, "pergunta": cfg["pergunta"], "fatia": cfg["fatia"],
            "transformacao_do_diario": cfg["transformacao"],
            "convencao_de_sinal": CONVENCAO_DE_SINAL, "metrica_escolhida": METRICA_ESCOLHIDA,
            "metrica_invertida": METRICA_INVERTIDA, "grupo_menos_favorecido": REGRA_DO_GRUPO_MENOS_FAVORECIDO,
            "quem_paga": REGRA_DO_PAGADOR, "empate": "intervalos de Wilson de 95% sobrepostos",
            "limiares_declarados": LIMIARES_DECLARADOS, "limiar_declarado_do_fluxo": cfg["limiar"],
            "o_que_nao_muda": ("este código não entra em h3/portoes.py nem em h3/avaliar_h3c.py ou h3/avaliar_h3d.py: "
                               "f6_trilema só devolve a chave nova quando recebe por_limiar, e o padrão é o de antes")}


def declaracao_justica_invertida_2(fluxo: str, cfg: dict) -> dict:
    """Segunda declaração: a regra do grupo menos favorecido nomeia sempre um grupo, pelo ponto.

    A v1 mandava dizer "empate" quando os intervalos de Wilson das duas taxas-base se sobrepunham, e
    com isso Toronto 2001 e 2002 ficavam sem o número que o Bloco G pede (as taxas-base são 21% e 19%,
    e os intervalos se cruzam). A v2 nomeia o grupo pela taxa-base pontual e carrega o empate ao lado,
    no campo taxa_base_empata_95. A mudança veio depois de ver os primeiros números do fluxo prisoes,
    então entra como declaração nova, com hash próprio, e conta no K (= 2), como manda a regra de casa.
    """
    v1 = declaracao_justica_invertida(fluxo, cfg)
    return {**v1,
            "grupo_menos_favorecido": REGRA_DO_GRUPO_MENOS_FAVORECIDO_2,
            "declaracao_anterior_sha256": _sha_json(v1),
            "por_que_da_segunda_declaracao": ("a v1 deixava Toronto 2001 e 2002 sem a taxa de falso negativo do grupo "
                                              "menos favorecido, porque as taxas-base (21% e 19%) têm intervalos de "
                                              "Wilson que se cruzam; a v2 nomeia o grupo pelo ponto e declara o empate"),
            "o_que_nao_muda_na_v2": ("a métrica escolhida, a métrica invertida, a regra do pagador, os limiares "
                                     "declarados e o recorte de cada fluxo são os da v1"),
            "resultados_seguem": "declaracao_2"}


def _recorte_justica(df: pd.DataFrame, fluxo: str, corte: int) -> pd.DataFrame:
    """Deixa o diário no sinal do f6_trilema: p = P(desfecho não adverso), y = 1 no não adverso."""
    from . import calibracao as C
    d = df[df["y"].notna()].copy()
    if fluxo == "prisoes":                       # o rótulo já é a liberação, o desfecho não adverso
        d["p"] = C.oficial_binario(d)
        d["y"] = d["y"].astype(float)
    elif fluxo == "prisoes20":                   # o rótulo é o registro na delegacia
        d["p"] = 1 - C.oficial_binario(d)
        d["y"] = 1 - d["y"].astype(float)
        d["ano"] = d["ano"].astype(int) * 10 + (d["trimestre"].astype(int) > 6) + 1
    elif fluxo == "prisoes_ri":                  # o rótulo é a prisão; só o cofre, como em avaliar_h3d
        d["p"] = 1 - C.oficial_binario(d)
        d["y"] = 1 - d["y"].astype(float)
        d = d[d["t_emissao"] >= corte]
    else:
        raise ValueError(f"fluxo sem recorte declarado: {fluxo}")
    d["ano"] = d["ano"].astype(int)
    return d


def main(argv=None):
    """Grava saida/<fluxo>/justica_invertida.json para os três fluxos de prisões.

    Uso:  python3 -m estocologia.governanca --declarar     # só a declaração, com os hashes
          python3 -m estocologia.governanca                # roda e completa os mesmos arquivos
    Lê ESTOCOLOGIA_SAIDA (padrão h3/saida). Não toca em portoes.py nem nos avaliar_h3*.
    """
    import hashlib
    import json
    import os
    import sys
    import time
    from pathlib import Path

    from .motor import sha_arquivo

    argv = list(sys.argv[1:] if argv is None else argv)
    declarar = "--declarar" in argv
    raiz = Path(__file__).resolve().parents[1]
    saida = Path(os.environ.get("ESTOCOLOGIA_SAIDA", raiz / "saida"))
    h_gov = sha_arquivo(Path(__file__).resolve())
    agora = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    resumo = {}
    for fluxo, cfg in FLUXOS_JUSTICA.items():
        alvo = saida / fluxo / "justica_invertida.json"
        anterior = json.loads(alvo.read_text()) if alvo.exists() else None
        decl = declaracao_justica_invertida(fluxo, cfg)
        h_decl = _sha_json(decl)
        if anterior and "declaracao" in anterior:            # a declaração gravada nunca se reescreve
            if anterior["declaracao"] != decl:
                print(f"AVISO {fluxo}: a declaração gravada difere da do código; a gravada fica", file=sys.stderr)
            decl, h_decl = anterior["declaracao"], anterior["declaracao_sha256"]
        decl2 = declaracao_justica_invertida_2(fluxo, cfg)
        h_decl2 = _sha_json(decl2)
        if anterior and "declaracao_2" in anterior:          # e a segunda também não
            if anterior["declaracao_2"] != decl2:
                print(f"AVISO {fluxo}: a declaração 2 gravada difere da do código; a gravada fica", file=sys.stderr)
            decl2, h_decl2 = anterior["declaracao_2"], anterior["declaracao_2_sha256"]
        doc = {"fluxo": fluxo,
               "nota_de_serializacao": ("estas são as três únicas taxas do H3 gravadas como null, e não como NaN, "
                                        "quando o denominador é 0: o caso é o recorte de 5 paradas de 2012 em "
                                        "prisoes_ri, sem nenhuma prisão. É representação, não critério"),
               "declaracao": decl, "declaracao_sha256": h_decl,
               "governanca_sha256": (anterior or {}).get("governanca_sha256", h_gov),
               "declarada_em_utc": (anterior or {}).get("declarada_em_utc", agora),
               "declaracao_2": decl2, "declaracao_2_sha256": h_decl2,
               "declaracao_2_em_utc": (anterior or {}).get("declaracao_2_em_utc", agora),
               "K": 2, "resultados_seguem": decl2["resultados_seguem"],
               "status": "declarada antes de rodar, sem resultado (v1 e v2)"}
        if not declarar:
            r = json.loads((saida / fluxo / "resultados.json").read_text())
            d = _recorte_justica(pd.read_parquet(saida / fluxo / "diario.parquet"), fluxo, int(r["corte"]))
            limiares = sorted(set(LIMIARES_DECLARADOS) | {cfg["limiar"]})
            por_ano = {str(int(a)): justica_invertida_por_limiar(gg, limiares, cfg["limiar"])
                       for a, gg in d.groupby("ano")}
            doc.update({"status": "rodada depois da declaração", "rodada_em_utc": agora,
                        "governanca_sha256_na_rodada": h_gov,
                        "entrada": {"n_previsto_no_recorte": int(len(d)), "anos": sorted(por_ano),
                                    "limiar_do_fluxo": cfg["limiar"], "limiares_varridos": limiares,
                                    "corte": int(r["corte"])},
                        "resultados_por_ano": por_ano})
            if fluxo != "prisoes_ri":
                dc = d[d["t_emissao"] >= int(r["corte"])]
                doc["resultados_por_ano_no_cofre"] = {str(int(a)): justica_invertida_por_limiar(gg, limiares, cfg["limiar"])
                                                      for a, gg in dc.groupby("ano")}
            else:
                doc["resultados_por_ano_no_cofre"] = "idêntico a resultados_por_ano: a fatia publicada já é o cofre"
        alvo.parent.mkdir(parents=True, exist_ok=True)
        alvo.write_text(json.dumps(doc, indent=1, ensure_ascii=False, default=float))
        resumo[fluxo] = {"arquivo": str(alvo), "declaracao_sha256": h_decl, "governanca_sha256": h_gov,
                         "status": doc["status"]}
    print(json.dumps(resumo, indent=1, ensure_ascii=False))
    return 0


if __name__ == "__main__":
    import sys as _sys

    _sys.exit(main())
