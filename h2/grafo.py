"""Exporta o hipergrafo G = (V, E_tree, E_graph, H) do H2 a partir dos pacotes e dos resultados.

Nós contam conceitos e hipóteses, não registros: cada variável, ambiente, filtro, decisão e cada
subconjunto testado pelo ICP (uma hipótese de invariância) é um nó. As previsões do diário ficam
fora da contagem de nós e são relatadas à parte.

Pontes entre domínios (E_graph): domínios ligados pela mesma patologia ou pelo mesmo achado de lente
(efeito de desenho > 2, ICP todo rejeitado, priori contrariada pelos dados, poda no mesmo filtro).
"""

from __future__ import annotations

import json
import os
from itertools import combinations
from pathlib import Path

import yaml

RAIZ = Path(__file__).resolve().parent
SAIDA = Path(os.environ.get("ESTOCOLOGIA_SAIDA", RAIZ / "saida"))


def main():
    V, Et, Eg, H = [], [], [], []
    marcas: dict[str, list] = {}

    def no(id_, camada, inc, defin, art):
        V.append({"id": id_, "camada": camada, "tipo_incerteza": inc, "definicao_formal": defin, "artefato_simulavel": art})

    no("RAIZ", "raiz", "nao_se_aplica", "Estocologia: julgamento e decisão sob incerteza", "rodar.py")
    for pasta in sorted((RAIZ / "dominios").iterdir()):
        if not (pasta / "pacote.yaml").exists() or not (SAIDA / pasta.name / "resultados.json").exists():
            continue
        p = yaml.safe_load((pasta / "pacote.yaml").read_text())
        r = json.loads((SAIDA / pasta.name / "resultados.json").read_text())
        d = p["nome"]
        art = f"dominios/{d}/pacote.yaml"
        no(f"{d}", "core", "nao_se_aplica", p["titulo"], art)
        Et.append(("RAIZ", d))
        for c, txt in (("L1", "Mundo: " + p["cabecalho"]["processo_gerador"]), ("L2", "Modelo: GLM " + p["familia"] + " com priori SHELF"),
                       ("L3", "Agente: " + p["decisao"]["acao"])):
            no(f"{d}:{c}", c, {"L1": "Aleatoria", "L2": "Epistemica", "L3": "Knightiana_Especificacao"}[c], txt, art)
            Et.append((d, f"{d}:{c}"))
        for v in ["y", *p["candidatos"]]:
            no(f"{d}:{v}", "L1", "Aleatoria", p["desfecho"] if v == "y" else p["candidatos"][v].get("descricao", v), f"dominios/{d}/preparo.py")
            Et.append((f"{d}:L1", f"{d}:{v}"))
        for a, b in p["dag"]:
            Et.append((f"{d}:{a}", f"{d}:{b}"))
        for amb in r["ingestao"]["ambientes"]:
            no(f"{d}:amb={amb}", "L1", "Knightiana_Especificacao", f"ambiente {amb}{' (cofre)' if amb in map(str, p['ambientes']['cofre']) else ''}", "estocologia/armazem.py")
            Et.append((f"{d}:L1", f"{d}:amb={amb}"))
        for k in ("filtro1", "filtro2", "filtro3", "filtro4"):
            if k in r:
                no(f"{d}:{k}", "L2", "Epistemica", f"{k} do funil", "estocologia/funil.py")
                Et.append((f"{d}:L2", f"{d}:{k}"))
        if "decisao" in r:
            no(f"{d}:decisao", "L3", "Knightiana_Especificacao", r["decisao"]["frequencias_naturais"], "estocologia/funil.py::f8")
            Et.append((f"{d}:L3", f"{d}:decisao"))
        f4 = r.get("filtro4")
        if f4:
            for S, pv in f4["icp"]["p_por_subconjunto"].items():
                hid = f"{d}:H[{S}]"
                no(hid, "L2", "Epistemica", f"H0: Y | {S} invariante entre ambientes (p = {pv:.3g})", "estocologia/icp.py")
                Et.append((f"{d}:filtro4", hid))
            for S in f4["icp"]["aceitos"]:
                H.append({"lente": "Morfonomia", "nos": [f"{d}:{v}" for v in S] + [f"{d}:y"], "nota": "conjunto invariante aceito pelo ICP"})
            if f4["icp"]["modelo_rejeitado"]:
                marcas.setdefault("ICP rejeita todos os conjuntos (ambiente age em Y ou pai oculto)", []).append(d)
            if f4["posterior"]["dp_posterior_sobre_dp_priori"] < 0.5 and abs(f4["posterior"]["beta_chave_media"] - r["priori"]["media"]) > 2 * r["priori"]["dp"]:
                marcas.setdefault("dados contrariam a priori da literatura (> 2 dp)", []).append(d)
        H.append({"lente": "Crisonomia", "nos": [f"{d}:L3", f"{d}:filtro1"], "nota": f"EVPI a priori {r['filtro1']['evpi_priori']:.3g}"})
        if "filtro2" in r:
            ed = r["filtro2"].get("efeito_de_desenho")
            if ed and ed > 2:
                marcas.setdefault("pseudorreplicação: efeito de desenho > 2 na unidade independente", []).append(d)
        status = r["status"].split(":")[0]
        marcas.setdefault(f"status: {status}", []).append(d)
    for rot, ds in marcas.items():
        for a, b in combinations(sorted(ds), 2):
            Eg.append({"de": a, "para": b, "tipo": "ponte", "rotulo": rot})
    g = {"nos": V, "arestas_tree": [{"de": a, "para": b} for a, b in Et], "arestas_graph": Eg, "hiperarestas": H,
         "contagem": {"nos": len(V), "arestas_tree": len(Et), "arestas_graph": len(Eg), "hiperarestas": len(H)}}
    (SAIDA / "hipergrafo.json").write_text(json.dumps(g, indent=1, ensure_ascii=False))
    print(g["contagem"])


if __name__ == "__main__":
    main()
