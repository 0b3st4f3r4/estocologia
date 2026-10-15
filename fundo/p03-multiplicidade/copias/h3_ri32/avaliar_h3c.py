"""Portões do H3c (CRITERIOS_H3c.md) para o fluxo prisoes20. Escrito antes da execução completa.

Uso:  python3 avaliar_h3c.py [--reproducao]
Lê saida/prisoes20 (ou $ESTOCOLOGIA_SAIDA) e grava portoes_h3c.json lá e em saida/.
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

import portoes as PT
from estocologia import calibracao as C
from estocologia import governanca as G
from estocologia.barramento import Barramento
from estocologia.motor import sha_arquivo

RAIZ = Path(__file__).resolve().parent
SAIDA = Path(os.environ.get("ESTOCOLOGIA_SAIDA", RAIZ / "saida"))
NOME = "prisoes20"


def carregar(saida: Path) -> dict:
    r = json.loads((saida / NOME / "resultados.json").read_text())
    r["_diario"] = pd.read_parquet(saida / NOME / "diario.parquet")
    return r


def main():
    r = carregar(SAIDA)
    out = {"fluxo": NOME, "criterios": "CRITERIOS_H3c.md", "criterios_sha256": sha_arquivo(RAIZ / "CRITERIOS_H3c.md")}
    # G1
    t = subprocess.run([sys.executable, "-m", "pytest", "-q", "-p", "no:cacheprovider", "tests"], cwd=RAIZ, capture_output=True, text=True)
    g1 = {"pytest": t.stdout.strip().splitlines()[-1] if t.stdout.strip() else t.stderr[-300:], "pytest_ok": t.returncode == 0}
    if "--reproducao" in sys.argv:
        with tempfile.TemporaryDirectory() as tmp:
            subprocess.run([sys.executable, "rodar.py", NOME], cwd=RAIZ, env={**os.environ, "ESTOCOLOGIA_SAIDA": tmp},
                           check=True, capture_output=True)
            r2 = carregar(Path(tmp))
            a = {k: v for k, v in r.items() if k != "K"}
            b = {k: v for k, v in r2.items() if k != "K"}
            g1["reproducao_limpa_identica"] = bool(PT.sem_volateis(a) == PT.sem_volateis(b) and r["_diario"].equals(r2["_diario"]))
    g1["passa"] = bool(g1["pytest_ok"] and g1.get("reproducao_limpa_identica", False))
    out["G1"] = g1
    # G2: oficial e, ao lado, o modelo cru em sombra
    d = PT.cofre(r)
    clima = C.climatologia_movel(r["_diario"])[PT.cofre_mask(r)]
    m = C.binario(d, clima)
    m["passa"] = bool(abs(m["citl"]) <= 0.02 and m["ece"] <= 0.03 and m["habilidade_vs_climatologia"] > 0)
    cru = d.assign(modelo=d["modelo_cru"].astype(float), origem="modelo")
    mc = C.binario(cru, clima)
    out["G2_cofre_2021_S2"] = {"oficial": m, "modelo_cru_sombra": {k: mc[k] for k in ("n", "citl", "ece", "habilidade_vs_climatologia")},
                               "delta_final": float(d["delta"].iloc[-1]) if len(d) else None,
                               "delta_min_max": [float(r["_diario"]["delta"].min()), float(r["_diario"]["delta"].max())]}
    # G3
    PT.SAIDA = SAIDA
    ok3, det3 = PT.g3({NOME: r})
    out["G3"] = {"passa": bool(ok3), **det3}
    # G5
    ok5, det5 = PT.g5({NOME: r})
    out["G5"] = {"passa": bool(ok5), **det5}
    q = Barramento(SAIDA / NOME / "barramento.jsonl").verificar()
    out["trilha"] = {"passa": not q, "detalhe": q or "íntegra"}
    # F6 (informativo): p e y em termos de liberação, como em governanca.f6_trilema (limiar 0,70 ⇔ p(registro) > 0,30)
    dd = r["_diario"][r["_diario"]["y"].notna()].copy()
    po = C.oficial_binario(dd)
    dd = dd.assign(p=1 - po, y=1 - dd["y"].astype(float), ano=dd["ano"].astype(int) * 10 + (dd["trimestre"].astype(int) > 6) + 1)
    out["F6_por_semestre_(ano*10+semestre)"] = G.f6_trilema(dd)
    # F5 (informativo): publicações por cor e trimestre
    det = pd.DataFrame({"year": r["_diario"]["ano"].astype(int) * 10 + (r["_diario"]["trimestre"].astype(int) + 2) // 3,
                        "colour": r["_diario"]["grupo"]})
    ok_y = r["_diario"]["y"].notna()
    out["F5_por_trimestre_(ano*10+trimestre)"] = G.f5_publicacoes(det[ok_y].reset_index(drop=True),
                                                                  r["_diario"]["y"][ok_y].astype(int).reset_index(drop=True),
                                                                  np.random.default_rng(2027))
    out["K"] = r["K"]
    out["contagens"] = r["contagens"]
    out["saida_h3c"] = all(out[k]["passa"] for k in ("G1", "G3", "G5", "trilha")) and m["passa"]
    for alvo in {SAIDA / NOME, RAIZ / "saida"}:
        alvo.mkdir(parents=True, exist_ok=True)
        (alvo / "portoes_h3c.json").write_text(json.dumps(out, indent=1, ensure_ascii=False, default=float))
    print(json.dumps({k: (v.get("passa") if isinstance(v, dict) else v) for k, v in out.items() if not k.startswith("F")}, ensure_ascii=False))
    print(json.dumps(m, ensure_ascii=False, default=float)[:600])
    return 0


if __name__ == "__main__":
    sys.exit(main())
