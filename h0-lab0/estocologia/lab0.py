"""Lab 0, Parte 11: o posto de saúde como processo gerador (agent-f1-code).

Reproduz a simulação do notebook de soluções do curso
(materiais/m0/lab0-a-funcao-da-media-solucoes.ipynb, Exercício 11) com a mesma
semente e a mesma ordem de sorteios, e acrescenta o modelo nulo e o teste severo
que o H0 exige (Φ3).

Unidades: taxas por hora; esperas devolvidas em minutos, salvo onde dito.
"""

from __future__ import annotations

import numpy as np

SEMENTE = 1909  # Erlang, 1909; a mesma do notebook
MU = 12.0
HORAS = 10
DIAS = 2_000


def W(lam: float, mu: float = MU) -> float:
    """Espera média de longo prazo da fila M/M/1, em horas. Pré: 0 <= lam < mu."""
    if not 0 <= lam < mu:
        raise ValueError(f"fila instável ou taxa inválida: lam={lam}, mu={mu}")
    return lam / (mu * (mu - lam))


def lindley(chegadas: np.ndarray, servico: np.ndarray) -> np.ndarray:
    """Espera de cada paciente (recursão de Lindley), na unidade das entradas.

    Pré: chegadas ordenadas, servico >= 0, mesmo tamanho. Pós: espera >= 0, espera[0] = 0.
    """
    n = chegadas.size
    espera = np.zeros(n)
    for i in range(1, n):
        espera[i] = max(0.0, espera[i - 1] + servico[i - 1] - (chegadas[i] - chegadas[i - 1]))
    return espera


def fila_do_dia(lam: float, mu: float, horas: float, rng: np.random.Generator) -> np.ndarray:
    """Um dia de um posto com um atendente, aberto vazio. Esperas em horas."""
    n = rng.poisson(lam * horas)
    chegadas = np.sort(rng.uniform(0, horas, n))
    servico = rng.exponential(1 / mu, n)
    return lindley(chegadas, servico)


def mundo(sorteia_taxa, rng, dias=DIAS, horas=HORAS, mu_de=lambda lam: MU):
    """Esperas de todos os pacientes (min), a taxa do dia de cada um e o índice do dia."""
    esperas, taxas, dia = [], [], []
    for d in range(dias):
        lam = sorteia_taxa()
        e = fila_do_dia(lam, mu_de(lam), horas, rng)
        esperas.append(60 * e)
        taxas.append(np.full(e.size, lam))
        dia.append(np.full(e.size, d))
    return np.concatenate(esperas), np.concatenate(taxas), np.concatenate(dia)


def media_por_paciente(e: np.ndarray, dia: np.ndarray) -> tuple[float, float]:
    """Média por paciente e erro padrão com o dia como unidade independente (razão de somas)."""
    soma, n = np.bincount(dia, weights=e), np.bincount(dia)
    m = soma.sum() / n.sum()
    return float(m), float(np.sqrt(np.sum((soma - m * n) ** 2)) / n.sum())


def erro_ingenuo(e: np.ndarray) -> float:
    """Erro padrão tratando cada paciente como independente (a pseudorreplicação que Φ2 barra)."""
    return float(e.std(ddof=1) / np.sqrt(e.size))


def ecdf(x):
    x = np.sort(np.asarray(x, dtype=float))
    return x, np.arange(1, x.size + 1) / x.size


def nulo_diferenca(rng: np.random.Generator, dias=DIAS, reps=2_000) -> np.ndarray:
    """Modelo nulo (M5): se a variabilidade da demanda não importasse, os dois mundos seriam
    o mesmo mundo da alegação. Simula 2·dias dias com λ = 8 e reparte os dias ao acaso em
    duas metades, `reps` vezes; devolve as diferenças de espera média por paciente (min)."""
    e, _, d = mundo(lambda: 8.0, rng, dias=2 * dias)
    soma, n = np.bincount(d, weights=e), np.bincount(d)
    difs = np.empty(reps)
    for r in range(reps):
        a = rng.permutation(2 * dias) < dias
        difs[r] = soma[a].sum() / n[a].sum() - soma[~a].sum() / n[~a].sum()
    return difs


def executar(semente: int = SEMENTE) -> dict:
    """Roda o Exercício 11 inteiro e o teste severo. Devolve só números (serializável)."""
    rng = np.random.default_rng(semente)
    r: dict = {"semente": semente}
    # (a) a função da média contra a média da função
    r["W_8"], r["W_5"], r["W_11"] = (60 * W(x) for x in (8, 5, 11))
    r["media_W"] = 60 * (W(5) + W(11)) / 2
    # (b) os dois mundos, na ordem de sorteio do notebook
    var_e, var_tx, var_d = mundo(lambda: rng.choice([5.0, 11.0]), rng)
    fix_e, _, fix_d = mundo(lambda: 8.0, rng)
    for nome, e, d in (("variavel", var_e, var_d), ("alegacao", fix_e, fix_d)):
        m, ep = media_por_paciente(e, d)
        r[nome] = {
            "pacientes": int(e.size),
            "dias": int(d.max() + 1),
            "espera_media": m,
            "ep_dia": ep,
            "ep_paciente": erro_ingenuo(e),
            "frac_acima_30": float(np.mean(e > 30)),
            "p90": float(np.quantile(e, 0.9)),
            "mediana": float(np.median(e)),
        }
    r["variavel"]["calmos"] = float(var_e[var_tx == 5].mean())
    r["variavel"]["surto"] = float(var_e[var_tx == 11].mean())
    r["variavel"]["frac_pacientes_surto"] = float(np.mean(var_tx == 11))
    # (d) outros mundos com a mesma média 8
    novos = {
        "7 ou 9": (lambda: rng.choice([7.0, 9.0]), lambda lam: 12.0),
        "uniforme 5-11": (lambda: rng.uniform(5, 11), lambda lam: 12.0),
        "5 ou 11, reforço no surto": (lambda: rng.choice([5.0, 11.0]), lambda lam: 24.0 if lam > 8 else 12.0),
    }
    r["mundos"] = {
        "8 todo dia": [r["alegacao"]["espera_media"], r["alegacao"]["frac_acima_30"]],
        "5 ou 11": [r["variavel"]["espera_media"], r["variavel"]["frac_acima_30"]],
    }
    for nome, (taxa, mu_de) in novos.items():
        e, _, d = mundo(taxa, rng, mu_de=mu_de)
        r["mundos"][nome] = [media_por_paciente(e, d)[0], float(np.mean(e > 30))]
    # Teste severo (Φ3), num fluxo de sorteios separado para não perturbar a reprodução
    rng_nulo = np.random.default_rng(semente + 1)
    difs = nulo_diferenca(rng_nulo)
    obs = r["variavel"]["espera_media"] - r["alegacao"]["espera_media"]
    r["teste_severo"] = {
        "diferenca_observada": obs,
        "nulo_dp": float(difs.std(ddof=1)),
        "nulo_max_abs": float(np.abs(difs).max()),
        "frac_nulo_ge_obs": float(np.mean(np.abs(difs) >= abs(obs))),
        "reps": int(difs.size),
    }
    return r
