"""Compara as três rodadas (original, literatura, fraca) por domínio e aplica o critério da DECLARACAO.md.

Uso: python3 comparar.py <pasta_de_saida_do_rodar.sh> [<pasta_original_do_repo>]
Grava resultados.json e tabela.md nesta pasta, e confere a rodada (a) contra os saida/ originais do repositório.
"""

import json
import sys
from pathlib import Path

AQUI = Path(__file__).resolve().parent
REPO = AQUI.parents[1]
VARS = ("original", "literatura", "fraca")
COMPUT = {"rhat_ok", "ess_ok", "sbc_ok"}


def h2_linha(r):
    st = r["status"]
    cat = st.split(":")[0]
    falhas = []
    if "filtro4" in r:
        falhas = sorted(k for k, v in r["filtro4"]["portao_m15"].items() if not v)
    d = r.get("decisao")
    po = r.get("filtro4", {}).get("posterior")
    return {"status": st, "categoria": cat, "falhas": falhas,
            "acao": d["acao"] if d else None, "p_adotar_compensa": d["p_adotar_compensa"] if d else None,
            "priori_media": r["priori"]["media"], "priori_dp": r["priori"]["dp"],
            "evpi_priori": r["filtro1"]["evpi_priori"], "f1_passa": r["filtro1"]["passa"],
            "f2_informativo": r.get("filtro2", {}).get("informativo"),
            "post_media": po["beta_chave_media"] if po else None, "post_ic90": po["beta_chave_ic90"] if po else None,
            "post_dp_sobre_priori": po["dp_posterior_sobre_dp_priori"] if po else None,
            "post_dp": (po["dp_posterior_sobre_dp_priori"] * r["priori"]["dp"]) if po else None,
            "habilidade_modelo": r.get("filtro4", {}).get("fora_do_ambiente", {}).get("habilidade", {}).get("modelo_ajustado")}


def h1_linha(r):
    falhas = sorted(k for k, v in r["portao_m15"].items() if not v)
    # a "ação" é a melhor ação discreta sob a posterior (proj.acoes(), a mesma que a banca MB7 compara);
    # o F8 só é emitido quando o M15 aprova, e isso já entra na categoria
    acao = r["banca"]["MB7_cenario_atipico"]["decisao_base"]
    pri = r["priori"]
    mb7 = r["banca"]["MB7_cenario_atipico"]
    return {"status": "aprovado" if r["aprovado"] else "reprovado: " + ", ".join(falhas),
            "categoria": "aprovado" if r["aprovado"] else "reprovado", "falhas": falhas, "acao": acao,
            "decisao_base_mb7": mb7["decisao_base"], "quantidade_mediana": mb7["quantidade_base"],
            "priori_beta": [pri["a"], pri["b"]],
            "post_dp_sobre_priori": r["critica"]["3_identificado_vs_importado"]["dp_posterior_sobre_dp_priori"],
            "habilidade_modelo": r["f3"]["placar"].get("agent-f4-frontier", {}).get("habilidade")}


def ler(base, h):
    out = {}
    if h == "h1-real":
        for n in "abc":
            f = base / n / "resultados.json"
            if f.exists():
                out[n] = h1_linha(json.loads(f.read_text()))
        return out
    for f in sorted(base.glob("*/resultados.json")):
        out[f.parent.name] = h2_linha(json.loads(f.read_text()))
    return out


def chave(x):
    return (x["categoria"], tuple(x["falhas"]), x["acao"])


def main():
    sai = Path(sys.argv[1])
    res, muda = {}, {"literatura": [], "fraca": []}
    reproducao = {}
    for h in ("h2", "h2g", "h1-real"):
        rodadas = {v: ler(sai / h / v, h) for v in VARS}
        orig_repo = ler(REPO / h / "saida", h)
        for n, a in rodadas["original"].items():
            o = orig_repo.get(n)
            igual = o is not None and chave(o) == chave(a) and (
                o.get("post_media") is None or abs(o["post_media"] - a["post_media"]) < 1e-9) and (
                h != "h1-real" or abs(o["post_dp_sobre_priori"] - a["post_dp_sobre_priori"]) < 1e-9)
            reproducao[f"{h}/{n}"] = bool(igual)
        for n in rodadas["original"]:
            a = rodadas["original"][n]
            linha = {v: rodadas[v].get(n) for v in VARS}
            for v in ("literatura", "fraca"):
                b = linha[v]
                if b is None:
                    continue
                if a.get("post_media") is not None and b.get("post_media") is not None and a.get("post_dp"):
                    b["desloc_em_dp_post_original"] = abs(b["post_media"] - a["post_media"]) / a["post_dp"]
                if chave(a) != chave(b):
                    comp = set(a["falhas"]) ^ set(b["falhas"])
                    muda[v].append({"dominio": f"{h}/{n}", "de": a["status"], "para": b["status"],
                                    "acao_de": a["acao"], "acao_para": b["acao"],
                                    "so_portao_computacional": bool(comp) and comp <= COMPUT and a["categoria"] == b["categoria"] and a["acao"] == b["acao"]})
            res[f"{h}/{n}"] = linha
    Nb, Nc = len(muda["literatura"]), len(muda["fraca"])
    if Nb == 0 and Nc == 0:
        ver = "as conclusões não dependiam das prioris simuladas"
    elif Nb == 0:
        ver = "não dependiam das prioris simuladas no conteúdo, mas dependiam de haver alguma priori informativa nos domínios que mudam em (c)"
    else:
        ver = "dependiam das prioris simuladas nos domínios que mudam em (b)"
    out = {"reproducao_rodada_a": reproducao, "reproducao_ok": all(reproducao.values()), "N_b": Nb, "N_c": Nc,
           "mudancas": muda, "veredito": ver, "dominios": res}
    (AQUI / "resultados.json").write_text(json.dumps(out, indent=1, ensure_ascii=False, default=float))
    # tabela
    f = lambda x, d=3: "—" if x is None else (f"{x:.{d}g}" if isinstance(x, float) else str(x))
    L = ["| domínio | status (a) original | status (b) literatura | status (c) fraca | F8 a / b / c | média posterior a / b / c | dp post/dp priori a / b / c | desloc. b, c (dp post a) |",
         "|---|---|---|---|---|---|---|---|"]
    for n, l in res.items():
        a, b, c = (l[v] for v in VARS)
        st = lambda x: x["status"].replace("aprovado no F4: decisão emitida", "aprovado").replace("podado no F3: a chave não vence o nulo de permutação", "podado F3").replace("reprovado no F4 (M15): ", "reprovado: ")
        L.append(f"| {n} | {st(a)} | {st(b)} | {st(c)} | {f(a['acao'])} / {f(b['acao'])} / {f(c['acao'])} | "
                 f"{f(a.get('post_media'))} / {f(b.get('post_media'))} / {f(c.get('post_media'))} | "
                 f"{f(a.get('post_dp_sobre_priori'), 2)} / {f(b.get('post_dp_sobre_priori'), 2)} / {f(c.get('post_dp_sobre_priori'), 2)} | "
                 f"{f(b.get('desloc_em_dp_post_original'), 2)}, {f(c.get('desloc_em_dp_post_original'), 2)} |")
    (AQUI / "tabela.md").write_text("\n".join(L) + "\n")
    print(json.dumps({k: out[k] for k in ("reproducao_ok", "N_b", "N_c", "mudancas", "veredito")}, indent=1, ensure_ascii=False))
    print([k for k, v in reproducao.items() if not v])


if __name__ == "__main__":
    main()
