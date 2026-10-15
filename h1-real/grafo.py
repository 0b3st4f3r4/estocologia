"""Exporta o hipergrafo tipado G = (V, E_tree, E_graph, H, Φ) a partir do barramento e dos resultados.

Nós: raízes, núcleo, as 3 camadas, os módulos M0–M15 e as faixas F1–F8 (o esqueleto), e uma
instância por ficha publicada (as hipóteses, contratos, diagnósticos e decisões de cada projeto).
Arestas: E_tree (dependência ficha ← ficha lida, e ficha → faixa/módulo que a produziu) e
E_graph (patologias e pontes declaradas pelos projetos). Hiperarestas: as 5 lentes, cada uma
envolvendo as fichas que leu. Grava saida/hipergrafo.json.
"""

from __future__ import annotations

import json
import os
from pathlib import Path

RAIZ = Path(__file__).resolve().parent
SAIDA = Path(os.environ.get("ESTOCOLOGIA_SAIDA", RAIZ / "saida"))

FAIXA_DE = {"agent-f1-code": "F1", "agent-f2-data": "F2", "agent-f3-lifecycle": "F3", "agent-f4-frontier": "F4",
            "agent-f5-governance": "F5", "agent-f6-fairness": "F6", "agent-f7-domain": "F7", "agent-f8-decision": "F8",
            "agent-m15-orchestrator": "M15"}
CAMADA_DE_TIPO = {"ficha_transferencia": "L3", "shelf_v4": "L2", "termo_escopo": "L3", "datacontract": "L1", "ingestao": "L1",
                  "dossie_governanca": "L3", "auditoria_justica": "L3", "portao_crisonomico": "L3", "processo_gerador_e_nulo": "L2",
                  "fronteira": "L2", "ciclo_de_vida": "L2", "lentes": "L2", "banca_e_critica": "L2", "decisao_executiva": "L3"}
INCERTEZA_DE_TIPO = {"datacontract": "Aleatoria", "ingestao": "Aleatoria", "fronteira": "Epistemica", "processo_gerador_e_nulo": "Epistemica",
                     "shelf_v4": "Epistemica", "banca_e_critica": "Knightiana_Especificacao", "termo_escopo": "Knightiana_Especificacao"}


def main():
    fichas = [json.loads(l) for l in (SAIDA / "barramento.jsonl").read_text().splitlines()]
    V, Et, Eg, H = [], [], [], []
    V += [{"id": f"N_Raiz{i}", "camada": "raiz", "tipo": "raiz"} for i in (1, 2, 3)]
    V.append({"id": "N_Core", "camada": "core", "tipo": "processo gerador oculto"})
    V += [{"id": c, "camada": c, "tipo": "camada"} for c in ("L1", "L2", "L3")]
    V += [{"id": f"N_C{i}", "camada": "conceito", "tipo": "conceito"} for i in range(1, 11)]
    V += [{"id": f"M{i}", "camada": "modulo", "tipo": "modulo"} for i in range(16)]
    V += [{"id": f"F{i}", "camada": "faixa", "tipo": "faixa"} for i in range(1, 9)]
    Et += [{"de": f"M{i}", "para": f"M{i + 1}", "tipo": "prerequisito"} for i in range(15)]
    por_hash = {}
    for f in fichas:
        nid = f"{f['projeto']}:{f['tipo']}:{f['hash'][:8]}"
        por_hash[f["hash"]] = nid
        V.append({"id": nid, "camada": CAMADA_DE_TIPO.get(f["tipo"], "L2"), "tipo_incerteza": INCERTEZA_DE_TIPO.get(f["tipo"], "nao_se_aplica"),
                  "tipo": f["tipo"], "projeto": f["projeto"], "estagio": f["estagio"], "artefato_simulavel": "saida/barramento.jsonl#" + f["hash"]})
        Et.append({"de": nid, "para": FAIXA_DE[f["agente"]], "tipo": "produzido_por"})
        for p in f["pais"]:
            if p in por_hash:
                Et.append({"de": por_hash[p], "para": nid, "tipo": "lido_por"})
    for res in sorted(SAIDA.glob("*/resultados.json")):
        r = json.loads(res.read_text())
        proj = res.parent.name
        nos_proj = [v["id"] for v in V if v.get("projeto") == proj]
        for lente in (r.get("lentes") or {}):
            H.append({"lente": lente, "projeto": proj, "nos": nos_proj})
        Eg.append({"de": f"{proj}:alegacao", "para": "N_Core", "tipo": "patologia", "rotulo": "desalinhamento intercamadas apontado pela banca"})
        V.append({"id": f"{proj}:alegacao", "camada": "L3", "tipo": "hipotese", "projeto": proj})
    Phi = ["simulabilidade", "margem_e_unidade", "camadas_e_teste_severo", "escopo_ecologico"]
    G = {"V": V, "E_tree": Et, "E_graph": Eg, "H": H, "Phi": Phi,
         "contagem": {"nos": len(V), "arestas_tree": len(Et), "arestas_graph": len(Eg), "hiperarestas": len(H)}}
    (SAIDA / "hipergrafo.json").write_text(json.dumps(G, indent=1, ensure_ascii=False))
    print(G["contagem"])


if __name__ == "__main__":
    main()
