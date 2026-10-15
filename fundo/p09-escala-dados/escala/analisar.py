"""Junta as medidas da Parte 2: seleção R1, lote, perfil (cProfile + tracemalloc), hipergrafo e extrapolação.

Uso:  python3 escala/analisar.py <pasta_dados>
Lê <dados>/dominios_auto/_selecao.json, <dados>/saida_lote/_lote.json, <dados>/saida_perfil/_lote.json e os
.pstats; monta o hipergrafo (grafo.py) sobre os 18 do H2g + 2 de saúde + os automáticos; grava
escala/analise.json (pequeno) e <dados>/hipergrafo_p09.json (grande).
"""

import io
import json
import os
import pstats
import sys
import time
import tracemalloc
from collections import Counter
from pathlib import Path

import numpy as np

RAIZ = Path(__file__).resolve().parents[1]


def tipo_no(i: str) -> str:
    if i == "RAIZ":
        return "raiz"
    if ":" not in i:
        return "dominio"
    s = i.split(":", 1)[1]
    if s in ("L1", "L2", "L3"):
        return "camada"
    if s.startswith("H["):
        return "hipotese_icp"
    if s.startswith("amb="):
        return "ambiente"
    if s.startswith("filtro"):
        return "filtro"
    if s == "decisao":
        return "decisao"
    return "variavel"


def classe(i: str) -> str:
    d = i.split(":", 1)[0]
    return "raiz" if d == "RAIZ" else "automatico" if d.startswith("auto_") else "curado"


def montar_grafo(dados: Path, saidas: list, dominios: list, destino: Path) -> dict:
    os.environ["ESTOCOLOGIA_DOMINIOS"] = os.pathsep.join(map(str, dominios))
    os.environ["ESTOCOLOGIA_SAIDAS"] = os.pathsep.join(map(str, saidas))
    os.environ["ESTOCOLOGIA_GRAFO"] = str(destino)
    sys.path.insert(0, str(RAIZ))
    import importlib

    import grafo
    importlib.reload(grafo)
    tracemalloc.start()
    t = time.time()
    grafo.main()
    seg = time.time() - t
    pico = tracemalloc.get_traced_memory()[1]
    tracemalloc.stop()
    g = json.loads(destino.read_text())
    cont = Counter((classe(n["id"]), tipo_no(n["id"])) for n in g["nos"])
    pontes = Counter(e["rotulo"] for e in g["arestas_graph"])
    return {"contagem": g["contagem"], "segundos_grafo": round(seg, 2), "tracemalloc_pico_grafo_bytes": int(pico),
            "bytes_json": destino.stat().st_size,
            "nos_por_classe_e_tipo": {f"{a}|{b}": c for (a, b), c in sorted(cont.items())},
            "pontes_por_rotulo": dict(pontes.most_common()),
            "dominios_no_grafo": len({n["id"] for n in g["nos"] if tipo_no(n["id"]) == "dominio"})}


def perfil(pasta: Path, top=25) -> dict:
    arqs = sorted(pasta.glob("*/perfil.pstats"))
    if not arqs:
        return {}
    st = pstats.Stats(str(arqs[0]), stream=io.StringIO())
    for a in arqs[1:]:
        st.add(str(a))
    tot = st.total_tt
    linhas = []
    for (f, ln, fn), (cc, nc, tt, ct, _) in st.stats.items():
        linhas.append({"funcao": f"{Path(f).name}:{ln}:{fn}", "tempo_proprio": tt, "tempo_cumulativo": ct, "chamadas": nc})
    proprio = sorted(linhas, key=lambda r: -r["tempo_proprio"])[:top]
    cum = sorted([r for r in linhas if "estocologia" in r["funcao"] or r["funcao"].split(":")[0] in
                  ("funil.py", "icp.py", "amostrador.py", "glm.py", "armazem.py", "estat.py", "dsep.py", "barramento.py")],
                 key=lambda r: -r["tempo_cumulativo"])[:top]
    return {"arquivos": len(arqs), "tempo_total_perfilado": tot, "top_tempo_proprio": proprio, "top_cumulativo_funil": cum}


def resumo_lote(f: Path) -> dict:
    if f.exists():
        L = json.loads(f.read_text())
    else:  # lote interrompido: junta as medidas por domínio que existirem
        ms = [json.loads(m.read_text()) for m in sorted(f.parent.glob("*/medida.json"))]
        if not ms:
            return {}
        L = {"dominios": len(ms), "trabalhadores": None, "segundos_parede_total": None, "medidas": ms, "interrompido": True}
    M = L["medidas"]
    etapa = Counter((m.get("etapa_final") or ("erro: " + m.get("erro", "?").split(":")[0])) for m in M)
    ok = [m for m in M if "segundos_parede" in m and m.get("status") != "erro"]
    seg = np.array([m["segundos_parede"] for m in ok]) if ok else np.array([np.nan])
    rss = np.array([m["rss_pico_kb"] for m in ok]) / 1024 if ok else np.array([np.nan])
    por_etapa = {}
    for e in sorted(set(m.get("etapa_final", "erro") for m in ok)):
        s = np.array([m["segundos_parede"] for m in ok if m.get("etapa_final") == e])
        por_etapa[e] = {"n": int(s.size), "mediana_s": float(np.median(s)), "media_s": float(s.mean()), "max_s": float(s.max())}
    tm = [m["tracemalloc_pico_bytes"] for m in M if "tracemalloc_pico_bytes" in m]
    erros = Counter(m.get("erro", "")[:120] for m in M if m.get("status") == "erro")
    lin = np.array([m["linhas"] for m in ok if "linhas" in m])
    sl = np.array([m["segundos_parede"] for m in ok if "linhas" in m])
    corr = float(np.corrcoef(np.log(lin), np.log(sl))[0, 1]) if lin.size > 3 else None
    return {"dominios": L["dominios"], "trabalhadores": L["trabalhadores"], "segundos_parede_total": L["segundos_parede_total"],
            "etapa_final": dict(etapa.most_common()), "erros": dict(erros.most_common(15)),
            "segundos_por_dominio": {"mediana": float(np.median(seg)), "media": float(np.mean(seg)),
                                     "p90": float(np.quantile(seg, 0.9)), "max": float(np.max(seg)), "soma": float(seg.sum())},
            "segundos_por_etapa": por_etapa,
            "rss_pico_mb": {"mediana": float(np.median(rss)), "max": float(np.max(rss))},
            "tracemalloc_pico_mb": ({"mediana": float(np.median(tm)) / 2**20, "max": float(np.max(tm)) / 2**20} if tm else None),
            "corr_log_linhas_log_segundos": corr,
            "acima_do_prazo_declarado_900s": int(sum(1 for m in M if m.get("segundos_parede", 0) > 900 or m.get("erro") == "prazo")),
            "segundos_cpu_por_dominio": ({"mediana": float(np.median([m["segundos_cpu"] for m in ok])),
                                          "soma": float(np.sum([m["segundos_cpu"] for m in ok]))} if ok else None),
            "interrompido": bool(L.get("interrompido", False))}


def main(dados: Path):
    out = {}
    sel = json.loads((dados / "dominios_auto" / "_selecao.json").read_text())
    out["selecao_R1"] = {"conjuntos_no_catalogo_completo": 3648, "passam_metadados": len(sel),
                         "por_motivo": dict(Counter(s["motivo"].split(" ")[0] for s in sel).most_common())}
    out["lote"] = resumo_lote(dados / "saida_lote" / "_lote.json")
    out["lote_perfil"] = resumo_lote(dados / "saida_perfil" / "_lote.json")
    out["perfil"] = perfil(dados / "saida_perfil")
    base = RAIZ / "saida_base"
    out["grafo_base_h2g"] = montar_grafo(dados, [base], [RAIZ / "dominios"], dados / "hipergrafo_base.json")
    out["grafo_p09"] = montar_grafo(dados, [base, RAIZ / "saida_saude", dados / "saida_lote"],
                                    [RAIZ / "dominios", dados / "dominios_auto"], dados / "hipergrafo_p09.json")
    (RAIZ / "escala" / "analise.json").write_text(json.dumps(out, indent=1, ensure_ascii=False))
    print(json.dumps({k: v for k, v in out.items() if k != "perfil"}, indent=1, ensure_ascii=False))


if __name__ == "__main__":
    main(Path(sys.argv[1]))
