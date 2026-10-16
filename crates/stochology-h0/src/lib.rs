//! stochology-h0 — a fila e a central de chamadas: o kernel do horizonte zero.
//!
//! Aqui vivem a formula de Erlang, a escala minima que entrega uma meta de servico, o **nulo de
//! Poisson** que serve de contraste e o resumo obrigatorio da segunda proibicao (media, mediana,
//! quantis) — o H0 e o horizonte onde o pedagio da media solitaria foi medido, e o helper e
//! compartilhado pelos outros horizontes.
//!
//! Declarado e nao implementado: a fila com desistencia (vive em `stochology-book/src/fila.rs`) e a
//! leitura da base real `dados-reais/bank_calls.csv` (vive no binario `labs --banco`).

use stochology_core::Rng;

/// Erlang B: probabilidade de bloqueio de `agentes` servidores com carga `carga = lambda/mu`.
/// Recorrencia estavel, presa em [0, 1].
pub fn erlang_b(agentes: u32, carga: f64) -> f64 {
    if carga <= 0.0 || agentes == 0 {
        return 0.0;
    }
    let mut b = 1.0_f64;
    for k in 1..=agentes {
        let kf = k as f64;
        b = carga * b / (kf + carga * b);
    }
    b.clamp(0.0, 1.0)
}

/// Erlang C: probabilidade de esperar, com `agentes` servidores e carga `carga`.
pub fn erlang_c(agentes: u32, carga: f64) -> f64 {
    if agentes == 0 {
        return 1.0;
    }
    if carga >= agentes as f64 {
        return 1.0;
    }
    let b = erlang_b(agentes, carga);
    let rho = carga / agentes as f64;
    (b / (1.0 - rho * (1.0 - b))).clamp(0.0, 1.0)
}

/// Menor numero de servidores que faz o serviço imediato cobrir `meta` da carga: Erlang C <= 1 - meta.
pub fn escala_minima(lambda: f64, mu: f64, meta: f64) -> u32 {
    if mu <= 0.0 || lambda <= 0.0 {
        return 0;
    }
    let carga = lambda / mu;
    let mut agentes = carga.ceil().max(1.0) as u32;
    while erlang_c(agentes, carga) > 1.0 - meta && agentes < 100_000 {
        agentes += 1;
    }
    agentes
}

/// Modelo nulo declarado do H0: chegadas de Poisson com a mesma media.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NuloPoisson {
    pub media: f64,
}

impl NuloPoisson {
    pub fn novo(media: f64) -> Self {
        NuloPoisson { media }
    }

    /// Uma contagem por passo, pelo metodo de Knuth.
    pub fn contagem(&self, rng: &mut Rng) -> u32 {
        if self.media <= 0.0 {
            return 0;
        }
        if self.media > 700.0 {
            // fora da faixa do metodo: Gaussiana com media e variancia iguais
            return (self.media + self.media.sqrt() * crate_normal(rng)).max(0.0).round() as u32;
        }
        let limite = (-self.media).exp();
        let mut produto = 1.0_f64;
        let mut k = 0u32;
        loop {
            k += 1;
            produto *= uniforme_aberta(rng);
            if produto <= limite || k > 10_000 {
                return k - 1;
            }
        }
    }

    pub fn serie(&self, semente: u64, passos: usize) -> Vec<u32> {
        let mut rng = Rng::novo(semente);
        (0..passos).map(|_| self.contagem(&mut rng)).collect()
    }
}

fn uniforme_aberta(rng: &mut Rng) -> f64 {
    ((rng.proximo() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
}

fn crate_normal(rng: &mut Rng) -> f64 {
    let u1 = uniforme_aberta(rng);
    let u2 = uniforme_aberta(rng);
    (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
}

/// Indice de dispersao: variancia sobre media. Poisson fica em 1; a cauda pesada fica acima.
pub fn indice_de_dispersao(contagens: &[u32]) -> f64 {
    if contagens.len() < 2 {
        return 0.0;
    }
    let n = contagens.len() as f64;
    let media = contagens.iter().map(|c| *c as f64).sum::<f64>() / n;
    if media == 0.0 {
        return 0.0;
    }
    let variancia = contagens
        .iter()
        .map(|c| (*c as f64 - media).powi(2))
        .sum::<f64>()
        / (n - 1.0);
    variancia / media
}

/// Resumo obrigatorio onde a funcao e nao linear: media, mediana, quantis e dispersao, juntos.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Resumo {
    pub n: usize,
    pub media: f64,
    pub mediana: f64,
    pub p05: f64,
    pub p95: f64,
    pub maximo: f64,
    pub desvio: f64,
}

/// Quantil por interpolacao linear sobre a amostra ordenada (regra type 7).
pub fn quantil(ordenada: &[f64], q: f64) -> f64 {
    if ordenada.is_empty() {
        return 0.0;
    }
    let q = q.clamp(0.0, 1.0);
    let posicao = q * (ordenada.len() as f64 - 1.0);
    let baixo = posicao.floor() as usize;
    let alto = posicao.ceil() as usize;
    if baixo == alto {
        return ordenada[baixo];
    }
    let peso = posicao - baixo as f64;
    ordenada[baixo] * (1.0 - peso) + ordenada[alto] * peso
}

pub fn resumo(serie: &[f64]) -> Resumo {
    let mut ordenada = serie.to_vec();
    ordenada.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = ordenada.len();
    let media = if n == 0 { 0.0 } else { ordenada.iter().sum::<f64>() / n as f64 };
    let desvio = if n < 2 {
        0.0
    } else {
        (ordenada.iter().map(|x| (x - media).powi(2)).sum::<f64>() / (n as f64 - 1.0)).sqrt()
    };
    Resumo {
        n,
        media,
        mediana: quantil(&ordenada, 0.5),
        p05: quantil(&ordenada, 0.05),
        p95: quantil(&ordenada, 0.95),
        maximo: ordenada.last().copied().unwrap_or(0.0),
        desvio,
    }
}

/// O pedagio de reportar so a media: quanto a media passa da mediana, em fracao da mediana.
pub fn pedagio_da_media(serie: &[f64]) -> f64 {
    let r = resumo(serie);
    if r.mediana == 0.0 {
        return 0.0;
    }
    r.media / r.mediana - 1.0
}

#[cfg(test)]
mod testes {
    use super::*;

    /// Rota independente: soma direta de `a^k/k!` em escala logaritmica (log-sum-exp), sem recorrencia.
    fn erlang_c_pela_soma(agentes: u32, carga: f64) -> f64 {
        let ln_fatorial: Vec<f64> = (0..=agentes as usize)
            .scan(0.0_f64, |acc, k| {
                if k > 0 {
                    *acc += (k as f64).ln();
                }
                Some(*acc)
            })
            .collect();
        let termos: Vec<f64> = (0..=agentes as usize)
            .map(|k| (k as f64) * carga.ln() - ln_fatorial[k])
            .collect();
        let maior = termos.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let ultimos = termos[..agentes as usize]
            .iter()
            .map(|t| (t - maior).exp())
            .sum::<f64>();
        let fila = (termos[agentes as usize] - maior).exp() * (agentes as f64) / (agentes as f64 - carga);
        fila / (ultimos + fila)
    }

    #[test]
    fn erlang_bate_com_a_referencia_exata() {
        // Valores exatos calculados com fracao racional (Fraction) em 2026-10-16.
        let casos = [
            (3u32, 2.0_f64, 0.444_444_444_444_444_4_f64),
            (5, 3.0, 0.236_151_603_498_542_28),
            (10, 5.0, 0.036_105_359_158_320_19),
            (20, 17.0, 0.385_056_229_893_324_83),
        ];
        for (agentes, carga, esperado) in casos {
            let obtido = erlang_c(agentes, carga);
            assert!(
                (obtido - esperado).abs() < 1e-12,
                "erlang_c({agentes}, {carga}) = {obtido}, referencia {esperado}"
            );
        }
    }

    #[test]
    fn a_recorrencia_e_a_soma_logaritmica_concordam() {
        for (agentes, carga) in [(1u32, 0.5_f64), (5, 3.0), (10, 5.0), (30, 20.0), (60, 45.0)] {
            let recorrencia = erlang_c(agentes, carga);
            let soma = erlang_c_pela_soma(agentes, carga);
            assert!(
                (recorrencia - soma).abs() < 1e-9,
                "duas rotas divergem em ({agentes}, {carga}): {recorrencia} contra {soma}"
            );
        }
    }

    #[test]
    fn erlang_c_cai_com_mais_servidores_e_fica_entre_b_e_um() {
        let carga = 8.0;
        let mut anterior = 1.0;
        for agentes in 9..40 {
            let c = erlang_c(agentes, carga);
            assert!(c <= anterior + 1e-12, "nao caiu em {agentes}: {c} > {anterior}");
            assert!(c >= erlang_b(agentes, carga) - 1e-12);
            assert!((0.0..=1.0).contains(&c));
            anterior = c;
        }
        assert_eq!(erlang_c(8, 8.0), 1.0, "carga igual a capacidade nao segura a fila");
    }

    #[test]
    fn escala_minima_entrega_a_meta_e_uma_a_menos_nao() {
        for (lambda, mu, meta) in [(8.0_f64, 1.0_f64, 0.8_f64), (234.0, 1.0, 0.8), (5.0, 2.0, 0.9)] {
            let agentes = escala_minima(lambda, mu, meta);
            let carga = lambda / mu;
            assert!(erlang_c(agentes, carga) <= 1.0 - meta + 1e-12);
            if (agentes as f64) > carga.ceil() {
                assert!(erlang_c(agentes - 1, carga) > 1.0 - meta - 1e-12);
            }
        }
    }

    #[test]
    fn nulo_de_poisson_bate_media_e_variancia() {
        let nulo = NuloPoisson::novo(8.0);
        let serie = nulo.serie(2026, 200_000);
        let media = serie.iter().map(|c| *c as f64).sum::<f64>() / serie.len() as f64;
        assert!((media - 8.0).abs() < 0.08, "media do nulo: {media}");
        let dispersao = indice_de_dispersao(&serie);
        assert!((dispersao - 1.0).abs() < 0.05, "Poisson fica em 1: {dispersao}");
    }

    #[test]
    fn a_serie_do_nulo_e_deterministica() {
        let nulo = NuloPoisson::novo(3.0);
        assert_eq!(nulo.serie(99, 500), nulo.serie(99, 500));
        assert_ne!(nulo.serie(99, 500), nulo.serie(100, 500));
    }

    #[test]
    fn a_cauda_pesada_estoura_o_indice_de_dispersao() {
        // Nulo declarado: Poisson, que fica em 1. O mundo de dois regimes fica muito acima.
        let nulo = NuloPoisson::novo(4.0);
        let quieto = nulo.serie(1, 20_000);
        let mut rng = Rng::novo(2);
        let mut agitado = Vec::with_capacity(20_000);
        for i in 0..20_000 {
            let media = if i % 20 == 0 { 40.0 } else { 2.0 };
            agitado.push(NuloPoisson::novo(media).contagem(&mut rng));
        }
        let mansa = indice_de_dispersao(&quieto);
        let brava = indice_de_dispersao(&agitado);
        assert!(mansa < 1.1, "nulo fica perto de 1: {mansa}");
        assert!(brava > 3.0, "dois regimes estouram o nulo: {brava}");
    }

    #[test]
    fn o_resumo_traz_mediana_e_quantis_junto_com_a_media() {
        let mut rng = Rng::novo(42);
        let serie: Vec<f64> = (0..20_000)
            .map(|_| {
                let u = uniforme_aberta(&mut rng);
                (1.0 / (1.0 - u)).powf(1.0 / 1.81)
            })
            .collect();
        let r = resumo(&serie);
        assert_eq!(r.n, 20_000);
        assert!(r.p05 < r.mediana && r.mediana < r.p95 && r.p95 <= r.maximo);
        assert!(r.media > r.mediana, "cauda a direita: media {} mediana {}", r.media, r.mediana);
        assert!(pedagio_da_media(&serie) > 0.5, "pedagio da media solitaria: {}", pedagio_da_media(&serie));
    }

    #[test]
    fn quantil_interpola_e_nao_sai_da_amostra() {
        let ordenada = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(quantil(&ordenada, 0.0), 1.0);
        assert_eq!(quantil(&ordenada, 1.0), 4.0);
        assert!((quantil(&ordenada, 0.5) - 2.5).abs() < 1e-12);
        assert!((quantil(&ordenada, 0.25) - 1.75).abs() < 1e-12);
    }
}
