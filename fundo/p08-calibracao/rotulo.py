"""(b) Testes de deriva de rótulo sobre os diários do H3 (DECLARACAO.md, item b).

R1: CITL móvel. A cada m resoluções (ordem de t_rotulo), sobre as W mais recentes: d = ȳ − p̄, z = (Σy − Σp)/√Σp(1−p).
    Liga com |d| > 0,02 e |z| > 3 em 3 verificações seguidas; desliga com |d| < 0,01 em 3 seguidas.
R2: BBSE (Lipton, Wang e Smola 2018). A cada m emissões, no instante t: fonte = W previsões mais recentes com
    t_rotulo ≤ t; alvo = W emissões mais recentes. ŷ = 1{p > π_fonte}; q̂ = (μ_alvo − FPR)/(TPR − FPR) em [0, 1];
    d_B = q̂ − p̄_alvo. Liga com |d_B| > 0,02 em 3 seguidas; desliga com |d_B| < 0,01 em 3 seguidas.
"""
from __future__ import annotations

import json
import sys

import numpy as np
import pandas as pd

from comum import AQUI, FLUXOS, C, diario, mes, resultados

# janela e intervalo do disjuntor de cada fluxo (h3/fluxos/*.py)
PARAM = {"voos": (3000, 300), "voos23": (3000, 300), "ciclones": (1500, 100),
         "prisoes": (400, 50), "prisoes20": (400, 50), "prisoes_ri": (400, 50)}
ON, OFF, Z, K = 0.02, 0.01, 3.0, 3


class Histerese:
    def __init__(self):
        self.ligado, self.c_on, self.c_off, self.eventos = False, 0, 0, []

    def passo(self, t, cond_on, cond_off, info):
        if not self.ligado:
            self.c_on = self.c_on + 1 if cond_on else 0
            if self.c_on >= K:
                self.ligado, self.c_on = True, 0
                self.eventos.append({"evento": "ligou", "t": int(t), "data": mes(t), **info})
        else:
            self.c_off = self.c_off + 1 if cond_off else 0
            if self.c_off >= K:
                self.ligado, self.c_off = False, 0
                self.eventos.append({"evento": "desligou", "t": int(t), "data": mes(t), **info})


def r1(d: pd.DataFrame, W: int, m: int):
    r = d[d["y"].notna()].sort_values("t_rotulo", kind="stable")
    p, y, t = C.oficial_binario(r), r["y"].to_numpy(float), r["t_rotulo"].to_numpy()
    h, serie = Histerese(), []
    for k in range(W, len(r) + 1, m):
        pp, yy = p[k - W:k], y[k - W:k]
        dd = yy.mean() - pp.mean()
        z = (yy.sum() - pp.sum()) / np.sqrt((pp * (1 - pp)).sum())
        h.passo(t[k - 1], abs(dd) > ON and abs(z) > Z, abs(dd) < OFF, {"d": round(float(dd), 4), "z": round(float(z), 2)})
        serie.append((int(t[k - 1]), float(dd), float(z), h.ligado))
    return h.eventos, serie


def r2(d: pd.DataFrame, W: int, m: int):
    e = d.sort_values("t_emissao", kind="stable")
    pe, te = C.oficial_binario(e), e["t_emissao"].to_numpy()
    r = d[d["y"].notna()].sort_values("t_rotulo", kind="stable")
    pr, yr, tr = C.oficial_binario(r), r["y"].to_numpy(float), r["t_rotulo"].to_numpy()
    h, serie, degeneradas = Histerese(), [], 0
    for k in range(W, len(e) + 1, m):
        t = te[k - 1]
        j = np.searchsorted(tr, t, side="right")
        if j < W:
            continue
        ps, ys = pr[j - W:j], yr[j - W:j]
        pi = ys.mean()
        yh = ps > pi
        if ys.sum() == 0 or (1 - ys).sum() == 0:
            degeneradas += 1
            continue
        tpr, fpr = yh[ys == 1].mean(), yh[ys == 0].mean()
        if tpr - fpr < 0.05:
            degeneradas += 1
            continue
        pa = pe[k - W:k]
        mu = (pa > pi).mean()
        q = float(np.clip((mu - fpr) / (tpr - fpr), 0, 1))
        db = q - pa.mean()
        h.passo(t, abs(db) > ON, abs(db) < OFF, {"d_B": round(db, 4), "q": round(q, 4), "p_alvo": round(float(pa.mean()), 4)})
        serie.append((int(t), float(db), q, float(pa.mean()), h.ligado))
    return h.eventos, serie, degeneradas


def estado(serie, corte):
    s = pd.DataFrame([(x[0], x[-1]) for x in serie], columns=["t", "on"])
    antes = s[s["t"] < corte]
    return {"ligado_no_corte": bool(antes["on"].iloc[-1]) if len(antes) else None,
            "fracao_ligado_dev": float(antes["on"].mean()) if len(antes) else None,
            "fracao_ligado_cofre": float(s[s["t"] >= corte]["on"].mean()) if (s["t"] >= corte).any() else None}


def resumo_eventos(ev, corte):
    lig = [e for e in ev if e["evento"] == "ligou"]
    return {"ligou_dev": len([e for e in lig if e["t"] < corte]), "ligou_cofre": len([e for e in lig if e["t"] >= corte]),
            "primeiro_no_cofre": next((e["data"] for e in lig if e["t"] >= corte), None),
            "ultimo_antes_do_corte": next((e["data"] for e in reversed(lig) if e["t"] < corte), None)}


def main():
    out = {}
    for fl in FLUXOS:
        res = resultados(fl)
        corte = res["corte"]
        d = diario(fl)
        W, m = PARAM[fl]
        e1, s1 = r1(d, W, m)
        e2, s2, deg = r2(d, W, m)
        orig = [{"sensor": e["sensor"], "evento": e["evento"], "t": e["t"], "data": mes(e["t"])}
                for e in res["eventos_operacao"] if e["tipo"] == "deriva"]
        out[fl] = {"W": W, "m": m, "corte": mes(corte),
                   "original": {"eventos": orig, **resumo_eventos(orig, corte)},
                   "R1": {"eventos": e1, **resumo_eventos(e1, corte), **estado(s1, corte), "verificacoes": len(s1),
                          "max_abs_d_cofre": max((abs(x[1]) for x in s1 if x[0] >= corte), default=None)},
                   "R2": {"eventos": e2, **resumo_eventos(e2, corte), **estado(s2, corte), "verificacoes": len(s2), "degeneradas": deg,
                          "max_abs_d_cofre": max((abs(x[1]) for x in s2 if x[0] >= corte), default=None)}}
        pd.DataFrame(s1, columns=["t", "d", "z", "ligado"]).to_csv(AQUI / "saida" / f"r1_serie_{fl}.csv", index=False)
        pd.DataFrame(s2, columns=["t", "d_B", "q", "p_alvo", "ligado"]).to_csv(AQUI / "saida" / f"r2_serie_{fl}.csv", index=False)
        o = out[fl]
        print(fl, "| original", {k: v for k, v in o["original"].items() if k != "eventos"},
              "| R1", {k: v for k, v in o["R1"].items() if k != "eventos"},
              "| R2", {k: v for k, v in o["R2"].items() if k != "eventos"})
    (AQUI / "saida" / "rotulo.json").write_text(json.dumps(out, indent=1, ensure_ascii=False))


if __name__ == "__main__":
    main()
