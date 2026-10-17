//! Núcleo da Estocologia: camadas L1-L3, invariantes Phi1-Phi4,
//! banca adversarial A1-A8 e orquestrador M.
//!
//! Todo artefato é determinístico: nenhuma função usa relógio, entropia do
//! sistema ou estado global. As sementes são explícitas em cada ficha.

pub mod agentes;
pub mod camada;
pub mod erlang;
pub mod funil;
pub mod invariantes;
pub mod orquestrador;
pub mod resolucao;
pub mod rng;
pub mod s1;

pub use agentes::{Aprovacao, AgenteA, Severidade, SEVERIDADE_MINIMA, aprovado, estressar};
pub use camada::{Camada, Ficha, TipoIncerteza};
pub use invariantes::{Falha, Portao, avaliar, relatorio};
pub use orquestrador::OrquestradorM;
pub use resolucao::{PerdaDeResolucao, Resolucao, medir as medir_perda_de_resolucao};
pub use rng::Rng;

/// Estimador de Hill sobre os `k` maiores valores positivos, usando `indice_corte` (`k` ou `k - 1`) como limiar.
pub fn hill_no_corte(serie: &[f64], k: usize, indice_corte: usize) -> f64 {
    let mut positivos: Vec<f64> = serie.iter().copied().filter(|x| *x > 0.0).collect();
    if positivos.len() <= indice_corte || positives_curto(positivos.len(), k) {
        return f64::NAN;
    }
    positivos.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let corte = positivos[indice_corte];
    if corte <= 0.0 {
        return f64::NAN;
    }
    let soma: f64 = positivos[..k].iter().map(|x| (x / corte).ln()).sum();
    if soma <= 0.0 {
        return f64::NAN;
    }
    k as f64 / soma
}

fn positives_curto(len: usize, k: usize) -> bool {
    len < k || k == 0
}

/// Decomposição de Murphy (REL, RES, UNC) agrupando por valor exato de previsão.
pub fn decomposicao_murphy(prev: &[f64], res: &[f64]) -> (f64, f64, f64) {
    if prev.is_empty() || prev.len() != res.len() {
        return (0.0, 0.0, 0.0);
    }
    let n = prev.len() as f64;
    let base = res.iter().sum::<f64>() / n;
    let mut distintos: Vec<f64> = prev.to_vec();
    distintos.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    distintos.dedup();
    let mut rel = 0.0;
    let mut resol = 0.0;
    for valor in distintos {
        let idx: Vec<usize> = (0..prev.len()).filter(|i| prev[*i] == valor).collect();
        if idx.is_empty() {
            continue;
        }
        let nb = idx.len() as f64;
        let ob = idx.iter().map(|i| res[*i]).sum::<f64>() / nb;
        rel += (nb / n) * (valor - ob) * (valor - ob);
        resol += (nb / n) * (ob - base) * (ob - base);
    }
    (rel, resol, base * (1.0 - base))
}
