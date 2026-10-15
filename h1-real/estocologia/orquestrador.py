"""agent-m15-orchestrator: a máquina de estados de 6 estágios, as podas, as lentes e a banca adversarial."""

from __future__ import annotations

import copy
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

import numpy as np

from . import agentes as A
from . import estat
from .barramento import Barramento
from .diario import Diario
from .projeto import Projeto

RAIZ = Path(__file__).resolve().parent.parent


def _json(o):
    if isinstance(o, dict):
        return {k: _json(v) for k, v in o.items() if not str(k).startswith("_")}
    if isinstance(o, (list, tuple)):
        return [_json(v) for v in o]
    if isinstance(o, (np.floating, np.integer)):
        return o.item()
    if isinstance(o, np.ndarray):
        return o.tolist()
    if isinstance(o, np.bool_):
        return bool(o)
    return o


# =====================================================================================
# Estágio 5 · Banca adversarial (MB1–MB8) e Ficha da Crítica de Modelos (M15)
# =====================================================================================
def banca(proj: Projeto, bus: Barramento, loja: A.Loja, ctx: dict, rng) -> dict:
    ataques = {}
    contrato = proj.contrato()
    cols = list(contrato["colunas"])

    # MB1 · nova bateria de propriedades, com mais exemplos e semente que o autor não escolheu
    env = {**os.environ, "HYPOTHESIS_PROFILE": "banca"}
    t = subprocess.run([sys.executable, "-m", "pytest", "-q", f"tests/test_{proj.nome}.py"], cwd=RAIZ, capture_output=True, text=True, env=env)
    ataques["MB1_propriedades_ineditas"] = {"passou": t.returncode == 0, "detalhe": (t.stdout.strip().splitlines() or ["?"])[-1]}

    # MB2 · lote corrompido: nulos, fora de faixa e chave duplicada; depois reingestão idêntica
    lote = proj.gerar_lote(99, np.random.default_rng(99))
    ruins = copy.deepcopy(lote)
    alvo_nulo, alvo_faixa = cols[-1], next(c for c, s in contrato["colunas"].items() if "max" in s or "valores" in s)
    for i in range(0, 5):
        ruins[i][alvo_nulo] = None
    for i in range(5, 10):
        s = contrato["colunas"][alvo_faixa]
        ruins[i][alvo_faixa] = (s["max"] + 10) if "max" in s else "VALOR_INVALIDO"
    ruins += [dict(ruins[i]) for i in range(20, 25)]
    with tempfile.TemporaryDirectory() as tmp:
        lj = A.Loja(Path(tmp), proj)
        r1 = lj.ingerir(ruins, 99, contrato)
        r2 = lj.ingerir(ruins, 99, contrato)
        try:
            lj.ler([99], ["*"])
            recusa = False
        except PermissionError:
            recusa = True
    ok = r1["quarentena"] == 15 and r2["idempotente"] and recusa
    ataques["MB2_lote_corrompido"] = {"passou": ok, "quarentena": r1["quarentena"], "esperado": 15, "reingestao_idempotente": r2["idempotente"],
                                      "select_estrela_recusado": recusa, "motivos": r1.get("motivos")}

    # MB3 · deriva injetada: o monitor KS precisa disparar
    deriva_lote = proj.gerar_lote(98, np.random.default_rng(98), cenario="deriva")
    with tempfile.TemporaryDirectory() as tmp:
        lj = A.Loja(Path(tmp), proj)
        lj.ingerir(deriva_lote, 98, contrato)
        dados_deriva = lj.ler([98], cols)
    ref = proj.entradas_cofre(loja.ler([proj.periodos[-1]], cols, agente="banca"))["deriva"]
    dv = estat.deriva(ref, proj.entradas_cofre(dados_deriva)["deriva"])
    ataques["MB3_deriva_injetada"] = {"passou": dv["p_ks"] < 0.01, **dv}

    # MB4 · dado patológico: o mundo viola o modelo; a checagem preditiva precisa acusar
    pat = proj.gerar_lote(97, np.random.default_rng(97), cenario="patologico")
    with tempfile.TemporaryDirectory() as tmp:
        lj = A.Loja(Path(tmp), proj)
        lj.ingerir(pat, 97, contrato)
        dados_pat = lj.ler([97], cols)
    a_pat = A.amostrar(proj, proj.suficientes(dados_pat), ctx["priori"], rng, cadeias=2, n=1500, aquec=500)
    ppc_pat = proj.ppc(dados_pat, a_pat["cadeias"].reshape(-1, proj.dim), rng)
    ataques["MB4_dado_patologico"] = {"passou": ppc_pat["p_valor"] < 0.01 and ctx["ppc"]["p_valor"] >= 0.01,
                                      "ppc_patologico": ppc_pat, "ppc_mundo": ctx["ppc"]}

    # MB5 · adulteração da trilha de auditoria
    with tempfile.TemporaryDirectory() as tmp:
        c = Path(tmp) / "bus.jsonl"
        shutil.copy(bus.caminho, c)
        linhas = c.read_text().splitlines()
        j = json.loads(linhas[len(linhas) // 2])
        j["payload"]["_adulterado"] = True
        linhas[len(linhas) // 2] = json.dumps(j, ensure_ascii=False)
        c.write_text("\n".join(linhas) + "\n")
        quebras = Barramento(c).verificar()
    ataques["MB5_trilha_adulterada"] = {"passou": bool(quebras) and not bus.verificar(), "quebras_detectadas": quebras[:2],
                                       "responsavel_nomeado": ctx["f5"]["responsavel_humano"]["nome"]}

    # MB6 · inversão da métrica de justiça: quem passa a pagar o erro?
    inv = ctx["f6"].get("inversao_metrica")
    ataques["MB6_inversao_metrica"] = {"passou": bool(inv and inv.get("perdem") and inv.get("ganham")), "inversao": inv}

    # MB7 · cenário atípico do domínio: priori alternativa, a decisão resiste?
    pri_alt = A.f7_elicitar(proj.priori_atipica())["priori"]
    a_alt = A.amostrar(proj, ctx["suf"], pri_alt, rng, cadeias=2, n=1500, aquec=500)
    q_alt = a_alt["q"].ravel()
    acoes = proj.acoes()
    melhor = lambda qs: acoes[int(np.argmax([np.mean([proj.utilidade(ac, th) for th in qs[::10]]) for ac in acoes]))]
    d0, d1 = melhor(ctx["f4"]["_q"]), melhor(q_alt)
    ataques["MB7_cenario_atipico"] = {"passou": True, "quantidade_base": float(np.median(ctx["f4"]["_q"])), "quantidade_priori_atipica": float(np.median(q_alt)),
                                      "decisao_base": d0, "decisao_priori_atipica": d1, "decisao_muda": d0 != d1,
                                      "leitura": "decisão robusta à priori" if d0 == d1 else "decisão sensível à priori: entra no termo de escopo"}

    # MB8 · violação da cláusula de saída
    disp_deriva = proj.disjuntor(dados_deriva, ctx)
    disp_mundo = proj.disjuntor(loja.ler([proj.periodos[-1]], cols, agente="banca"), ctx)
    ataques["MB8_disjuntor_violado"] = {"passou": disp_deriva["disparou"] and not disp_mundo["disparou"], "com_deriva": disp_deriva, "mundo_normal": disp_mundo}

    return ataques


def critica(proj: Projeto, ctx: dict, K: int) -> dict:
    """As 6 perguntas da Ficha da Crítica de Modelos (M15)."""
    q = ctx["f4"]["_q"]
    rr = np.random.default_rng(1)
    pri = np.array([proj.quantidade(proj.amostra_priori(ctx["priori"], rr)) for _ in range(4000)])
    encolhimento = float(np.std(q) / np.std(pri))
    c = proj.critica(ctx)
    return {
        "1_checagem_preditiva": ctx["ppc"],
        "2_modelo_rival": c["rival"],
        "3_identificado_vs_importado": {"dp_posterior_sobre_dp_priori": encolhimento,
                                        "leitura": "identificado pelos dados" if encolhimento < 0.5 else "dominado pela priori importada"},
        "4_performatividade": c["performatividade"],
        "5_premissas_indecidiveis": c["premissas"],
        "6_tentativas": K,
    }


# =====================================================================================
# O ciclo
# =====================================================================================
def rodar(proj: Projeto, bus: Barramento, diario: Diario, saida: Path, semente: int) -> dict:
    rng = np.random.default_rng(semente)
    saida = Path(saida) / proj.nome
    saida.mkdir(parents=True, exist_ok=True)
    loja = A.Loja(saida.parent / "loja", proj)
    ctx: dict = {"semente": semente}
    K = 1
    log: list[str] = []

    # 1 · Tradução e elicitação
    ctx["priori"] = A.f7(proj, bus)
    log.append("1 · F7: tradução, SHELF e termo de escopo")

    # 2 · Ingestão, grão, ética
    ctx["f2"] = A.f2(proj, bus, loja, rng)
    treino_p = proj.periodos[:-1]
    ctx["treino"] = loja.ler(treino_p, list(proj.colunas_modelo))
    cols_todas = list(proj.contrato()["colunas"])
    bruto = loja.ler(treino_p, cols_todas)
    linhas = [dict(zip(cols_todas, vals)) for vals in zip(*[bruto[c].tolist() for c in cols_todas])]
    ctx["f5"] = A.f5(proj, bus, linhas, ctx["treino"], rng)
    ctx["f6"] = A.f6(proj, bus, ctx)
    log.append("2 · F2 + F5 + F6")

    # 3 · Simulação e fronteira, com o portão crisonômico antes da inferência pesada
    ctx["suf"] = proj.suficientes(ctx["treino"])
    rr = np.random.default_rng(semente + 7)
    pri_q = np.array([proj.quantidade(proj.amostra_priori(ctx["priori"], rr)) for _ in range(2000)])
    vi = estat.evpi(pri_q, proj.utilidade, proj.acoes())
    poda = vi["evpi"] < proj.custo_inferencia
    A.publicar(bus, proj, 3, "agent-m15-orchestrator", "portao_crisonomico",
               {"evpi_priori": vi["evpi"], "custo_inferencia": proj.custo_inferencia, "podado": poda,
                "log": "Podado por Crisonomia: curiosidade sem uso operacional (M11/M14)" if poda else "inferência autorizada"})
    ctx["f1"] = A.f1(proj, bus, ctx["treino"], rng)
    ultimo_treino = loja.ler([treino_p[-1]], list(proj.colunas_modelo))
    calib = proj.resultado_unidade(ultimo_treino)
    ctx["f4"] = A.f4(proj, bus, ctx["suf"], ctx["priori"], calib, rng)
    ctx["ppc"] = proj.ppc(ctx["treino"], ctx["f4"]["_amostras"], rng)
    ctx["f3"] = A.f3(proj, bus, loja, ctx["treino"], ctx["f4"]["_amostras"], ctx["f4"]["conforme"]["intervalo"], diario, K)
    log.append("3 · F1 + F4 + F3")

    # 4 · As 5 lentes
    ctx["lentes"] = proj.lentes(ctx)
    A.publicar(bus, proj, 4, "agent-m15-orchestrator", "lentes", _json(ctx["lentes"]))
    log.append("4 · lentes")

    # 5 · Banca adversarial e portão M15
    ctx["banca"] = banca(proj, bus, loja, ctx, rng)
    ctx["critica"] = critica(proj, ctx, K)
    modelo = ctx["f3"]["placar"].get("agent-f4-frontier", {})
    portao = {
        "rhat_ok": ctx["f4"]["diagnostico"]["rhat_max"] < 1.01,
        "ess_ok": ctx["f4"]["diagnostico"]["ess_min"] > 400,
        "sbc_ok": ctx["f4"]["sbc"]["p_uniformidade"] > 0.01,
        "ppc_ok": ctx["ppc"]["p_valor"] >= 0.01,
        "conforme_ok": ctx["f3"]["cobertura_conforme_out_of_time"] >= 1 - ctx["f4"]["conforme"]["alpha"] - 0.05,
        "modelo_vence_climatologia": modelo.get("habilidade", -1) > 0,
        "hypothesis_ok": ctx["f1"]["hypothesis"]["ok"],
        "trilha_integra": not bus.verificar(),
        "banca": all(v["passou"] for v in ctx["banca"].values()),
    }
    aprovado = all(portao.values())
    A.publicar(bus, proj, 5, "agent-m15-orchestrator", "banca_e_critica", _json({"ataques": ctx["banca"], "critica": ctx["critica"], "portao": portao, "aprovado": aprovado}))
    log.append(f"5 · banca: {'aprovado' if aprovado else 'REPROVADO'}")

    # 6 · Decisão (só depois de aprovado)
    if aprovado:
        ctx["f8"] = A.f8(proj, bus, ctx, saida)
        log.append("6 · F8: decisão executiva")
    res = _json({k: v for k, v in ctx.items() if k not in ("treino",)})
    res["portao_m15"] = portao
    res["aprovado"] = aprovado
    res["log"] = log
    res["varredura"] = loja.log_varredura
    (saida / "resultados.json").write_text(json.dumps(res, indent=1, ensure_ascii=False, default=str))
    return res
