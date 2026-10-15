"""Esteira do H0 (PLAN.md §1): roda o subgrafo, verifica Φ1–Φ4 e grava resultados.json.

Uso:  python3 portoes.py        (sai com código 1 se algum portão falhar)
"""

from __future__ import annotations

import hashlib
import json
import subprocess
import sys
from pathlib import Path

import yaml

from estocologia.esquema import validar
from estocologia.lab0 import executar

RAIZ = Path(__file__).resolve().parent

# Valores publicados na Fundamentação e no roteiro do M0 (arredondados como lá aparecem)
REFERENCIA = {
    ("alegacao", "espera_media"): (9.3, 0.05),
    ("variavel", "espera_media"): (20.7, 0.05),
    ("alegacao", "frac_acima_30"): (0.080, 0.0005),
    ("variavel", "frac_acima_30"): (0.254, 0.0005),
    ("variavel", "ep_dia"): (0.5, 0.05),
    ("variavel", "ep_paciente"): (0.07, 0.005),
}


def hash_de(r: dict) -> str:
    return hashlib.sha256(json.dumps(r, sort_keys=True).encode()).hexdigest()


def main() -> int:
    portoes: dict[str, tuple[bool, str]] = {}

    # Φ1 · Simulabilidade: testes de propriedades + reprodução determinística (duas execuções)
    t = subprocess.run([sys.executable, "-m", "pytest", "-q", "tests"], cwd=RAIZ, capture_output=True, text=True)
    r1, r2 = executar(), executar()
    h1, h2 = hash_de(r1), hash_de(r2)
    ok = t.returncode == 0 and h1 == h2
    portoes["Φ1 simulabilidade"] = (ok, f"pytest: {t.stdout.strip().splitlines()[-1]}; hash igual nas 2 execuções: {h1 == h2}")

    # Φ2 · Margem na unidade independente; o próprio nulo arbitra qual margem é a honesta
    v, a, ts = r1["variavel"], r1["alegacao"], r1["teste_severo"]
    razao = v["ep_dia"] / v["ep_paciente"]
    dp_prev_dia = (2 ** 0.5) * a["ep_dia"]
    dp_prev_pac = (2 ** 0.5) * a["ep_paciente"]
    ok = razao > 3 and abs(ts["nulo_dp"] / dp_prev_dia - 1) < 0.2 and ts["nulo_dp"] > 2 * dp_prev_pac
    portoes["Φ2 margem e unidade"] = (
        ok,
        f"ep por dia {v['ep_dia']:.2f} vs por paciente {v['ep_paciente']:.2f} (×{razao:.1f}); "
        f"dp do nulo {ts['nulo_dp']:.3f}, previsto pelo dia {dp_prev_dia:.3f}, pelo paciente {dp_prev_pac:.3f}",
    )

    # Φ3 · Diagnóstico por camada e teste severo contra o modelo nulo
    ficha = yaml.safe_load((RAIZ / "ficha.yaml").read_text())
    erros = validar(ficha, RAIZ)
    z = ts["diferenca_observada"] / ts["nulo_dp"]
    ok = not erros and ts["frac_nulo_ge_obs"] == 0 and z > 5
    portoes["Φ3 camadas e teste severo"] = (
        ok,
        f"ficha: {'válida' if not erros else erros}; diferença {ts['diferenca_observada']:.1f} min = {z:.0f} dp do nulo; "
        f"nulo nunca chegou lá em {ts['reps']} repartições (máx |dif| {ts['nulo_max_abs']:.2f})",
    )

    # Φ4 · Escopo ecológico: termo, regra, frequências naturais e circuit breaker declarados
    termo = ficha.get("termo_de_escopo", {})
    faltam = [k for k in ("vale_para", "nao_vale_para", "regra_de_decisao", "frequencias_naturais", "circuit_breaker") if not termo.get(k)]
    reforco = r1["mundos"]["5 ou 11, reforço no surto"]
    ok = not faltam and reforco[0] < a["espera_media"]
    portoes["Φ4 escopo ecológico"] = (ok, f"campos faltando: {faltam or 'nenhum'}; reforço no surto: {reforco[0]:.1f} min, {reforco[1]:.1%} > 30 min")

    # Conferência com os números publicados
    conf = {}
    for (grupo, k), (ref, tol) in REFERENCIA.items():
        conf[f"{grupo}.{k}"] = {"obtido": r1[grupo][k], "publicado": ref, "ok": abs(r1[grupo][k] - ref) <= tol}
    portoes["Conferência com a Fundamentação"] = (all(c["ok"] for c in conf.values()), f"{sum(c['ok'] for c in conf.values())}/{len(conf)} números batem")

    saida = {"resultados": r1, "hash": h1, "conferencia": conf, "portoes": {k: {"ok": o, "detalhe": d} for k, (o, d) in portoes.items()}}
    (RAIZ / "resultados.json").write_text(json.dumps(saida, indent=1, ensure_ascii=False))
    for k, (o, d) in portoes.items():
        print(f"{'PASSA' if o else 'FALHA'}  {k}: {d}")
    return 0 if all(o for o, _ in portoes.values()) else 1


if __name__ == "__main__":
    sys.exit(main())
