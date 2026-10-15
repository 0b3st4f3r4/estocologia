"""Compara as execuções das cópias (copias/) com os resultados publicados e aplica o G3 do próprio horizonte.

- H2: v2 refeito contra o publicado (tem de ser idêntico) e v1 reconstruído contra v2, domínio a domínio;
- H2g com α = 0,05/38 (Bonferroni): o G3 de h2g/portoes.py sobre a saída, contra a conta de analise_multiplicidade.py;
- H2g com o v1 reconstruído: o G3 de h2g/portoes.py e o G3 com Bonferroni (p-valores por subconjunto da saída v1).

Uso:  python3 comparar_execucoes.py <pasta com as saídas h2_v1, h2_v2, h2g_bonf, h2g_v1>   → resultados/execucoes.json
"""

import importlib.util
import json
import sys
from pathlib import Path

AQUI = Path(__file__).resolve().parent
RAIZ = AQUI.parent.parent
SAIDAS = Path(sys.argv[1])
sys.path.insert(0, str(AQUI))
import analise_multiplicidade as A  # noqa: E402

spec = importlib.util.spec_from_file_location("portoes_h2g", RAIZ / "h2g" / "portoes.py")
sys.path.insert(0, str(RAIZ / "h2g"))
PG = importlib.util.module_from_spec(spec)
spec.loader.exec_module(PG)


def carregar(d: Path) -> dict:
    return {p.parent.name: json.loads(p.read_text()) for p in sorted(d.glob("*/resultados.json"))}


def resumo(r: dict) -> dict:
    f4 = r.get("filtro4")
    o = {"status": r["status"], "F3_p": r.get("filtro3", {}).get("p_permutacao")}
    if f4:
        o.update({"rhat_max": f4["posterior"]["rhat_max"], "cobertura_conforme": f4["fora_do_ambiente"]["cobertura_conforme"],
                  "beta_chave_ic90": f4["posterior"]["beta_chave_ic90"], "S_hat": f4["icp"]["S_hat"],
                  "aceitos": len(f4["icp"]["aceitos"]), "decisao": r.get("decisao", {}).get("acao")})
    return o


def g3_bonf(res: dict, a: float) -> dict:
    dom = []
    for nome, r in res.items():
        f4 = r.get("filtro4")
        if not f4:
            continue
        icp = f4["icp"]
        t = {"p": {A.conjunto(k): v for k, v in icp["p_por_subconjunto"].items()},
             "saturados": {A.conjunto(k) for k in icp.get("saturados", [])}}
        S = A.s_hat(t, a)
        if S:
            dom.append((nome, A.area_curta(r["area"]), A.fmt(S)))
    return {"dominios": dom, "areas": sorted({x[1] for x in dom}), "passa": len({x[1] for x in dom}) >= 3}


def main():
    out = {}
    pub, v2, v1 = carregar(RAIZ / "h2" / "saida"), carregar(SAIDAS / "h2_v2"), carregar(SAIDAS / "h2_v1")
    out["h2_v2_identico_ao_publicado"] = {n: PG.sem_volateis(pub[n]) == PG.sem_volateis(v2[n]) for n in pub}
    out["h2_lado_a_lado"] = {n: {"v2": resumo(v2[n]), "v1_reconstruido": resumo(v1[n])} for n in sorted(pub)}
    out["h2_G3"] = {"v2": PG.g3(v2)[1].split(" | ")[0], "v1_reconstruido": PG.g3(v1)[1].split(" | ")[0]}

    bonf = carregar(SAIDAS / "h2g_bonf")
    ok, det = PG.g3(bonf)
    out["h2g_bonferroni_38"] = {"G3_do_portoes": {"passa": ok, "detalhe": det.split(" | ")[0]},
                                "S_hat_por_dominio": {n: r["filtro4"]["icp"]["S_hat"] for n, r in bonf.items() if "filtro4" in r},
                                "alpha": {n: r["filtro4"]["icp"]["alpha"] for n, r in bonf.items() if "filtro4" in r}}
    conta = json.loads((AQUI / "resultados" / "multiplicidade.json").read_text())
    esperado = {d[0]: d[2] for d in conta["correcoes"]["Bonferroni, M_distintos (principal) = 38 (α/M = 0.00132)"]["h2g"]["dominios"]}
    obtido = {n: A.fmt(frozenset(S)) for n, S in out["h2g_bonferroni_38"]["S_hat_por_dominio"].items() if S}
    out["h2g_bonferroni_38"]["bate_com_a_conta"] = esperado == obtido
    pub_g = carregar(RAIZ / "h2g" / "saida")
    out["h2g_bonferroni_38"]["outras_saidas_iguais_ao_publicado"] = {
        n: all(PG.sem_volateis(pub_g[n].get(k)) == PG.sem_volateis(bonf[n].get(k)) for k in ("status", "filtro1", "filtro2", "filtro3"))
        and (("filtro4" not in pub_g[n]) or PG.sem_volateis(pub_g[n]["filtro4"]["posterior"]) == PG.sem_volateis(bonf[n]["filtro4"]["posterior"]))
        for n in pub_g}

    gv1 = carregar(SAIDAS / "h2g_v1")
    ok1, det1 = PG.g3(gv1)
    out["h2g_v1_reconstruido"] = {"lado_a_lado": {n: {"v2_publicado": resumo(pub_g[n]), "v1_reconstruido": resumo(gv1[n])} for n in sorted(pub_g)},
                                  "G3_do_portoes": {"passa": ok1, "detalhe": det1.split(" | ")[0]},
                                  "G3_bonferroni_38": g3_bonf(gv1, 0.05 / 38)}
    (AQUI / "resultados" / "execucoes.json").write_text(json.dumps(out, indent=1, ensure_ascii=False, default=float))
    print("H2 v2 idêntico ao publicado:", all(out["h2_v2_identico_ao_publicado"].values()), out["h2_v2_identico_ao_publicado"])
    print("H2 G3:", out["h2_G3"])
    for n, x in out["h2_lado_a_lado"].items():
        print(f'  {n:27} v2: {x["v2"]["status"][:40]:40} | v1: {x["v1_reconstruido"]["status"][:40]}')
    print("H2g Bonferroni:", out["h2g_bonferroni_38"]["G3_do_portoes"], "| bate com a conta:", out["h2g_bonferroni_38"]["bate_com_a_conta"])
    print("  outras saídas iguais ao publicado:", all(out["h2g_bonferroni_38"]["outras_saidas_iguais_ao_publicado"].values()))
    print("H2g v1:", out["h2g_v1_reconstruido"]["G3_do_portoes"], "| com Bonferroni:", out["h2g_v1_reconstruido"]["G3_bonferroni_38"])
    for n, x in out["h2g_v1_reconstruido"]["lado_a_lado"].items():
        print(f'  {n:27} v2: {x["v2_publicado"]["status"][:40]:40} | v1: {x["v1_reconstruido"]["status"][:40]}')


if __name__ == "__main__":
    main()
