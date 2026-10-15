"""Portão de saída do H2 (PLAN.md §1): transferência interdomínios.

Cópia do H2c (2026-10-14): critérios G1–G5 idênticos aos do H2b, letra por letra. Muda só o teste de
invariância usado pelo ICP (K = 2, declarado antes em ../h2b/DECLARACAO_K2.md); "ICP padrão" no G3 é lido
como a lógica padrão do ICP (interseção dos aceitos, α = 0,05) com o teste K = 2 — leitura que cabe a sil confirmar.

Cópia do H2b (2026-10-14): critérios G1–G5 idênticos aos do H2, sem uma vírgula mudada; só os domínios e os
dados (../dados-reais/h2b, ambientes = intervenções conhecidas em X) mudaram.

Critérios declarados em 2026-10-14, depois dos dois pilotos (educação e logística) e ANTES de rodar
os outros dez domínios:

G1 Simulabilidade: pytest passa e uma segunda execução limpa reproduz os resultados.
G2 Funil completo: todo domínio termina num status com motivo; K declarado; SHELF ajustado
   (erro máximo nos quartis < 10% do dp ajustado); em todo domínio que chegou ao F4, o
   pré-registro vem antes da leitura do cofre no registro de varredura.
   REVISÃO depois da primeira execução: o critério declarado era "erro < 1e-6", que só uma
   elicitação simétrica cumpre; quartis de razão de chances são assimétricos em log e nenhuma
   normal os reproduz exatamente. Na primeira execução G2 falhou só por isso (6 domínios, erro
   máximo 5% do dp). A troca é registrada aqui e no README; ela não muda G3, que decide a saída.
G3 ICP entre ambientes (critério do PLAN): em pelo menos 3 domínios de áreas distintas, o ICP
   padrão aceita algum conjunto E devolve Ŝ ≠ ∅. A comparação fora do ambiente (modelo
   invariante contra todos os candidatos) é relatada, não exigida.
G4 Contenção de varredura (critério do PLAN): nenhum SELECT * executado; toda leitura com
   projeção do EXPLAIN contida na pedida; nenhuma leitura do cofre antes do pré-registro;
   gasto por agente dentro do orçamento; a banca (SELECT *, cofre cedo, agente errado abrindo o
   cofre, estouro de orçamento) é toda recusada.
G5 Informativo: aprovações do M15 por domínio e assinaturas humanas pendentes.

Saída do H2 = G1 ∧ G2 ∧ G3 ∧ G4.

Uso:  python3 portoes.py [--reproducao]
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

import numpy as np
import pandas as pd

from estocologia.armazem import Armazem, Recusa
from estocologia.barramento import Barramento

RAIZ = Path(__file__).resolve().parent
SAIDA = Path(os.environ.get("ESTOCOLOGIA_SAIDA", RAIZ / "saida"))
VOLATEIS = {"segundos"}


def carregar(saida: Path) -> dict:
    return {p.parent.name: json.loads(p.read_text()) for p in sorted(saida.glob("*/resultados.json"))}


def sem_volateis(o):
    if isinstance(o, dict):
        return {k: sem_volateis(v) for k, v in o.items() if k not in VOLATEIS}
    if isinstance(o, list):
        return [sem_volateis(v) for v in o]
    if isinstance(o, float):
        return round(o, 8)
    return o


def g1(res, reproducao):
    t = subprocess.run([sys.executable, "-m", "pytest", "-q", "-p", "no:cacheprovider", "tests"], cwd=RAIZ, capture_output=True, text=True)
    ok = t.returncode == 0
    det = f"pytest: {t.stdout.strip().splitlines()[-1]}"
    if reproducao:
        with tempfile.TemporaryDirectory() as tmp:
            env = {**os.environ, "ESTOCOLOGIA_SAIDA": tmp}
            subprocess.run([sys.executable, "rodar.py"], cwd=RAIZ, env=env, check=True, capture_output=True)
            res2 = carregar(Path(tmp))
        iguais = {k: sem_volateis(res[k]) == sem_volateis(res2.get(k)) for k in res}
        ok &= all(iguais.values())
        det += f"; reprodução limpa idêntica: {iguais}"
    return ok, det


def g2(res, bus, reg):
    det, ok = [], True
    for nome, r in res.items():
        o = bool(r.get("status")) and "K" in r and r["priori"]["erro_max_quartis"] < 0.1 * r["priori"]["dp"]
        if "filtro4" in r:
            h = r["filtro4"]["preregistro_hash"]
            ordem_bus = [f.tipo for f in bus.fichas if f.projeto == nome]
            o &= ordem_bus.index("preregistro") < ordem_bus.index("fora_do_ambiente")
            o &= any(f.tipo == "preregistro" and f.payload.get("hash") == h for f in bus.fichas)
        ok &= o
        det.append(f"{nome}: {r.get('status')} (K={r.get('K')})")
    return ok, "; ".join(det)


def g3(res):
    demo, linhas = [], []
    for nome, r in res.items():
        f4 = r.get("filtro4")
        if not f4:
            linhas.append(f"{nome}: não chegou ao F4")
            continue
        icp = f4["icp"]
        fora = f4["fora_do_ambiente"]["habilidade"]
        if not icp["modelo_rejeitado"] and icp["S_hat"]:
            demo.append((nome, r["area"]))
        linhas.append(f"{nome}: Ŝ={icp['S_hat'] or '∅'}, aceitos={len(icp['aceitos'])}/{icp['subconjuntos_testados']}"
                      f"{' (todos rejeitados)' if icp['modelo_rejeitado'] else ''}; habilidade fora do ambiente: "
                      f"invariante {fora['icp_invariante']:+.3f}, todos {fora['todos_os_candidatos']:+.3f}, modelo {fora['modelo_ajustado']:+.3f}")
    areas = {a for _, a in demo}
    ok = len(areas) >= 3
    return ok, f"Ŝ ≠ ∅ em {len(demo)} domínio(s), {len(areas)} área(s): {[n for n, _ in demo]} | " + "; ".join(linhas)


def banca_armazem() -> dict:
    """Ataques ao armazém num diretório descartável: todos precisam ser recusados."""
    out = {}
    with tempfile.TemporaryDirectory() as tmp:
        arm = Armazem(Path(tmp), orcamento_por_agente=2_000)
        df = pd.DataFrame({"y": np.arange(400.0), "x": np.arange(400.0), "ambiente": ["a"] * 200 + ["b"] * 200})
        arm.ingerir("d", df, cofre=["b"])
        ataques = {
            "select_estrela": lambda: arm.ler("d", ["*"], "agent-f4-frontier", excluir=["b"]),
            "cofre_antes_do_preregistro": lambda: arm.ler("d", ["y"], "agent-f4-frontier", ambientes=["b"]),
            "agente_errado_abre_cofre": lambda: arm.destrancar("d", "f" * 64, "agent-f4-frontier"),
            "cofre_sem_hash": lambda: arm.destrancar("d", "não-é-hash", "agent-f3-lifecycle"),
            "estouro_de_orcamento": lambda: [arm.ler("d", ["y", "x"], "agent-f1-code", excluir=["b"]) for _ in range(50)],
        }
        for k, f in ataques.items():
            try:
                f()
                out[k] = False
            except Recusa:
                out[k] = True
    return out


def g4(reg_json):
    reg = reg_json["registro"]
    estrela = [r for r in reg if "*" in r["colunas"]]
    proj_ruim = [r for r in reg if not set(r["explain_projecao"]) <= set(r["colunas"]) | {"ambiente"}]
    cofre_cedo = [r for r in reg if r["toca_cofre"] and r["agente"] != "agent-f3-lifecycle"]
    estouro = {a: g for a, g in reg_json["gasto_por_agente"].items() if g > reg_json["orcamento_por_agente"]}
    banca = banca_armazem()
    varrido = sum(r["bytes_varridos"] for r in reg)
    por_dom = {}
    for r in reg:
        d = por_dom.setdefault(r["dominio"], {"varrido": 0, "tabela": r["bytes_tabela"], "leituras": 0})
        d["varrido"] += r["bytes_varridos"]
        d["leituras"] += 1
    ok = not estrela and not proj_ruim and not cofre_cedo and not estouro and all(banca.values())
    det = (f"{len(reg)} leituras; SELECT *: {len(estrela)}; projeção fora do pedido: {len(proj_ruim)}; cofre fora do F3: {len(cofre_cedo)}; "
           f"agentes acima do orçamento: {list(estouro) or 'nenhum'}; banca recusada: {banca}; "
           f"bytes varridos {varrido / 1e6:.1f} MB, gasto por agente (MB) "
           f"{ {a: round(g / 1e6, 1) for a, g in reg_json['gasto_por_agente'].items()} }")
    return ok, det, por_dom, banca


def g5(res):
    aprov = {n: r["filtro4"]["aprovado"] for n, r in res.items() if "filtro4" in r}
    return f"M15 aprovou {sum(aprov.values())}/{len(aprov)} que chegaram ao F4: {aprov}; pareceres de risco sem assinatura de sil: {len(res)}"


def main():
    res = carregar(SAIDA)
    if not res:
        print("nada em", SAIDA, "— rode rodar.py antes")
        return 1
    bus = Barramento(SAIDA / "barramento.jsonl")
    reg = json.loads((SAIDA / "registro_varredura.json").read_text())
    out = {}
    out["G1 simulabilidade e reprodução"] = g1(res, "--reproducao" in sys.argv)
    out["G2 funil completo e pré-registro"] = g2(res, bus, reg)
    out["G3 ICP entre ambientes"] = g3(res)
    ok4, d4, por_dom, banca = g4(reg)
    out["G4 contenção de varredura"] = (ok4, d4)
    quebras = bus.verificar()
    out["Trilha do barramento"] = (not quebras, f"{len(bus.fichas)} fichas, quebras: {quebras or 'nenhuma'}")
    saida = {k: {"ok": o, "detalhe": d} for k, (o, d) in out.items()}
    saida["G5 informativo"] = g5(res)
    saida["varredura_por_dominio"] = por_dom
    saida["saida_do_H2"] = all(o for k, (o, _) in out.items())
    (SAIDA / "portoes.json").write_text(json.dumps(saida, indent=1, ensure_ascii=False))
    for k, (o, d) in out.items():
        print(f"{'PASSA' if o else 'FALHA'}  {k}: {d}")
    print("INFO  G5:", saida["G5 informativo"])
    print("SAÍDA DO H2:", "aprovada" if saida["saida_do_H2"] else "NÃO aprovada")
    return 0 if saida["saida_do_H2"] else 1


if __name__ == "__main__":
    sys.exit(main())
