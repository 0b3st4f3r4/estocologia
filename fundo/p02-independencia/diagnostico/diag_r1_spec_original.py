"""Diagnóstico pós-congelamento (não faz parte da versão congelada): meu código de R1 com as covariáveis do
escore que o original usa (docstring de h1-real/projetos/b_real.py:22-24, não publicadas no README):
idade, idade², educ, educ², negro, hispânico, casado, sem diploma, re74, u74, re74², re75, u75, re75²."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import numpy as np, pandas as pd
from comum import DADOS, logit_ajustar, sigm
nsw = pd.read_csv(os.path.join(DADOS, "nsw_experimental.csv")); cps = pd.read_csv(os.path.join(DADOS, "cps_observational.csv"))
tr = nsw[nsw.treat == 1]
def X(d):
    r74, r75 = d.re74 / 1000, d.re75 / 1000
    c = [d.age, d.age**2, d.educ, d.educ**2, d.black, d.hisp, d.marr, d.nodegree, r74, (d.re74 == 0)*1.0, r74**2,
         r75, (d.re75 == 0)*1.0, r75**2]
    return np.column_stack([np.ones(len(d))] + [np.asarray(v, float) for v in c])
for lam in (0.0, 1e-3 / (len(tr) + len(cps))):
    A = np.vstack([X(tr), X(cps)]); z = np.r_[np.ones(len(tr)), np.zeros(len(cps))]
    mu, sd = A[:, 1:].mean(0), A[:, 1:].std(0); A[:, 1:] = (A[:, 1:] - mu) / sd
    e = sigm(A @ logit_ajustar(A, z, lam=lam)); ec = e[len(tr):]; w = ec / (1 - ec)
    yt, yc = (tr.re78 > 0).mean(), ((cps.re78 > 0) * w).sum() / w.sum()
    print(f"lam={lam:.2e}  dR_ipw={yt - yc:.4f}  drenda={tr.re78.mean() - (cps.re78 * w).sum() / w.sum():.1f}  "
          f"ess={w.sum()**2 / (w**2).sum():.1f}  frac_abaixo={np.mean(ec < e[:len(tr)].min()):.4f}")
