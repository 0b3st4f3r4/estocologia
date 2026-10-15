"""Compara as três rodadas (original, literatura, fraca) por domínio e aplica o critério da DECLARACAO.md.

Uso: python3 comparar.py <pasta_de_saida_do_rodar.sh> [--tolerancia-dp BANDA] [--json NOME] [--tabela NOME]
Grava resultados.json e tabela.md nesta pasta (nomes trocáveis por --json/--tabela), e confere a rodada (a) contra
os saida/ originais do repositório.

O portão da rodada (a) tem duas partes: a categórica (categoria de status, lista de portões que falham e ação do F8,
igualdade exata) e a numérica, comparada contra uma **banda de equivalência** em dp do posterior da própria rodada
(a). O default 1e-9 é o limiar histórico, que reprova a rodada (a) por diferença numérica; a banda declarada em
DECLARACAO_2.md é 0,05 dp e se passa com --tolerancia-dp 0.05.
"""

import argparse
import json
from pathlib import Path

AQUI = Path(__file__).resolve().parent
REPO = AQUI.parents[1]
VARS = ("original", "literatura", "fraca")
COMPUT = {"rhat_ok", "ess_ok", "sbc_ok"}
TOL_DP = 1e-9


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


def desvio_rodada_a(o, a, h):
    """Desvio da rodada (a) recalculada contra o saida/ do repositório, na unidade da banda.

    H2/H2g: |m_repo - m_a| / dp posterior da rodada (a). No H1 real a forma do resultado não traz a média posterior,
    então a quantidade comparada é a razão dp posterior / dp priori (adimensional, em (0, 1]) e a banda entra nela em
    unidades absolutas. Devolve None quando não há quantidade numérica dos dois lados (domínio podado antes do F4).
    """
    if h == "h1-real":
        if o.get("post_dp_sobre_priori") is None or a.get("post_dp_sobre_priori") is None:
            return None
        return abs(o["post_dp_sobre_priori"] - a["post_dp_sobre_priori"])
    if o.get("post_media") is None or a.get("post_media") is None:
        return None
    if a.get("post_dp"):
        return abs(o["post_media"] - a["post_media"]) / a["post_dp"]
    return abs(o["post_media"] - a["post_media"])


def calcular(sai, tol=TOL_DP):
    res, muda = {}, {"literatura": [], "fraca": []}
    reproducao, desvios = {}, {}
    for h in ("h2", "h2g", "h1-real"):
        rodadas = {v: ler(sai / h / v, h) for v in VARS}
        orig_repo = ler(REPO / h / "saida", h)
        for n, a in rodadas["original"].items():
            o = orig_repo.get(n)
            dev = desvio_rodada_a(o, a, h) if o is not None else None
            desvios[f"{h}/{n}"] = dev
            # categórica (sem banda): categoria de status, portões que falham e ação do F8, iguais;
            # numérica: desvio dentro da banda declarada (tol = 0,05 dp em DECLARACAO_2.md; 1e-9 é o histórico)
            igual = o is not None and chave(o) == chave(a) and (dev is None or dev <= tol)
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
    com_dev = {k: v for k, v in desvios.items() if v is not None}
    pior = max(com_dev, key=com_dev.get) if com_dev else None
    ok = all(reproducao.values())
    if ok:
        vrd_a = (f"a rodada (a) reproduz dentro da banda declarada (desvio máximo {com_dev[pior]:.6g} na unidade da "
                 f"banda, {pior}, contra banda de {tol:g}); o veredito do ponto 5 vale sem rótulo condicional")
    else:
        vrd_a = (f"a rodada (a) não reproduz dentro da banda de {tol:g}: {sum(1 for v in reproducao.values() if not v)} "
                 f"dos {len(reproducao)} domínios reprovam (desvio máximo {com_dev[pior]:.6g} na unidade da banda, "
                 f"{pior}); o veredito publicado continua condicional a essa leitura")
    out = {"tolerancia_dp": tol, "reproducao_rodada_a": reproducao, "reproducao_ok": ok,
           "desvios_rodada_a": desvios, "desvio_max_dp": com_dev[pior] if pior else None,
           "desvio_max_dp_dominio": pior,
           "dominios_sem_quantidade_numerica": sorted(k for k, v in desvios.items() if v is None),
           "veredito_rodada_a": vrd_a,
           "N_b": Nb, "N_c": Nc, "mudancas": muda, "veredito": ver, "dominios": res}
    return out, res


def gravar(out, res, json_nome="resultados.json", tabela_nome="tabela.md"):
    (AQUI / json_nome).write_text(json.dumps(out, indent=1, ensure_ascii=False, default=float))
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
    (AQUI / tabela_nome).write_text("\n".join(L) + "\n")


def main():
    ap = argparse.ArgumentParser(description="Compara as três rodadas e confere a rodada (a) contra os saida/ do repo.")
    ap.add_argument("saida", help="pasta de saída das três rodadas (ex.: /home/silvano-neto/p05-saida)")
    ap.add_argument("--tolerancia-dp", type=float, default=TOL_DP,
                    help="banda de equivalência do portão da rodada (a), em dp do posterior dela "
                         "(default 1e-9, o limiar histórico; a banda declarada em DECLARACAO_2.md é 0.05)")
    ap.add_argument("--json", default="resultados.json", help="nome do JSON gravado nesta pasta")
    ap.add_argument("--tabela", default="tabela.md", help="nome da tabela gravada nesta pasta")
    args = ap.parse_args()
    out, res = calcular(Path(args.saida), args.tolerancia_dp)
    gravar(out, res, args.json, args.tabela)
    print(json.dumps({k: out[k] for k in ("tolerancia_dp", "reproducao_ok", "desvio_max_dp", "desvio_max_dp_dominio",
                                          "N_b", "N_c", "mudancas", "veredito", "veredito_rodada_a")},
                     indent=1, ensure_ascii=False))
    print("reprovam:", [k for k, v in out["reproducao_rodada_a"].items() if not v])


if __name__ == "__main__":
    main()
