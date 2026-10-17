//! Motor das cinco lentes operacionais.

use stochology_core::Rng;

/// Fluxonomia: informação mútua empírica I(X;Y), em bits.
pub fn informacao_mutua(x: &[usize], y: &[usize]) -> f64 {
    assert_eq!(x.len(), y.len(), "amostras de tamanhos diferentes");
    let n = x.len() as f64;
    if n == 0.0 {
        return 0.0;
    }
    let mut total = 0.0;
    let mut xs: Vec<usize> = x.to_vec();
    let mut ys: Vec<usize> = y.to_vec();
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();
    for &a in &xs {
        for &b in &ys {
            let n_xy = x
                .iter()
                .zip(y.iter())
                .filter(|(p, q)| **p == a && **q == b)
                .count() as f64;
            let n_x = x.iter().filter(|p| **p == a).count() as f64;
            let n_y = y.iter().filter(|q| **q == b).count() as f64;
            if n_xy > 0.0 {
                total += (n_xy / n) * ((n_xy * n) / (n_x * n_y)).log2();
            }
        }
    }
    total
}

/// Desigualdade do processamento: I(X;Z) <= I(X;Y) quando Z sai de Y.
pub fn fluxonomia_nao_aumenta(x: &[usize], y: &[usize], z: &[usize]) -> bool {
    informacao_mutua(x, z) <= informacao_mutua(x, y) + 1e-9
}

/// Crononomia: arrasto da volatilidade, mu - sigma^2/2.
pub fn arrasto_da_volatilidade(mu: f64, sigma: f64) -> f64 {
    mu - 0.5 * sigma * sigma
}

/// Crononomia: a moeda de Peters cresce na média de conjunto e morre no tempo.
pub fn nao_ergodico(ganho: f64, perda: f64) -> bool {
    (1.0 + ganho) * (1.0 - perda) < 1.0
}

/// Crononomia: simulação multiplicativa com semente fixa.
/// Devolve (mediana do log da riqueza, fração de trajetórias arruinadas).
pub fn simular_multiplicativo(
    semente: u64,
    passos: usize,
    trajetorias: usize,
    ganho: f64,
    perda: f64,
) -> (f64, f64) {
    let mut rng = Rng::novo(semente);
    let mut logs: Vec<f64> = Vec::with_capacity(trajetorias);
    let mut arruinadas = 0usize;
    let limiar = 0.1_f64.ln();
    for _ in 0..trajetorias {
        let mut log_riqueza = 0.0_f64;
        for _ in 0..passos {
            if rng.uniforme(2) == 0 {
                log_riqueza += (1.0 + ganho).ln();
            } else {
                log_riqueza += (1.0 - perda).ln();
            }
        }
        if log_riqueza < limiar {
            arruinadas += 1;
        }
        logs.push(log_riqueza);
    }
    logs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mediana = logs[logs.len() / 2];
    (mediana, arruinadas as f64 / trajetorias as f64)
}

/// Escalonomia: estimador de Hill sobre a cauda superior.
pub fn indice_hill(valores: &[f64], k: usize) -> f64 {
    stochology_core::hill_no_corte(valores, k, k)
}

/// Escalonomia: cauda selvagem quando o expoente fica abaixo de 2.
pub fn cauda_selvagem(alfa: f64) -> bool {
    alfa < 2.0
}

/// Morfonomia: inclinação por mínimos quadrados.
pub fn inclinacao(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let num: f64 = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| (x - mx) * (y - my))
        .sum();
    let den: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    num / den
}

/// Morfonomia: a mesma regra sobrevive quando muda o regime.
pub fn invariante_entre_regimes(inclinacoes: &[f64], tolerancia: f64) -> bool {
    match (
        inclinacoes.iter().cloned().fold(f64::MIN, f64::max),
        inclinacoes.iter().cloned().fold(f64::MAX, f64::min),
    ) {
        (maior, menor) => (maior - menor).abs() <= tolerancia,
    }
}

/// Crisonomia: escore de Brier.
pub fn escore_de_brier(previsoes: &[f64], resultados: &[f64]) -> f64 {
    let n = previsoes.len() as f64;
    previsoes
        .iter()
        .zip(resultados.iter())
        .map(|(p, o)| (p - o) * (p - o))
        .sum::<f64>()
        / n
}

/// Crisonomia: decomposição de Murphy (REL, RES, UNC).
///
/// Agrupa por previsão exata: para resultados binários a identidade
/// Brier = REL - RES + UNC fecha byte a byte.
pub fn decomposicao_murphy(prev: &[f64], res: &[f64]) -> (f64, f64, f64) {
    stochology_core::decomposicao_murphy(prev, res)
}

/// Crisonomia: valor esperado da informação perfeita.
///
/// pagamentos é o payoff do melhor ato em cada estado; melhor_sem_info é o
/// melhor payoff esperado antes de olhar o dado.
pub fn evpi(pagamentos: &[f64], probabilidades: &[f64], melhor_sem_info: f64) -> f64 {
    let com_info: f64 = pagamentos
        .iter()
        .zip(probabilidades.iter())
        .map(|(p, w)| p * w)
        .sum();
    com_info - melhor_sem_info
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn cadeia_deterministica_respeita_processamento() {
        let x: Vec<usize> = (0..400).map(|i| i % 4).collect();
        let y: Vec<usize> = x.iter().map(|v| v / 2).collect();
        let z: Vec<usize> = y.iter().map(|v| (*v + 1) % 2).collect();
        assert!(fluxonomia_nao_aumenta(&x, &y, &z));
    }

    #[test]
    fn moeda_de_peters_nao_e_ergodica() {
        assert!(nao_ergodico(0.50, 0.40));
        let (mediana, arruinadas) = simular_multiplicativo(2026, 400, 2_000, 0.50, 0.40);
        assert!(mediana < 0.0, "mediana da riqueza deveria encolher: {}", mediana);
        assert!(arruinadas > 0.9, "fração arruinada: {}", arruinadas);
    }

    #[test]
    fn arrasto_da_volatilidade_desconta_variacao() {
        assert!((arrasto_da_volatilidade(0.051, 0.0) - 0.051).abs() < 1e-12);
        assert!(arrasto_da_volatilidade(0.051, 0.2) < 0.051);
    }

    #[test]
    fn hill_recupera_alfa_de_pareto() {
        let mut rng = Rng::novo(1909);
        let alfa = 1.81;
        let amostra: Vec<f64> = (0..20_000)
            .map(|_| {
                let u = (rng.proximo() as f64) / (u64::MAX as f64);
                (1.0 - u).powf(-1.0 / alfa)
            })
            .collect();
        let h = indice_hill(&amostra, 500);
        assert!((h - alfa).abs() < 0.25, "Hill devolveu {}", h);
        assert!(cauda_selvagem(h));
    }

    #[test]
    fn regime_invariante_aprova_regime_quebrado() {
        assert!(invariante_entre_regimes(&[2.00, 2.02, 1.99], 0.05));
        assert!(!invariante_entre_regimes(&[2.00, 2.60], 0.05));
    }

    #[test]
    fn brier_e_murphy_fecham_a_conta() {
        let prev = [0.1, 0.1, 0.2, 0.2, 0.8, 0.8, 0.9, 0.9];
        let res = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let brier = escore_de_brier(&prev, &res);
        let (rel, resol, unc) = decomposicao_murphy(&prev, &res);
        assert!((brier - 0.1).abs() < 1e-12, "brier {}", brier);
        assert!((brier - (rel - resol + unc)).abs() < 1e-12);
    }

    #[test]
    fn evpi_do_exemplo_vale_tres() {
        let pagamentos = [10.0, 10.0];
        let probabilidades = [0.7, 0.3];
        // Melhor ato sem informação: A rende 0,7*10 = 7.
        assert!((evpi(&pagamentos, &probabilidades, 7.0) - 3.0).abs() < 1e-9);
        assert!((evpi(&pagamentos, &probabilidades, 10.0)).abs() < 1e-9);
    }

    #[test]
    fn inclinacao_simples() {
        let xs = [1.0, 2.0, 3.0, 4.0];
        let ys = [3.0, 5.0, 7.0, 9.0];
        assert!((inclinacao(&xs, &ys) - 2.0).abs() < 1e-9);
    }
}
