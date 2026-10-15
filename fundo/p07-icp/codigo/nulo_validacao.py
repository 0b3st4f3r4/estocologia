"""Ponto 7 (fundo/p07-icp), Bloco B2: taxa nula da validação estrita da genética.

Implementa ../DECLARACAO_2.md (hash em ../DECLARACAO_2.sha256). Lê os 27 pares (Y, x) e o T observado de
../saida/genetica.json e as colunas do parquet de Kemmeren. Não recalcula o ICP, não reescreve nada do H2g nem a
DECLARACAO.md.

Uso:  OMP_NUM_THREADS=1 python3 nulo_validacao.py
Saída: ../saida/nulo_validacao.json
"""

from __future__ import annotations

import hashlib
import json
import os
import time
from pathlib import Path

import numpy as np
import pandas as pd

RAIZ = Path(__file__).resolve().parent
ARQ = "kemmeren2014_obs_int.parquet"
B = 999
SEMENTE = 20261015
ALPHA = 0.05
SAIDA = Path(os.environ.get("P07_SAIDA", RAIZ.parent / "saida"))


def caminho_parquet() -> Path:
    cands = []
    env = os.environ.get("ESTOCOLOGIA_DADOS")
    if env:
        cands.append(Path(env).parent / "h2f" / ARQ)
    cands.append(RAIZ.parents[2] / "dados-reais" / "h2f" / ARQ)
    for c in cands:
        if c.exists():
            return c
    raise SystemExit("parquet não encontrado em: " + ", ".join(map(str, cands)))


def sha256(p: Path) -> str:
    h = hashlib.sha256()
    with p.open("rb") as f:
        for blk in iter(lambda: f.read(1 << 20), b""):
            h.update(blk)
    return h.hexdigest()


def pares(gen: dict) -> list[dict]:
    """Os 27 pares (Y, x) na ordem de genetica.json: rótulos de resumo.y_com_S_hat_com_gene na ordem de por_y,
    e dentro de cada Y a ordem da lista validacao."""
    com = list(gen["resumo"]["y_com_S_hat_com_gene"])
    out = []
    for r in gen["por_y"]:
        if r["rotulo"] in com:
            for v in r["validacao"]:
                out.append({"rotulo": r["rotulo"], "y": r["y"], "x": v["x"], "gene": v["gene"],
                            "delecao_funcionou_json": bool(v["delecao_funcionou"]),
                            "fora_da_faixa_json": bool(v["fora_da_faixa"]),
                            "estrito_json": bool(v["validacao_estrita"])})
    return out


def criterio(v_x: np.ndarray, v_y: np.ndarray, idx: np.ndarray, imin_x: int, imin_y: int, imax_y: int):
    """Critério estrito nas linhas idx, tirando cada linha do próprio conjunto de comparação.
    Mínimo/máximo são únicos em todas as 27 colunas X e nas 10 colunas Y (conferido na declaração)."""
    xs, ys = np.sort(v_x), np.sort(v_y)
    min_x = np.where(idx == imin_x, xs[1], xs[0])
    min_y = np.where(idx == imin_y, ys[1], ys[0])
    max_y = np.where(idx == imax_y, ys[-2], ys[-1])
    delecao = v_x[idx] <= min_x
    fora = (v_y[idx] < min_y) | (v_y[idx] > max_y)
    return delecao, fora


def principal(d: pd.DataFrame, gen: dict):
    pp = pares(gen)
    n = d.shape[0]
    rng = np.random.default_rng(SEMENTE)
    I = rng.choice(n, size=(B, len(pp)), replace=True)
    dele = np.zeros((B, len(pp)), dtype=bool)
    fora = np.zeros((B, len(pp)), dtype=bool)
    por_par, conferencia = [], {"flags_batem": 0, "min_max_unicos": True, "uma_linha_de_mutante": True}
    for j, p in enumerate(pp):
        v_x = d[p["gene"]].to_numpy(float)
        v_y = d[p["y"]].to_numpy(float)
        imin_x = int(np.argmin(v_x))
        imin_y, imax_y = int(np.argmin(v_y)), int(np.argmax(v_y))
        if (np.sort(v_x)[0] == np.sort(v_x)[1]) or (np.sort(v_y)[0] == np.sort(v_y)[1]) or (np.sort(v_y)[-1] == np.sort(v_y)[-2]):
            conferencia["min_max_unicos"] = False
        linha = np.flatnonzero(d["deleted_gene"].to_numpy() == p["gene"])
        if linha.size != 1:
            conferencia["uma_linha_de_mutante"] = False
            continue
        dl, fr = criterio(v_x, v_y, I[:, j], imin_x, imin_y, imax_y)
        dele[:, j], fora[:, j] = dl, fr
        # observado, recalculado para conferir os flags gravados
        i = int(linha[0])
        dl_o, fr_o = criterio(v_x, v_y, np.array([i]), imin_x, imin_y, imax_y)
        dl_o, fr_o = bool(dl_o[0]), bool(fr_o[0])
        if dl_o == p["delecao_funcionou_json"] and fr_o == p["fora_da_faixa_json"]:
            conferencia["flags_batem"] += 1
        estrito = dl & fr
        por_par.append({
            "rotulo": p["rotulo"], "y": p["y"], "x": p["x"], "gene": p["gene"], "linha_mutante": i,
            "delecao_funcionou": dl_o, "fora_da_faixa": fr_o, "validacao_estrita": bool(dl_o and fr_o),
            "taxa_nula_delecao": float(dl.mean()), "taxa_nula_faixa": float(fr.mean()),
            "taxa_nula_estrito": float(estrito.mean()),
        })
    estrito = dele & fora
    T = estrito.sum(1)
    T_dele = dele.sum(1)
    T_faixa = fora.sum(1)
    T_obs = int(sum(v["estrito_json"] for v in pp))
    obs_dele = int(sum(v["delecao_funcionou_json"] for v in pp))
    obs_faixa = int(sum(v["fora_da_faixa_json"] for v in pp))

    def estat(T_b, obs):
        return {"observado": int(obs), "media_nula": float(T_b.mean()), "desvio_nulo": float(T_b.std(ddof=1)),
                "q50": float(np.quantile(T_b, 0.5)), "q95": float(np.quantile(T_b, 0.95)),
                "q99": float(np.quantile(T_b, 0.99)), "maximo_nulo": int(T_b.max()),
                "n_sorteios_ge_observado": int((T_b >= obs).sum()),
                "p": float((1 + int((T_b >= obs).sum())) / (B + 1))}

    out = {
        "declaracao": "../DECLARACAO_2.md",
        "declaracao_sha256": sha256(RAIZ.parent / "DECLARACAO_2.md"),
        "parquet": {"caminho": str(caminho_parquet()), "sha256": sha256(caminho_parquet())},
        "n_linhas": int(n), "n_pares": len(pp), "B": B, "semente": SEMENTE, "alpha": ALPHA,
        "sorteio": f"np.random.default_rng({SEMENTE}).choice({n}, size=({B}, {len(pp)}), replace=True)",
        "observado": {"T": T_obs, "n_delecao_funcionou": obs_dele, "n_fora_da_faixa": obs_faixa,
                      "genes_validados_estrito": gen["resumo"]["genes_validados_estrito"],
                      "T_de_genetica_json": sum(v["estrito_json"] for v in pp)},
        "nulo_principal": estat(T, T_obs),
        "secundaria_informativa": {"T_delecao": estat(T_dele, obs_dele), "T_faixa": estat(T_faixa, obs_faixa)},
        "conferencia": conferencia,
        "por_par": por_par,
        "segundos": None,
    }
    return out


def main():
    t0 = time.time()
    gen = json.loads((RAIZ.parent / "saida" / "genetica.json").read_text())
    d = pd.read_parquet(caminho_parquet())
    out = principal(d, gen)
    assert out["conferencia"]["flags_batem"] == out["n_pares"], "flags gravados não reproduzem o critério"
    assert out["observado"]["T"] == out["observado"]["T_de_genetica_json"]
    out["segundos"] = round(time.time() - t0, 1)
    SAIDA.mkdir(parents=True, exist_ok=True)
    p = SAIDA / "nulo_validacao.json"
    p.write_text(json.dumps(out, indent=1, ensure_ascii=False, default=float))
    n = out["nulo_principal"]
    print(f"T_obs={n['observado']}  media_nula={n['media_nula']:.6f}  q95_nula={n['q95']:.6f}  "
          f"p={n['p']:.4f}  ({out['segundos']} s) -> {p}")


if __name__ == "__main__":
    main()
