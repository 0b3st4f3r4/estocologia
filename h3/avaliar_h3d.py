"""Portões do H3d (CRITERIOS_H3d.md) para o fluxo prisoes_ri. Cópia do avaliar_h3c.py, escrita antes da execução
completa: acrescenta o critério relativo (b) e o F6 com o limiar 0,10.

Uso:  python3 avaliar_h3d.py [--reproducao]
Lê saida/prisoes_ri (ou $ESTOCOLOGIA_SAIDA) e grava portoes_h3d.json lá e em saida/.
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
NOME = "prisoes_ri"


def carregar(saida: Path) -> dict:
    r = json.loads((saida / NOME / "resultados.json").read_text())
    r["_diario"] = pd.read_parquet(saida / NOME / "diario.parquet")
    return r


def main():
    r = carregar(SAIDA)
    out = {"fluxo": NOME, "criterios": "CRITERIOS_H3d.md", "criterios_sha256": sha_arquivo(RAIZ / "CRITERIOS_H3d.md")}
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
    m["passa_a_absoluto"] = bool(abs(m["citl"]) <= 0.02 and m["ece"] <= 0.03 and m["habilidade_vs_climatologia"] > 0)
    yb = m["y_medio"]
    m["relativo"] = {"citl_sobre_ybar": abs(m["citl"]) / yb, "ece_sobre_ybar": m["ece"] / yb}
    m["passa_b_relativo"] = bool(abs(m["citl"]) <= 0.2 * yb and m["ece"] <= 0.3 * yb)
    m["passa"] = m["passa_a_absoluto"] and m["passa_b_relativo"]
    cru = d.assign(modelo=d["modelo_cru"].astype(float), origem="modelo")
    mc = C.binario(cru, clima)
    mc["relativo"] = {"citl_sobre_ybar": abs(mc["citl"]) / mc["y_medio"], "ece_sobre_ybar": mc["ece"] / mc["y_medio"]}
    out["G2_cofre_2013_2015"] = {"oficial": m, "modelo_cru_sombra": {k: mc[k] for k in ("n", "citl", "ece", "habilidade_vs_climatologia", "relativo")},
                                 "platt_a_b_final": [float(d["platt_a"].iloc[-1]), float(d["platt_b"].iloc[-1])] if len(d) else None}
    # G3
    PT.SAIDA = SAIDA
    ok3, det3 = PT.g3({NOME: r})
    out["G3"] = {"passa": bool(ok3), **det3}
    # G5
    ok5, det5 = PT.g5({NOME: r})
    out["G5"] = {"passa": bool(ok5), **det5}
    q = Barramento(SAIDA / NOME / "barramento.jsonl").verificar()
    out["trilha"] = {"passa": not q, "detalhe": q or "íntegra"}
    # F6 (informativo): p e y em termos de não prisão, como em governanca.f6_trilema (limiar 0,90 ⇔ p(prisão) > 0,10);
    # só os anos do cofre
    dd = r["_diario"][r["_diario"]["y"].notna() & (r["_diario"]["t_emissao"] >= r["corte"])].copy()
    po = C.oficial_binario(dd)
    dd = dd.assign(p=1 - po, y=1 - dd["y"].astype(float), ano=dd["ano"].astype(int))
    out["F6_por_ano_do_cofre"] = G.f6_trilema(dd, limiar=0.90)
    # F5 (informativo): publicações por cor e ano
    det = pd.DataFrame({"year": r["_diario"]["ano"].astype(int), "colour": r["_diario"]["grupo"]})
    ok_y = r["_diario"]["y"].notna()
    out["F5_por_ano"] = G.f5_publicacoes(det[ok_y].reset_index(drop=True),
                                                                  r["_diario"]["y"][ok_y].astype(int).reset_index(drop=True),
                                                                  np.random.default_rng(2027))
    out["K"] = r["K"]
    out["contagens"] = r["contagens"]
    out["saida_h3d"] = all(out[k]["passa"] for k in ("G1", "G3", "G5", "trilha")) and m["passa"]
    for alvo in {SAIDA / NOME, RAIZ / "saida"}:
        alvo.mkdir(parents=True, exist_ok=True)
        (alvo / "portoes_h3d.json").write_text(json.dumps(out, indent=1, ensure_ascii=False, default=float))
    print(json.dumps({k: (v.get("passa") if isinstance(v, dict) else v) for k, v in out.items() if not k.startswith("F")}, ensure_ascii=False))
    print(json.dumps(m, ensure_ascii=False, default=float)[:600])
    return 0


if __name__ == "__main__":
    sys.exit(main())
