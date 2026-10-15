"""Sensores contínuos do H3: deriva (F3), disjuntor (F8) e calibração adaptativa de quantis (F4).

Sensor de deriva (PLAN §5.3). A cada janela de n valores novos de um sinal de entrada, calcula KS e
Wasserstein contra uma referência. Os limiares não são convencionais: saem da própria variação do
desenvolvimento. Nas janelas da fase de calibração, alto é o q99 e baixo o q90 do KS. A histerese
pede k_on janelas seguidas acima de alto para ligar o alarme e k_off abaixo de baixo para desligar.
O Wasserstein acompanha na mesma unidade do sinal; o KS decide.

Disjuntor (F8). Compara, numa janela móvel de previsões resolvidas, a perda do modelo com a do
reserva determinístico. O modelo continua prevendo em sombra enquanto o disjuntor está aberto, e é
a perda em sombra que fecha o disjuntor. Abrir pede k_on verificações seguidas com perda acima de
(1 + margem) vezes a do reserva; fechar pede k_off verificações seguidas abaixo da do reserva.

Calibrador de quantis (F4). O modelo dá centro m e escala s; os quantis saem dos resíduos
padronizados z = (y − m)/s já resolvidos (os últimos n_max), com o nível de cada quantil corrigido
online pela inferência conforme adaptativa (Gibbs & Candès 2021): τ_ef ← τ_ef + γ (τ − 1{y < q}).
"""

from __future__ import annotations

from collections import deque

import numpy as np
from scipy import stats


class SensorDeriva:
    def __init__(self, nome: str, n_janela: int, n_ref: int, k_on: int = 2, k_off: int = 2):
        self.nome, self.n, self.n_ref, self.k_on, self.k_off = nome, n_janela, n_ref, k_on, k_off
        self.ref: np.ndarray | None = None
        self.buf: list = []
        self.recentes: deque = deque(maxlen=n_ref)
        self.calib: list[float] = []
        self.alto = self.baixo = None
        self.ligado = False
        self.cont_on = self.cont_off = 0
        self.historico: list[dict] = []

    def alimentar(self, valores, agora: int, calibrando: bool) -> list[dict]:
        """Devolve os eventos de alarme (ligou/desligou) gerados por estes valores."""
        eventos = []
        for v in np.asarray(valores, dtype=float).ravel():
            if not np.isfinite(v):
                continue
            self.recentes.append(v)
            if self.ref is None:
                if len(self.recentes) == self.n_ref:
                    self.ref = np.array(self.recentes)
                continue
            self.buf.append(v)
            if len(self.buf) < self.n:
                continue
            jan = np.array(self.buf)
            self.buf = []
            ks = float(stats.ks_2samp(self.ref, jan).statistic)
            w = float(stats.wasserstein_distance(self.ref, jan))
            reg = {"t": agora, "ks": ks, "wasserstein": w, "alarme": self.ligado}
            if calibrando or self.alto is None:
                self.calib.append(ks)
            else:
                if not self.ligado:
                    self.cont_on = self.cont_on + 1 if ks > self.alto else 0
                    if self.cont_on >= self.k_on:
                        self.ligado, self.cont_on = True, 0
                        eventos.append({"sensor": self.nome, "evento": "ligou", "t": agora, "ks": ks, "wasserstein": w})
                else:
                    self.cont_off = self.cont_off + 1 if ks < self.baixo else 0
                    if self.cont_off >= self.k_off:
                        self.ligado, self.cont_off = False, 0
                        eventos.append({"sensor": self.nome, "evento": "desligou", "t": agora, "ks": ks, "wasserstein": w})
                reg["alarme"] = self.ligado
            self.historico.append(reg)
        return eventos

    def fixar_limiares(self) -> dict:
        c = np.array(self.calib) if self.calib else np.array([1.0])
        self.alto, self.baixo = float(np.quantile(c, 0.99)), float(np.quantile(c, 0.90))
        return {"sensor": self.nome, "alto": self.alto, "baixo": self.baixo, "janelas_calibracao": len(self.calib)}

    def rebasear(self) -> None:
        """Ciclo de vida (F3): depois do reajuste, a referência passa a ser o passado recente."""
        if len(self.recentes) >= self.n_ref // 2:
            self.ref = np.array(self.recentes)
        self.buf, self.cont_on, self.cont_off = [], 0, 0
        self.ligado = False


class Disjuntor:
    def __init__(self, janela: int, margem: float = 0.05, k_on: int = 3, k_off: int = 3, intervalo: int = 1):
        self.perdas: deque = deque(maxlen=janela)
        self.janela, self.margem, self.k_on, self.k_off = janela, margem, k_on, k_off
        self.intervalo = max(1, intervalo)
        self.aberto = False
        self.c_on = self.c_off = 0
        self.novos = 0

    def alimentar(self, perda_modelo: float, perda_reserva: float) -> None:
        self.perdas.append((perda_modelo, perda_reserva))
        self.novos += 1

    def verificar(self, agora: int) -> dict | None:
        if len(self.perdas) < self.janela or self.novos < self.intervalo:
            return None
        self.novos = 0
        a = np.array(self.perdas)
        pm, pr = float(a[:, 0].mean()), float(a[:, 1].mean())
        if not self.aberto:
            self.c_on = self.c_on + 1 if pm > (1 + self.margem) * pr else 0
            if self.c_on >= self.k_on:
                self.aberto, self.c_on = True, 0
                return {"evento": "abriu", "t": agora, "perda_modelo": pm, "perda_reserva": pr}
        else:
            self.c_off = self.c_off + 1 if pm < pr else 0
            if self.c_off >= self.k_off:
                self.aberto, self.c_off = False, 0
                return {"evento": "fechou", "t": agora, "perda_modelo": pm, "perda_reserva": pr}
        return None


class CalibradorQuantis:
    def __init__(self, niveis, n_max: int = 2000, gama: float = 0.005, n_min: int = 50):
        self.niveis = np.asarray(niveis, dtype=float)
        self.ef = self.niveis.copy()
        self.z: deque = deque(maxlen=n_max)
        self.gama, self.n_min = gama, n_min

    def quantis(self, m, s):
        m, s = np.asarray(m, float), np.asarray(s, float)
        if len(self.z) < self.n_min:
            zq = stats.norm.ppf(self.niveis)
        else:
            zq = np.quantile(np.array(self.z), np.clip(self.ef, 0.001, 0.999))
        return m[..., None] + s[..., None] * zq

    def atualizar(self, y: float, m: float, s: float, q: np.ndarray) -> None:
        if s > 0 and np.isfinite(y):
            self.z.append((y - m) / s)
            self.ef = self.ef + self.gama * (self.niveis - (y < q).astype(float))


def pinball(y: float, q: np.ndarray, niveis: np.ndarray) -> float:
    d = y - q
    return float(np.mean(np.maximum(niveis * d, (niveis - 1) * d)))
