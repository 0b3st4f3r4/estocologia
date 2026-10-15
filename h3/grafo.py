"""Exporta o hipergrafo G = (V, E_tree, E_graph, H) do H3.

Na malha contínua, cada previsão emitida é um nó da camada L3 (Agente). Ela tem tipo de incerteza,
definição formal (o evento previsto, o instante e a origem) e artefato simulável (o fluxo e o motor
que a reproduzem). Os nós de topologia leve vêm dos contratos: fluxo, camadas, tabelas, colunas,
sensores, disjuntor, decisão, escopo, modelo e reserva.

PLAN §5.1: "o banco de hipergrafos guarda apenas a topologia leve". Por isso os nós e as arestas
vão para Parquet, não para JSON, e só o resumo das contagens fica em JSON.

Pontes (E_graph): fluxos ligados pela mesma patologia operacional. São elas:
- disjuntor aberto no cofre;
- alarme de deriva no cofre;
- dados com titulares;
- ação que pode influir no mundo.
"""

from __future__ import annotations

import importlib
import json
import os
from itertools import combinations
from pathlib import Path

import pandas as pd

from rodar import FLUXOS

RAIZ = Path(__file__).resolve().parent
SAIDA = Path(os.environ.get("ESTOCOLOGIA_SAIDA", RAIZ / "saida"))


def main():
    V, Et, Eg, H = [], [], [], []

    def no(i, camada, inc, defin, art):
        V.append((i, camada, inc, defin, art))

    no("RAIZ", "raiz", "nao_se_aplica", "Estocologia: julgamento e decisão sob incerteza (malha contínua, H3)", "rodar.py")
    marcas: dict[str, list] = {}
    for nome, (mod, cls) in FLUXOS.items():
        p = SAIDA / nome / "resultados.json"
        if not p.exists():
            continue
        r = json.loads(p.read_text())
        f = getattr(importlib.import_module(mod), cls)
        art = f"fluxos/{nome}.py"
        no(nome, "core", "nao_se_aplica", f.contrato["pergunta"], art)
        Et.append(("RAIZ", nome))
        for c, txt, inc in (("L1", "Mundo: " + f.cabecalho["processo_gerador"], "Aleatoria"),
                            ("L2", "Modelo: " + (f.__doc__ or "").split("Modelo (F4):")[-1].split("\n\n")[0].strip()[:300], "Epistemica"),
                            ("L3", "Agente: " + f.contrato["decisao"]["acao"], "Knightiana_Especificacao")):
            no(f"{nome}:{c}", c, inc, txt, art)
            Et.append((nome, f"{nome}:{c}"))
        for tab, info in r["ingestao"].items():
            no(f"{nome}:tabela:{tab}", "L1", "Aleatoria", f"tabela de eventos {tab}: {info['linhas']} linhas, disponível em t_disp", "estocologia/armazem.py")
            Et.append((f"{nome}:L1", f"{nome}:tabela:{tab}"))
            for col in info["colunas"]:
                no(f"{nome}:{tab}.{col}", "L1", "Aleatoria", f"coluna {col} de {tab}", "estocologia/armazem.py")
                Et.append((f"{nome}:tabela:{tab}", f"{nome}:{tab}.{col}"))
        for comp, camada, inc, txt, a in (("modelo", "L2", "Epistemica", "previsor do fluxo", art),
                                         ("reserva", "L2", "Epistemica", "reserva determinístico (F1)", art),
                                         ("disjuntor", "L3", "Knightiana_Especificacao", f"disjuntor F8 {f.disjuntor}", "estocologia/monitor.py"),
                                         ("escopo", "L3", "Knightiana_Especificacao", f.contrato["escopo"], art),
                                         ("decisao", "L3", "Knightiana_Especificacao", f.contrato["decisao"]["regra"], art)):
            no(f"{nome}:{comp}", camada, inc, txt, a)
            Et.append((f"{nome}:{camada}", f"{nome}:{comp}"))
        for s in f.sensores:
            no(f"{nome}:sensor:{s}", "L2", "Epistemica", f"sensor de deriva KS/Wasserstein {s}", "estocologia/monitor.py")
            Et.append((f"{nome}:L2", f"{nome}:sensor:{s}"))
        for k, e in enumerate(r["eventos_operacao"]):
            eid = f"{nome}:evento:{k}"
            no(eid, "L3", "Knightiana_Especificacao", f"{e['tipo']} {e['evento']} em t={e['t']}", "estocologia/motor.py")
            Et.append((f"{nome}:L3", eid))
            if e["tipo"] == "deriva":
                H.append({"lente": "Crononomia", "nos": [eid, f"{nome}:sensor:{e['sensor']}", f"{nome}:modelo"], "nota": "alarme de deriva → reajuste"})
                if e["t"] >= r["corte"] and e["evento"] == "ligou":
                    marcas.setdefault("alarme de deriva no cofre", []).append(nome)
            else:
                H.append({"lente": "Crisonomia", "nos": [eid, f"{nome}:disjuntor", f"{nome}:modelo", f"{nome}:reserva", f"{nome}:decisao"],
                          "nota": "disjuntor: política de ignorância"})
                if e["t"] >= r["corte"] and e["evento"] == "abriu":
                    marcas.setdefault("disjuntor aberto no cofre", []).append(nome)
        if f.contrato.get("tem_titulares"):
            marcas.setdefault("dados com titulares (F5/F6)", []).append(nome)
        if f.contrato.get("pode_influir"):
            marcas.setdefault("ação pode influir no mundo (M11)", []).append(nome)
        d = pd.read_parquet(SAIDA / nome / "diario.parquet", columns=["id", "t_emissao", "t_alvo", "origem"])
        V.extend(zip(nome + ":prev:" + d["id"].astype(str), ["L3"] * len(d), ["Aleatoria"] * len(d),
                     "previsão de " + nome + " emitida em " + d["t_emissao"].astype(str) + " para " + d["t_alvo"].astype(str) + " (" + d["origem"] + ")",
                     [art] * len(d)))
        Et.extend(zip([f"{nome}:decisao"] * len(d), nome + ":prev:" + d["id"].astype(str)))
    for rot, ds in marcas.items():
        for a, b in combinations(sorted(set(ds)), 2):
            Eg.append({"de": a, "para": b, "tipo": "ponte", "rotulo": rot})
    nos = pd.DataFrame(V, columns=["id", "camada", "tipo_incerteza", "definicao_formal", "artefato_simulavel"])
    assert nos["id"].is_unique
    nos.to_parquet(SAIDA / "hipergrafo_nos.parquet", index=False)
    pd.DataFrame(Et, columns=["de", "para"]).to_parquet(SAIDA / "hipergrafo_arestas_tree.parquet", index=False)
    resumo = {"nos": int(len(nos)), "nos_previsao": int((nos["id"].str.contains(":prev:")).sum()),
              "nos_topologia": int((~nos["id"].str.contains(":prev:")).sum()), "arestas_tree": len(Et),
              "arestas_graph": len(Eg), "hiperarestas": len(H), "meta_do_PLAN": 10**6, "pontes": Eg,
              "hiperarestas_amostra": H[:20]}
    (SAIDA / "hipergrafo_resumo.json").write_text(json.dumps(resumo, indent=1, ensure_ascii=False))
    print({k: v for k, v in resumo.items() if k not in ("pontes", "hiperarestas_amostra")})


if __name__ == "__main__":
    main()
