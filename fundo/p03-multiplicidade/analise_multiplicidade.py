"""Ponto 3: inventário dos testes ICP do H2 … H2g, K global e correções de multiplicidade (DECLARACAO.md).

Lê os resultados gravados (h2*/saida/*/resultados.json) e os 8 testes refeitos por icp_direto.py
(resultados/icp_h2_posto_corrigido.json e resultados/icp_h2b_diagnosticos.json). Não roda nenhum horizonte.

Uso:  python3 analise_multiplicidade.py      → resultados/multiplicidade.json e tabelas no terminal
"""

import json
import math
from pathlib import Path

AQUI = Path(__file__).resolve().parent
RAIZ = AQUI.parent.parent
RODADAS = ["h2", "h2b", "h2c", "h2d", "h2e", "h2f", "h2g"]
ALFA = 0.05


def area_curta(a: str) -> str:
    return a.split()[0].strip("(),")


def conjunto(chave: str) -> frozenset:
    return frozenset() if chave == "∅" else frozenset(chave.split("+"))


def familia(t: dict) -> dict:
    """Subconjuntos que contam (fora os saturados quando a regra K = 3 está ativa)."""
    return {S: p for S, p in t["p"].items() if S not in t["saturados"]}


def s_hat(t: dict, a: float):
    """Ŝ do ICP no nível a: interseção dos aceitos (p > a) da família; None se nenhum é aceito."""
    ac = [S for S, p in familia(t).items() if p > a]
    if not ac:
        return None
    return frozenset.intersection(*ac)


def estatisticas(t: dict) -> dict:
    F = familia(t)
    cand = sorted(set().union(*F)) if F else []
    p_max = max(F.values())
    p_j = {j: max((p for S, p in F.items() if j not in S), default=0.0) for j in cand}
    j_min = min(p_j, key=p_j.get) if p_j else None
    return {"p_max": p_max, "p_j": p_j, "p_dom": p_j[j_min] if j_min else 1.0, "j_min": j_min}


def conta_area(t: dict, S) -> bool:
    """Ŝ ≠ ∅ conta para o G3; na genética, aplica também a regra declarada do H2f ({plate} sozinho não conta)."""
    return bool(S)


def conta_area_regra_plate(t: dict, S) -> bool:
    if not S:
        return False
    if t["area"] == "Genética":
        return any(v != "plate" for v in S)
    return True


def carregar_testes() -> list[dict]:
    testes = []
    for h in RODADAS:
        for f in sorted((RAIZ / h / "saida").glob("*/resultados.json")):
            r = json.loads(f.read_text())
            f4 = r.get("filtro4")
            if not f4:
                continue
            icp = f4["icp"]
            p = {conjunto(k): float(v) for k, v in icp["p_por_subconjunto"].items()}
            sat = {conjunto(k) for k in icp.get("saturados", [])} if icp.get("regra_saturacao", {}).get("ativa") else set()
            testes.append({"rodada": h, "dominio": r["dominio"], "area": area_curta(r["area"]), "K": r["K"],
                           "alpha_original": float(icp.get("alpha", 0.05)), "teste": icp.get("teste") or "K1 linear (sem posto)",
                           "p": p, "saturados": sat, "S_hat_gravado": frozenset(icp["S_hat"]),
                           "aceitos_gravados": len(icp["aceitos"]), "tipo": "F4", "origem": str(f.relative_to(RAIZ))})
    extras = [("resultados/icp_h2_posto_corrigido.json", "reexecução do posto (H2b)", "K1 linear com posto"),
              ("resultados/icp_h2b_diagnosticos.json", "diagnóstico depois da poda (H2b)", "K1 linear com posto")]
    for arq, tipo, teste in extras:
        for nome, r in json.loads((AQUI / arq).read_text()).items():
            testes.append({"rodada": "h2b", "dominio": nome, "area": area_curta(r["area"]), "K": 1, "alpha_original": r["alpha"],
                           "teste": teste, "p": {conjunto(k): float(v) for k, v in r["p_por_subconjunto"].items()},
                           "saturados": set(), "S_hat_gravado": frozenset(r["S_hat"]), "aceitos_gravados": r["aceitos"],
                           "tipo": tipo, "origem": arq})
    return testes


def assinatura(t: dict) -> tuple:
    return (t["dominio"], t["teste"], tuple(sorted((tuple(sorted(S)), round(p, 12)) for S, p in t["p"].items())),
            tuple(sorted(tuple(sorted(S)) for S in t["saturados"])))


def holm(pvals: list[float], alfa: float) -> list[float | None]:
    """Devolve, por hipótese, o limiar em que foi rejeitada (None se não foi)."""
    M = len(pvals)
    ordem = sorted(range(M), key=lambda i: pvals[i])
    out = [None] * M
    for k, i in enumerate(ordem):
        lim = alfa / (M - k)
        if pvals[i] <= lim:
            out[i] = lim
        else:
            break
    return out


def by(pvals: list[float], alfa: float) -> list[float | None]:
    M = len(pvals)
    c = sum(1 / k for k in range(1, M + 1))
    ordem = sorted(range(M), key=lambda i: pvals[i])
    kmax = 0
    for k, i in enumerate(ordem, start=1):
        if pvals[i] <= k * alfa / (M * c):
            kmax = k
    lim = kmax * alfa / (M * c)
    return [lim if (kmax and pvals[i] <= lim) else None for i in range(M)]


def fmt(S) -> str:
    return "rejeita tudo" if S is None else ("∅" if not S else "{" + ", ".join(sorted(S)) + "}")


def main():
    testes = carregar_testes()
    for t in testes:
        t.update(estatisticas(t))
    # 1. conferência: a regra p_dom ≤ a < p_max reproduz o Ŝ gravado no α original
    falhas = []
    for t in testes:
        S = s_hat(t, t["alpha_original"])
        S_ = frozenset() if S is None else S
        n_ac = sum(p > t["alpha_original"] for p in familia(t).values())
        if S_ != t["S_hat_gravado"] or n_ac != t["aceitos_gravados"]:
            falhas.append((t["rodada"], t["dominio"], fmt(S), sorted(t["S_hat_gravado"]), n_ac, t["aceitos_gravados"]))
        nao_vazio = bool(S)
        assert nao_vazio == (t["p_dom"] <= t["alpha_original"] < t["p_max"]), (t["dominio"], t["rodada"])
    if falhas:
        print("FALHA NA CONFERÊNCIA:", falhas)
        raise SystemExit(1)
    print(f"conferência: os {len(testes)} testes reproduzem Ŝ e o número de aceitos gravados no α original")

    # 2. distintos e contagens de M
    distintos, vistos = [], {}
    for t in testes:
        a = assinatura(t)
        if a in vistos:
            t["repete"] = vistos[a]
        else:
            vistos[a] = f'{t["rodada"]}/{t["dominio"]}'
            t["repete"] = None
            distintos.append(t)
    n_exec_f4 = sum(t["tipo"] == "F4" for t in testes)
    n_rank = sum(t["tipo"].startswith("reexecução") for t in testes)
    n_diag = sum(t["tipo"].startswith("diagnóstico") for t in testes)
    M_dist = len(distintos)
    M = {"M_distintos (principal)": M_dist, "M_sem_reexecucoes_do_posto": M_dist - n_rank,
         "M_todas_as_execucoes_vistas": n_exec_f4 + n_diag + n_rank + 10}
    print("execuções do F4 gravadas:", n_exec_f4, "| diagnósticos:", n_diag, "| reexecuções do posto:", n_rank,
          "| distintos:", M_dist, "| M:", M)

    # 3. correções. Família principal: os distintos. Holm e BY também sobre M = 32 e M = 72 (duplicatas repetem o p).
    def g3_rodada(rodada: str, nivel_por_teste, regra=conta_area) -> dict:
        sel = [t for t in testes if t["rodada"] == rodada and t["tipo"] == "F4"]
        dom = []
        for t in sel:
            a = nivel_por_teste(t)
            S = s_hat(t, a) if a is not None else None
            if a is not None and S is not None and regra(t, S):
                dom.append((t["dominio"], t["area"], fmt(S), a))
        areas = sorted({x[1] for x in dom})
        return {"dominios": dom, "areas": areas, "passa": len(areas) >= 3}

    sig_idx = {assinatura(t): i for i, t in enumerate(distintos)}
    p_dist = [t["p_dom"] for t in distintos]

    def nivel_via_procedimento(lims_dist):
        return lambda t: lims_dist[sig_idx[assinatura(t)]]

    resultados = {"M": M, "alfa": ALFA, "correcoes": {}}
    # original (sem correção): o α de cada teste
    resultados["correcoes"]["sem correção (α do horizonte)"] = {
        r: g3_rodada(r, lambda t: t["alpha_original"]) for r in RODADAS}
    # Bonferroni
    for nomeM, m in M.items():
        a = ALFA / m
        resultados["correcoes"][f"Bonferroni, {nomeM} = {m} (α/M = {a:.3g})"] = {r: g3_rodada(r, lambda t, a=a: a) for r in RODADAS}
    # Holm e BY
    for nomeM, m in M.items():
        extra = m - M_dist
        # duplicatas: para M = 72, as execuções repetidas entram com o mesmo p; para M = 32, saem as 6 do posto
        if extra >= 0:
            pv = p_dist + [p for p in sorted(t["p_dom"] for t in testes if t["repete"])][:max(0, extra - 10)]
            # as 10 execuções do v1 repetem os ICPs do H2 (exceto o Preston, podado no v1): mesmo p_dom
            v1 = [t["p_dom"] for t in testes if t["rodada"] == "h2" and t["dominio"] != "desenvolvimento_preston"]
            pv = pv + (v1 if extra >= 10 else [])
            assert len(pv) == m, (len(pv), m)
            base = distintos
        else:
            base = [t for t in distintos if not t["tipo"].startswith("reexecução")]
            pv = [t["p_dom"] for t in base]
        for nomeP, proc in (("Holm", holm), ("Benjamini–Yekutieli", by)):
            lims = proc(pv, ALFA)
            lim_base = {assinatura(t): lims[i] for i, t in enumerate(base)}
            resultados["correcoes"][f"{nomeP}, {nomeM} = {m}"] = {
                r: g3_rodada(r, lambda t, lb=lim_base: lb.get(assinatura(t))) for r in RODADAS}
    # sequencial: α/7 por rodada, Bonferroni sobre os m_r testes avaliados na rodada
    m_r = {"h2": 11, "h2b": 11, "h2c": 3, "h2d": 5, "h2e": 7, "h2f": 12, "h2g": 13}
    for r in RODADAS:
        n = sum(t["rodada"] == r for t in testes)
        assert n == m_r[r], (r, n)
    resultados["correcoes"]["sequencial, α/7 por rodada e Bonferroni na rodada"] = {
        r: g3_rodada(r, lambda t, r=r: ALFA / (7 * m_r[r])) for r in RODADAS}
    resultados["m_r"] = m_r

    # regra do plate (H2f), para a rodada H2g
    resultados["regra_plate_h2g"] = {
        "sem correção": g3_rodada("h2g", lambda t: t["alpha_original"], conta_area_regra_plate),
        f"Bonferroni M = {M_dist}": g3_rodada("h2g", lambda t: ALFA / M_dist, conta_area_regra_plate)}

    # sensibilidade NÃO declarada (a não monotonia apareceu depois): Ŝ ≠ ∅ no nível de Bonferroni E modelo não
    # rejeitado no α original do horizonte (p_max > α original), para não contar Ŝ de modelos quase rejeitados
    resultados["nao_declarada_bonferroni_e_modelo_nao_rejeitado_no_alfa_original"] = {
        r: g3_rodada(r, lambda t: (ALFA / M_dist) if t["p_max"] > t["alpha_original"] else None) for r in RODADAS}
    resultados["nao_declarada_bonferroni_e_p_max_acima_de_0_05"] = {
        r: g3_rodada(r, lambda t: (ALFA / M_dist) if t["p_max"] > 0.05 else None) for r in RODADAS}

    # leitura agrupada (informativa): união das descobertas de todas as rodadas por área, Bonferroni M principal
    a = ALFA / M_dist
    uni = sorted({(t["area"], t["dominio"], fmt(s_hat(t, a))) for t in distintos if s_hat(t, a)})
    resultados["agrupado_bonferroni_principal"] = {"descobertas": uni, "areas": sorted({x[0] for x in uni})}

    # tabela por teste
    tab = []
    for t in testes:
        tab.append({"rodada": t["rodada"], "dominio": t["dominio"], "area": t["area"], "K": t["K"], "tipo": t["tipo"],
                    "teste": t["teste"], "alpha_original": t["alpha_original"], "subconjuntos": len(t["p"]),
                    "saturados": len(t["saturados"]), "p_max": t["p_max"], "p_dom": t["p_dom"], "variavel_do_p_dom": t["j_min"],
                    "bits_de_evidencia": -math.log2(t["p_dom"]) if t["p_dom"] > 0 else float("inf"),
                    "S_hat_original": fmt(s_hat(t, t["alpha_original"])),
                    "S_hat_bonferroni_principal": fmt(s_hat(t, ALFA / M_dist)),
                    "repete": t["repete"], "origem": t["origem"]})
    resultados["testes"] = tab
    resultados["custo_occam_bits"] = {k: math.log2(m) + math.log2(1 / ALFA) for k, m in M.items()}
    (AQUI / "resultados" / "multiplicidade.json").write_text(json.dumps(resultados, indent=1, ensure_ascii=False, default=str))

    print("\nTESTES (p_dom = menor p de variável; Ŝ ≠ ∅ no nível a ⇔ p_dom ≤ a < p_max)")
    for x in tab:
        print(f'{x["rodada"]:4} {x["dominio"]:27} {x["area"]:13} {x["tipo"][:12]:12} α={x["alpha_original"]:<6g} '
              f'p_max={x["p_max"]:.3g} p_dom={x["p_dom"]:.3g} ({x["variavel_do_p_dom"]}) Ŝ={x["S_hat_original"]} '
              f'→ Bonf: {x["S_hat_bonferroni_principal"]}{"  [repete " + x["repete"] + "]" if x["repete"] else ""}')
    print("\nG3 POR CORREÇÃO (áreas com Ŝ ≠ ∅)")
    for nome, por in resultados["correcoes"].items():
        linha = "; ".join(f'{r}: {len(v["areas"])} {v["areas"]}' for r, v in por.items())
        print(f"- {nome}\n    {linha}\n    H2g passa: {por['h2g']['passa']}; alguma rodada passa: {any(v['passa'] for v in por.values())}")
    print("\nregra do plate no H2g:", {k: (v["areas"], v["passa"]) for k, v in resultados["regra_plate_h2g"].items()})
    for k in ("nao_declarada_bonferroni_e_modelo_nao_rejeitado_no_alfa_original", "nao_declarada_bonferroni_e_p_max_acima_de_0_05"):
        print(k, {r: v["areas"] for r, v in resultados[k].items()})
    print("agrupado (Bonferroni principal):", resultados["agrupado_bonferroni_principal"])
    print("custo de Occam (bits):", resultados["custo_occam_bits"])


if __name__ == "__main__":
    main()
