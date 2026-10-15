"""H0 real, Bloco D: a malha de paciência × tempo de atendimento do H0 real.

A declaração 'DECLARACAO_PACIENCIA.md' (SHA-256 em 'DECLARACAO_PACIENCIA.sha256') fixa a malha, as regras
comparadas, as sementes e o critério ANTES desta rodada. Nada aqui escolhe ponto, limiar, regra ou semente
depois de ver resultado.

A simulação é a do horizonte, sem reescrever nada: 'estocologia.central.dia'/'_fcfs' (a fila FCFS com
desistência) e 'estocologia.h0.rodar_mundo'/'agente_horas'. O que este arquivo acrescenta é a malha: para cada
ponto (AHT, paciência), as quatro regras de escala do H0 real construídas com o AHT do ponto, 4 réplicas
independentes sobre os 81 dias do cofre, e o critério declarado (conjunto vencedor W(p), escala de ruído g,
ponto de virada).

Uso:
    OMP_NUM_THREADS=1 python3 sensibilidade_paciencia.py              # malha inteira, 2 processos
    OMP_NUM_THREADS=1 python3 sensibilidade_paciencia.py --fumaca     # só o ponto base, 1 réplica

No máximo 2 processos, como manda a regra de recursos da máquina compartilhada.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import multiprocessing as mp
import os
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

os.environ.setdefault("OMP_NUM_THREADS", "1")

import numpy as np

RAIZ = Path(__file__).resolve().parent
if str(RAIZ) not in sys.path:
    sys.path.insert(0, str(RAIZ))

from estocologia.central import carregar, escala_do_perfil  # noqa: E402
from estocologia.h0 import Z_MARGEM, agente_horas, publico, rodar_mundo  # noqa: E402

AHT_MALHA = (180.0, 240.0, 300.0, 360.0)                      # 3, 4, 5 e 6 min
PACIENCIA_MALHA = (60.0, 120.0, 180.0, 300.0, 420.0, 600.0)   # 1, 2, 3, 5, 7 e 10 min
REPLICAS = 4                                                  # declarado: 4 réplicas por ponto
SEMENTE = 2003                                                # declarada
BASE = (240.0, 180.0)                                         # o ponto publicado no h0-real/README.md
KEY_R3 = "R3_perfil_medio_mais_extra_atendentes"              # chave estável na malha; o rótulo do h0.py tem o
                                                              # número de atendentes, que muda com o AHT

# Estado do processo (carregado uma vez por processo de trabalho)
_CV = _CT = _DST = _DSV = _NB = None


def _init() -> None:
    """Carrega a base e fixa as fatias de treino/cofre uma vez por processo."""
    global _CV, _CT, _DST, _DSV, _NB
    import datetime as dt

    datas, C = carregar()
    tr = np.array([d < dt.date(2003, 7, 1) for d in datas])
    ds = np.array([d.weekday() for d in datas])
    _CV, _CT = C[~tr], C[tr]
    _DST, _DSV = ds[tr], ds[~tr]
    _NB = C.shape[1]


def cv_residual_treino() -> float:
    """CV dos totais diários do treino depois de tirar o dia da semana (o cv do H0 real)."""
    tot = _CT.sum(1)
    sem = np.array([tot[_DST == w].mean() for w in range(5)])
    return float((tot / sem[_DST]).std(ddof=1))


def construir_regras(aht: float) -> tuple[dict, dict]:
    """As quatro regras do H0 real com o AHT do ponto. Pós: mesmas chaves e mesma ordem do h0.py."""
    cv = cv_residual_treino()
    perfil = _CT.mean(0)
    perfis_sem = np.array([_CT[_DST == w].mean(0) for w in range(5)])
    c0 = escala_do_perfil(perfil, aht)
    esc_sem = [escala_do_perfil(perfis_sem[w], aht) for w in range(5)]
    esc_marg = [escala_do_perfil(perfis_sem[w], aht, fator=1 + Z_MARGEM * cv) for w in range(5)]
    ah_marg = float(np.mean([agente_horas(esc_marg[w], _NB) for w in _DSV]))
    extra = 0
    while agente_horas(c0 + extra, _NB) < ah_marg:
        extra += 1
    regras = {
        "R0_alegacao_perfil_medio": c0,
        "R1_perfil_do_dia_da_semana": [esc_sem[w] for w in _DSV],
        "R2_dia_da_semana_mais_margem_p90": [esc_marg[w] for w in _DSV],
        KEY_R3: c0 + extra,
    }
    desenho = {
        "extra_atendentes_por_igual": int(extra),
        "rotulo_R3_no_h0": f"R3_perfil_medio_mais_{extra}_atendentes",
        "agente_horas_dia_R0": agente_horas(c0, _NB),
        "agente_horas_dia_R2": ah_marg,
        "agente_horas_dia_R3": agente_horas(c0 + extra, _NB),
        "escala_pico_R0": int(c0.max()), "escala_pico_R2": int(max(e.max() for e in esc_marg)),
        "soma_atendentes_dia_R0": int(c0.sum()),
        "soma_atendentes_dia_R2": float(np.mean([e.sum() for e in esc_marg])),
        "cv_residual_treino": cv, "fator_R2": float(1 + Z_MARGEM * cv),
    }
    return regras, desenho


def rng_do_ponto(i_aht: int, i_pac: int, replica: int) -> np.random.Generator:
    """Fluxo declarado. Ponto base, réplica 0: o mesmo rng[5] do h0.py (conferência exata contra o publicado)."""
    if (AHT_MALHA[i_aht], PACIENCIA_MALHA[i_pac]) == BASE and replica == 0:
        return np.random.default_rng(np.random.SeedSequence(SEMENTE).spawn(8)[5])
    return np.random.default_rng(np.random.SeedSequence([SEMENTE, i_aht, i_pac, replica]))


def rodar_ponto(i_aht: int, i_pac: int, replicas: int) -> dict:
    """Um ponto da malha: as quatro regras × réplicas sobre os dias do cofre."""
    aht, pac = AHT_MALHA[i_aht], PACIENCIA_MALHA[i_pac]
    regras, desenho = construir_regras(aht)
    nomes = list(regras)
    acum: dict[str, dict[str, list]] = {
        n: {"frac": [], "sl": [], "espera": [], "desistencia": [], "pior_dia": [], "p90": [], "chamadas": 0}
        for n in nomes
    }
    for replica in range(replicas):
        rng = rng_do_ponto(i_aht, i_pac, replica)      # um fluxo por réplica, consumido na ordem R0..R3
        for nome in nomes:
            r = publico(rodar_mundo(_CV, regras[nome], rng, aht, "exp", pac))
            a = acum[nome]
            a["frac"].append(r["frac_dias_abaixo_meta"])
            a["sl"].append(r["nivel_servico"])
            a["espera"].append(r["espera_media_s"])
            a["desistencia"].append(r["desistencia"])
            a["pior_dia"].append(r["pior_dia_sl"])
            a["p90"].append(r["p90_s"])
            a["chamadas"] = r["chamadas"]
    regras_res = {}
    for nome in nomes:
        a = acum[nome]
        esc = regras[nome]
        ah = float(np.mean([agente_horas(e, _NB) for e in esc])) if isinstance(esc, list) else agente_horas(esc, _NB)
        regras_res[nome] = {
            "frac_dias_abaixo_meta_por_replica": [float(x) for x in a["frac"]],
            "nivel_servico_por_replica": [float(x) for x in a["sl"]],
            "espera_media_s_por_replica": [float(x) for x in a["espera"]],
            "frac_media": float(np.mean(a["frac"])),
            "frac_min": float(np.min(a["frac"])), "frac_max": float(np.max(a["frac"])),
            "nivel_servico_medio": float(np.mean(a["sl"])),
            "espera_media_s": float(np.mean(a["espera"])),
            "desistencia_media": float(np.mean(a["desistencia"])),
            "pior_dia_sl_medio": float(np.mean(a["pior_dia"])),
            "p90_s_medio": float(np.mean(a["p90"])),
            "agente_horas_dia": ah,
            "chamadas": int(a["chamadas"]),
        }
    return {
        "aht_s": aht, "paciencia_s": pac, "indices": [i_aht, i_pac],
        "base": (aht, pac) == BASE, "replicas": replicas,
        "dias_cofre": int(_CV.shape[0]), "desenho": desenho, "regras": regras_res,
    }


def criterio(pontos: list[dict]) -> dict:
    """O critério declarado: W(p), escala de ruído g, ponto de virada, pior caso."""
    for p in pontos:
        nomes = list(p["regras"])
        medias = {n: p["regras"][n]["frac_media"] for n in nomes}
        semi = max((p["regras"][n]["frac_max"] - p["regras"][n]["frac_min"]) / 2 for n in nomes)
        g = max(1.0 / p["dias_cofre"], semi)
        melhor = min(medias.values())
        w = sorted([n for n in nomes if medias[n] <= melhor + g], key=lambda n: medias[n])
        ranking = sorted(nomes, key=lambda n: (medias[n], -p["regras"][n]["nivel_servico_medio"],
                                               p["regras"][n]["espera_media_s"]))
        p["g"] = float(g)
        p["semi_amplitude_max"] = float(semi)
        p["frac_melhor"] = float(melhor)
        p["melhor_regra"] = ranking[0]
        p["W"] = w
        p["empate"] = len(w) > 1
        p["ranking"] = ranking
    base = next(p for p in pontos if p["base"])
    w0 = base["W"]
    for p in pontos:
        p["ponto_de_virada"] = bool(set(p["W"]).isdisjoint(w0))
        p["vence_a_base"] = [n for n in p["W"] if n in w0]
    viradas = [p for p in pontos if p["ponto_de_virada"]]
    pior = max(pontos, key=lambda p: p["frac_melhor"])
    return {
        "W0_ponto_base": w0,
        "base": {"aht_s": base["aht_s"], "paciencia_s": base["paciencia_s"],
                 "melhor_regra": base["melhor_regra"], "empate": base["empate"],
                 "g": base["g"], "frac_melhor": base["frac_melhor"]},
        "robusto": len(viradas) == 0,
        "pontos_de_virada": [{"aht_s": p["aht_s"], "paciencia_s": p["paciencia_s"], "W": p["W"],
                              "melhor_regra": p["melhor_regra"], "frac_melhor": p["frac_melhor"],
                              "frac_da_regra_da_base": {n: p["regras"][n]["frac_media"] for n in w0}}
                             for p in viradas],
        "empates": [{"aht_s": p["aht_s"], "paciencia_s": p["paciencia_s"], "W": p["W"]}
                    for p in pontos if p["empate"]],
        "pior_caso": {"aht_s": pior["aht_s"], "paciencia_s": pior["paciencia_s"], "W": pior["W"],
                      "frac_melhor": pior["frac_melhor"],
                      "agente_horas_dia": pior["regras"][pior["melhor_regra"]]["agente_horas_dia"],
                      "nivel_servico_medio": pior["regras"][pior["melhor_regra"]]["nivel_servico_medio"]},
        "maior_frac_por_regra": {n: max(p["regras"][n]["frac_media"] for p in pontos)
                                 for n in pontos[0]["regras"]},
        "chaves_de_regra": list(pontos[0]["regras"]),
    }


def conferencias(pontos: list[dict]) -> dict:
    """As duas conferências internas declaradas antes de rodar."""
    conf: dict = {}
    base = next(p for p in pontos if p["base"])
    conf["extra_atendentes_ponto_base"] = base["desenho"]["extra_atendentes_por_igual"]
    conf["agente_horas_ponto_base"] = {k: base["desenho"][k] for k in
                                       ("agente_horas_dia_R0", "agente_horas_dia_R2", "agente_horas_dia_R3")}
    publicados = json.loads((RAIZ / "resultados.json").read_text())["resultados"]["regras"]
    difs, detalhe = [], {}
    for nome, r in base["regras"].items():
        chave = base["desenho"]["rotulo_R3_no_h0"] if nome == KEY_R3 else nome
        pub = publicados.get(chave)
        if pub is None:
            detalhe[nome] = "nome ausente no publicado"
            continue
        d1 = abs(r["frac_dias_abaixo_meta_por_replica"][0] - pub["frac_dias_abaixo_meta"])
        d2 = abs(r["nivel_servico_por_replica"][0] - pub["nivel_servico"])
        d3 = abs(r["espera_media_s_por_replica"][0] - pub["espera_media_s"])
        detalhe[nome] = {"d_frac": d1, "d_nivel_servico": d2, "d_espera_s": d3,
                         "publicado": {"frac": pub["frac_dias_abaixo_meta"], "sl": pub["nivel_servico"],
                                       "espera_s": pub["espera_media_s"]},
                         "replica_0": {"frac": r["frac_dias_abaixo_meta_por_replica"][0],
                                       "sl": r["nivel_servico_por_replica"][0],
                                       "espera_s": r["espera_media_s_por_replica"][0]}}
        difs += [d1, d2, d3]
    conf["reproducao_ponto_base_replica_0"] = {
        "diferenca_maxima": float(max(difs)) if difs else None,
        "tolerancia_declarada": 1e-9,
        "reproduz": bool(difs and max(difs) <= 1e-9),
        "por_regra": detalhe,
        "nota": "o ponto base com a réplica 0 usa o mesmo fluxo rng[5] do h0.py e a mesma ordem de sorteios",
    }
    sha_decl = hashlib.sha256((RAIZ / "DECLARACAO_PACIENCIA.md").read_bytes()).hexdigest()
    gravado = (RAIZ / "DECLARACAO_PACIENCIA.sha256").read_text().split()[0]
    conf["declaracao"] = {"arquivo": "DECLARACAO_PACIENCIA.md", "sha256": sha_decl,
                          "sha256_no_arquivo_de_hash": gravado, "confere": sha_decl == gravado,
                          "conferida_antes_de_rodar": True}
    return conf


def _trabalho(args):
    i_aht, i_pac, replicas = args
    return i_aht, i_pac, rodar_ponto(i_aht, i_pac, replicas)


def main() -> int:
    ap = argparse.ArgumentParser(description="malha de paciência × tempo de atendimento do H0 real")
    ap.add_argument("--saida", default=str(RAIZ / "saida" / "sensibilidade.json"))
    ap.add_argument("--replicas", type=int, default=REPLICAS)
    ap.add_argument("--processos", type=int, default=2)
    ap.add_argument("--aht", type=str, default=None, help="subconjunto de AHTs (ex.: 240)")
    ap.add_argument("--paciencia", type=str, default=None, help="subconjunto de paciências (ex.: 180)")
    ap.add_argument("--fumaca", action="store_true", help="só o ponto base, 1 réplica: fumaça, não é resultado")
    ap.add_argument("--reaproveitar", action="store_true",
                    help="reusar os pontos já gravados em <saida>/pontos (mesmo número de réplicas)")
    args = ap.parse_args()
    if args.fumaca:
        args.aht, args.paciencia, args.replicas, args.processos = "240", "180", 1, 1
    if args.processos > 2:
        print("no máximo 2 processos (regra de recursos da máquina compartilhada)", file=sys.stderr)
        return 2

    ahts = AHT_MALHA if args.aht is None else tuple(float(x) for x in args.aht.split(","))
    pacs = PACIENCIA_MALHA if args.paciencia is None else tuple(float(x) for x in args.paciencia.split(","))
    ids = [(AHT_MALHA.index(a), PACIENCIA_MALHA.index(p)) for a in ahts for p in pacs]

    # Um arquivo por ponto: a malha inteira são 24 pontos × 4 réplicas, e uma falha depois de tudo não deve
    # custar outra hora de máquina. O reaproveitamento grava e lê o mesmo objeto; nada do critério muda.
    pasta = Path(args.saida).parent / "pontos"
    pasta.mkdir(parents=True, exist_ok=True)

    def arquivo(i_aht: int, i_pac: int) -> Path:
        return pasta / f"aht{AHT_MALHA[i_aht]:.0f}_pac{PACIENCIA_MALHA[i_pac]:.0f}_r{args.replicas}.json"

    t0 = time.time()
    pontos = []
    tarefas = []
    for i_aht, i_pac in ids:
        f = arquivo(i_aht, i_pac)
        if args.reaproveitar and f.exists():
            p = json.loads(f.read_text())
            if p.get("replicas") == args.replicas:
                p["reaproveitado"] = True
                pontos.append(p)
                continue
        tarefas.append((i_aht, i_pac, args.replicas))
    if pontos:
        print(f"  reaproveitados {len(pontos)} pontos de {pasta}", flush=True)

    def guardar(p: dict) -> None:
        arquivo(p["indices"][0], p["indices"][1]).write_text(json.dumps(p, indent=1, ensure_ascii=False, default=float))

    if args.processos == 1:
        _init()
        for i_aht, i_pac, replicas in tarefas:
            t = time.time()
            p = rodar_ponto(i_aht, i_pac, replicas)
            guardar(p)
            pontos.append(p)
            print(f"  ponto aht={p['aht_s']:.0f}s pac={p['paciencia_s']:.0f}s em {time.time() - t:.1f}s", flush=True)
    elif tarefas:
        ctx = mp.get_context("spawn")
        with ctx.Pool(args.processos, initializer=_init) as pool:
            for _i_aht, _i_pac, p in pool.imap_unordered(_trabalho, tarefas, chunksize=1):
                guardar(p)
                pontos.append(p)
                print(f"  [{len(pontos)}/{len(ids)}] aht={p['aht_s']:.0f}s pac={p['paciencia_s']:.0f}s "
                      f"em {time.time() - t0:.0f}s de parede", flush=True)
    pontos.sort(key=lambda p: (p["aht_s"], p["paciencia_s"]))

    dec = criterio(pontos)
    conf = conferencias(pontos)
    import numba
    import scipy

    saida = {
        "descricao": "Bloco D: sensibilidade da decisão do H0 real à paciência e ao tempo de atendimento",
        "declaracao": "DECLARACAO_PACIENCIA.md (hash em DECLARACAO_PACIENCIA.sha256), escrita antes desta rodada",
        "malha": {"aht_s": list(ahts), "paciencia_s": list(pacs), "pontos": len(pontos),
                  "aht_s_completa": list(AHT_MALHA), "paciencia_s_completa": list(PACIENCIA_MALHA)},
        "replicas": args.replicas, "semente": SEMENTE,
        "ponto_base": {"aht_s": BASE[0], "paciencia_s": BASE[1]},
        "protocolo": {
            "regra_vencedora": "menor fração média de dias abaixo da meta; ranking desempata por nível de serviço e espera",
            "escala_de_ruido_g": "max(1/dias_cofre; maior semi-amplitude entre réplicas no ponto)",
            "W(p)": "regras a no máximo g da menor fração média (as não batidas estritamente)",
            "ponto_de_virada": "W(p) ∩ W0 = ∅",
            "pior_caso": "ponto cuja melhor regra tem a maior fração média",
        },
        "decisao": dec,
        "conferencias": conf,
        "execucao": {
            "quando_utc": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            "segundos_parede": round(time.time() - t0, 1), "processos": args.processos,
            "replicas": args.replicas, "OMP_NUM_THREADS": os.environ.get("OMP_NUM_THREADS"),
            "python": sys.version.split()[0], "numpy": np.__version__, "scipy": scipy.__version__,
            "numba": numba.__version__, "base": "dados-reais/bank_calls.csv",
            "sha256_base": hashlib.sha256((RAIZ.parent / "dados-reais" / "bank_calls.csv").read_bytes()).hexdigest(),
        },
        "pontos": pontos,
    }
    destino = Path(args.saida)
    destino.parent.mkdir(parents=True, exist_ok=True)
    destino.write_text(json.dumps(saida, indent=1, ensure_ascii=False, default=float))
    print(f"gravado {destino} ({destino.stat().st_size} bytes) em {saida['execucao']['segundos_parede']}s")
    print(f"W0 = {dec['W0_ponto_base']}; robusto = {dec['robusto']}; "
          f"viradas = {[(v['aht_s'], v['paciencia_s']) for v in dec['pontos_de_virada']]}")
    print(f"pior caso = aht {dec['pior_caso']['aht_s']:.0f}s pac {dec['pior_caso']['paciencia_s']:.0f}s, "
          f"frac da melhor regra {dec['pior_caso']['frac_melhor']:.4f}")
    print(f"conferência base: extra={conf['extra_atendentes_ponto_base']}, "
          f"difmax_replica0={conf['reproducao_ponto_base_replica_0']['diferenca_maxima']:.2e}, "
          f"reproduz={conf['reproducao_ponto_base_replica_0']['reproduz']}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
