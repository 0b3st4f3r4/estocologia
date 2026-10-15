"""Esteira do H0 em domínio real (PLAN.md §1): roda o subgrafo, verifica Φ1–Φ4 e grava resultados.json.

Uso:  python3 portoes.py        (sai com código 1 se algum portão falhar; leva ~5 min)
"""

from __future__ import annotations

import hashlib
import json
import subprocess
import sys
from pathlib import Path

import numpy as np
import yaml

from estocologia.central import DADOS, META_SL
from estocologia.esquema import validar
from estocologia.h0 import executar

RAIZ = Path(__file__).resolve().parent


def hash_de(r: dict) -> str:
    return hashlib.sha256(json.dumps(r, sort_keys=True, default=float).encode()).hexdigest()


def figura(r: dict, caminho: Path) -> None:
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt

    pd = r["por_dia"]
    fig, ax = plt.subplots(figsize=(7.5, 4.2))
    for chave, rot, cor in (("sl_nulo", f"mundo da alegação, com desistência ({len(pd['sl_nulo'])} dias simulados)", "#7a7a7a"),
                            ("sl_formula_por_dia", "a fórmula da alegação (sem desistência), com a taxa real de cada dia", "#d08c00"),
                            ("sl_real_R0", "chegadas reais, escala da alegação, com desistência", "#b0302a")):
        x = np.sort(pd[chave])
        ax.step(x, np.arange(1, x.size + 1) / x.size, where="post", label=rot, color=cor, lw=2)
    ax.axvline(META_SL, ls="--", color="k", lw=1)
    ax.text(META_SL, 0.02, " meta 80%", fontsize=9)
    ax.set_xlabel("nível de serviço do dia (fração atendida em até 20 s)")
    ax.set_ylabel("fração dos dias do cofre (jul–out 2003)")
    ax.set_xlim(0.15, 1.0)
    ax.legend(loc="upper left", fontsize=8, frameon=False)
    ax.set_title("Mesma escala, três mundos: a unidade é o dia, não a chamada", fontsize=10)
    fig.tight_layout()
    fig.savefig(caminho, dpi=130)
    plt.close(fig)


def main() -> int:
    portoes: dict[str, tuple[bool, str]] = {}

    # Φ1 · Simulabilidade: testes de propriedades + reprodução determinística (duas execuções rápidas)
    t = subprocess.run([sys.executable, "-m", "pytest", "-q", "tests"], cwd=RAIZ, capture_output=True, text=True)
    h1, h2 = hash_de(executar(rapido=True)), hash_de(executar(rapido=True))
    ok = t.returncode == 0 and h1 == h2
    portoes["Φ1 simulabilidade"] = (ok, f"pytest: {t.stdout.strip().splitlines()[-1]}; hash igual nas 2 execuções: {h1 == h2}")

    r = executar()

    # Φ2 · Margem na unidade independente; o nulo arbitra qual margem é a honesta
    real = r["mundos"]["com_desistencia"]["real"]
    ts = r["teste_severo"]
    razao = real["ep_dia"] / real["ep_chamada"]
    ok = razao > 3 and abs(ts["nulo_dp_metades"] / ts["ep_dia_nulo_previsto"] - 1) < 0.2 and ts["nulo_dp_metades"] > 2 * ts["ep_chamada_nulo_previsto"]
    portoes["Φ2 margem e unidade"] = (
        ok,
        f"ep por dia {real['ep_dia']:.3f} s vs por chamada {real['ep_chamada']:.4f} s (×{razao:.0f}); "
        f"dp do nulo {ts['nulo_dp_metades']:.4f}, previsto pelo dia {ts['ep_dia_nulo_previsto']:.4f}, pela chamada {ts['ep_chamada_nulo_previsto']:.4f}",
    )

    # Φ3 · Diagnóstico por camada e teste severo contra o modelo nulo
    ficha = yaml.safe_load((RAIZ / "ficha.yaml").read_text())
    erros = validar(ficha, RAIZ)
    ok = not erros and ts["p_frac"] < 0.001 and ts["z"] > 5
    portoes["Φ3 camadas e teste severo"] = (
        ok,
        f"ficha: {'válida' if not erros else erros}; dias abaixo da meta: {ts['frac_dias_abaixo_meta_real']:.0%} real contra "
        f"{ts['frac_dias_abaixo_meta_nulo']:.0%} no nulo (q99 do nulo {ts['nulo_frac_q99']:.0%}, p < {1 / 4000:.4f}); "
        f"espera por dia {ts['diferenca_espera_dia_s']:+.1f} s = {ts['z']:.0f} ep",
    )

    # Φ4 · Escopo ecológico: termo, regra, frequências naturais, circuit breaker, e a regra melhora a cauda
    termo = ficha.get("termo_de_escopo", {})
    faltam = [k for k in ("vale_para", "nao_vale_para", "regra_de_decisao", "frequencias_naturais", "circuit_breaker") if not termo.get(k)]
    rg = r["regras"]
    r0, r2 = rg["R0_alegacao_perfil_medio"], rg["R2_dia_da_semana_mais_margem_p90"]
    ok = not faltam and r2["frac_dias_abaixo_meta"] < r0["frac_dias_abaixo_meta"] / 2
    portoes["Φ4 escopo ecológico"] = (
        ok,
        f"campos faltando: {faltam or 'nenhum'}; regra R2: {r2['frac_dias_abaixo_meta']:.0%} dos dias abaixo da meta contra "
        f"{r0['frac_dias_abaixo_meta']:.0%} (custo {r2['agente_horas_dia'] / r0['agente_horas_dia'] - 1:+.1%} em agente-horas)",
    )

    # Conferência com a base: hash do arquivo e forma
    soma = hashlib.sha256(DADOS.read_bytes()).hexdigest()
    esperado = {l.split()[1]: l.split()[0] for l in (DADOS.parent / "SHA256SUMS").read_text().splitlines()}[DADOS.name]
    ok = soma == esperado and r["dias_treino"] + r["dias_cofre"] == 164
    portoes["Conferência com a base real"] = (ok, f"sha256 {'confere' if soma == esperado else 'NÃO confere'}; {r['dias_treino']} dias de treino + {r['dias_cofre']} no cofre")

    figura(r, RAIZ / "figura_dias.png")
    saida = {"resultados": r, "hash": hash_de(r), "portoes": {k: {"ok": o, "detalhe": d} for k, (o, d) in portoes.items()}}
    (RAIZ / "resultados.json").write_text(json.dumps(saida, indent=1, ensure_ascii=False, default=float))
    for k, (o, d) in portoes.items():
        print(f"{'PASSA' if o else 'FALHA'}  {k}: {d}")
    return 0 if all(o for o, _ in portoes.values()) else 1


if __name__ == "__main__":
    sys.exit(main())
