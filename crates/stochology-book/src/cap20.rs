//! Capítulo 20 — o relógio ao vivo, o canário de vazamento e o cofre.
//!
//! Laboratório autocontido, sem dependências externas: SplitMix64 embutido, logística por mínimos
//! quadrados reponderados (IRLS), escalonamento de Platt, ECE de 10 caixas de massa igual e o
//! canário plantado de propósito (uma coluna igual ao rótulo, publicada no instante do rótulo).
//! Rodar da raiz do repositório:
//!     cargo run --release --bin labs -- --cap 20

const SEMENTE: u64 = 2021;
const N_TREINO: usize = 120_000;
const N_DESENVOLVIMENTO: usize = 58_000;
const N_COFRE: usize = 24_466;
const CAIXAS: usize = 10;

// As três covariáveis têm a mesma distribuição nos três períodos: o que mudou entre o período de
// treino (2005–2012) e o período do desenvolvimento e do cofre (2013–2015) foi o nível do mundo.
const P_NOITE: f64 = 0.18;
const P_VELOCIDADE: f64 = 0.28;
const P_PLACA: f64 = 0.12;

// O intercepto do período antigo (taxa ≈ 3,6%) e o do período novo (taxa ≈ 2,3%).
const B0_TREINO: f64 = -4.005;
const B0_NOVO: f64 = -4.42;
const B_NOITE: f64 = 0.8;
const B_VELOCIDADE: f64 = 1.0;
const B_PLACA: f64 = 0.5;

struct Sm64 {
    estado: u64,
}

impl Sm64 {
    fn novo(semente: u64) -> Self {
        Sm64 {
            estado: semente ^ 0x9E37_79B9_7F4A_7C15,
        }
    }
    fn proximo(&mut self) -> u64 {
        self.estado = self.estado.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.estado;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn uniforme(&mut self) -> f64 {
        ((self.proximo() >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn bernoulli(&mut self, p: f64) -> f64 {
        if self.uniforme() < p {
            1.0
        } else {
            0.0
        }
    }
}

fn sigmoide(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

fn logito(p: f64) -> f64 {
    (p / (1.0 - p)).ln()
}

struct Periodo {
    y: Vec<f64>,
    x: Vec<Vec<f64>>,
}

fn gerar(n: usize, b0: f64, p_noite: f64, p_velocidade: f64, p_placa: f64, rng: &mut Sm64) -> Periodo {
    let mut y = Vec::with_capacity(n);
    let noite = vec![0.0; n];
    let velocidade = vec![0.0; n];
    let placa = vec![0.0; n];
    let mut x = vec![vec![1.0; n], noite, velocidade, placa];
    for i in 0..n {
        let x1 = rng.bernoulli(p_noite);
        let x2 = rng.bernoulli(p_velocidade);
        let x3 = rng.bernoulli(p_placa);
        x[1][i] = x1;
        x[2][i] = x2;
        x[3][i] = x3;
        let eta = b0 + B_NOITE * x1 + B_VELOCIDADE * x2 + B_PLACA * x3;
        y.push(rng.bernoulli(sigmoide(eta)));
    }
    Periodo { y, x }
}

// ---------------------------------------------------------------- logística por IRLS

/// Logística por mínimos quadrados reponderados, com uma crista pequena fora do intercepto.
fn ajustar(x: &[Vec<f64>], y: &[f64], lambda: f64) -> Vec<f64> {
    let p = x.len();
    let n = y.len();
    let mut b = vec![0.0; p];
    for _ in 0..60 {
        let mut eta = vec![0.0; n];
        let mut w = vec![0.0; n];
        let mut z = vec![0.0; n];
        for i in 0..n {
            eta[i] = (0..p).map(|c| x[c][i] * b[c]).sum();
            let mu = sigmoide(eta[i]).clamp(1e-9, 1.0 - 1e-9);
            w[i] = (mu * (1.0 - mu)).max(1e-9);
            z[i] = eta[i] + (y[i] - mu) / w[i];
        }
        let mut a = vec![vec![0.0; p + 1]; p];
        for i in 0..n {
            for j in 0..p {
                for k in 0..p {
                    a[j][k] += w[i] * x[j][i] * x[k][i];
                }
                a[j][p] += w[i] * x[j][i] * z[i];
            }
        }
        for j in 1..p {
            a[j][j] += lambda;
        }
        for col in 0..p {
            let mut piv = col;
            for r in (col + 1)..p {
                if a[r][col].abs() > a[piv][col].abs() {
                    piv = r;
                }
            }
            a.swap(col, piv);
            let d = a[col][col];
            for k in col..=p {
                a[col][k] /= d;
            }
            for r in 0..p {
                if r != col {
                    let f = a[r][col];
                    if f != 0.0 {
                        for k in col..=p {
                            a[r][k] -= f * a[col][k];
                        }
                    }
                }
            }
        }
        let novo: Vec<f64> = (0..p).map(|j| a[j][p]).collect();
        let passo: f64 = (0..p).map(|j| (novo[j] - b[j]).abs()).fold(0.0, f64::max);
        b = novo;
        if passo < 1e-10 {
            break;
        }
    }
    b
}

fn prever(b: &[f64], x: &[Vec<f64>], i: usize) -> f64 {
    sigmoide((0..b.len()).map(|c| x[c][i] * b[c]).sum())
}

// ---------------------------------------------------------------- métricas

fn ece(p: &[f64], y: &[f64]) -> f64 {
    let n = p.len();
    let mut ordem: Vec<usize> = (0..n).collect();
    ordem.sort_by(|&a, &b| p[a].partial_cmp(&p[b]).unwrap());
    let tam = n / CAIXAS;
    let mut soma = 0.0;
    for c in 0..CAIXAS {
        let ini = c * tam;
        let fim = if c == CAIXAS - 1 { n } else { (c + 1) * tam };
        let m = (fim - ini) as f64;
        let mp: f64 = (ini..fim).map(|k| p[ordem[k]]).sum::<f64>() / m;
        let my: f64 = (ini..fim).map(|k| y[ordem[k]]).sum::<f64>() / m;
        soma += m / n as f64 * (mp - my).abs();
    }
    soma
}

fn brier(p: &[f64], y: &[f64]) -> f64 {
    let n = p.len() as f64;
    p.iter().zip(y).map(|(a, b)| (a - b).powi(2)).sum::<f64>() / n
}

fn media(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn citl(p: &[f64], y: &[f64]) -> f64 {
    media(p) - media(y)
}

fn habilidade(p: &[f64], y: &[f64], referencia: f64) -> f64 {
    let b_ref: f64 = y.iter().map(|v| (referencia - v).powi(2)).sum::<f64>() / y.len() as f64;
    1.0 - brier(p, y) / b_ref
}

pub fn rodar() {
    println!("== Capítulo 20 :: o relogio, o canario e o cofre ==");
    let mut rng = Sm64::novo(SEMENTE);
    let treino = gerar(N_TREINO, B0_TREINO, P_NOITE, P_VELOCIDADE, P_PLACA, &mut rng);
    let desenvolvimento = gerar(
        N_DESENVOLVIMENTO,
        B0_NOVO,
        P_NOITE,
        P_VELOCIDADE,
        P_PLACA,
        &mut rng,
    );
    let cofre = gerar(N_COFRE, B0_NOVO, P_NOITE, P_VELOCIDADE, P_PLACA, &mut rng);

    println!(
        "semente {} | treino {} paradas (taxa {:.4}) | desenvolvimento {} (taxa {:.4}) | cofre {} (taxa {:.4})",
        SEMENTE,
        N_TREINO,
        media(&treino.y),
        N_DESENVOLVIMENTO,
        media(&desenvolvimento.y),
        N_COFRE,
        media(&cofre.y)
    );

    // O modelo cru: treinado no regime antigo, aplicado ao regime novo.
    let b_cru = ajustar(&treino.x, &treino.y, 1e-4);
    let p_cru: Vec<f64> = (0..N_COFRE).map(|i| prever(&b_cru, &cofre.x, i)).collect();
    // A correção de Platt: intercepto e inclinação, ajustados no desenvolvimento.
    let etapa_dev: Vec<f64> = (0..N_DESENVOLVIMENTO)
        .map(|i| logito(prever(&b_cru, &desenvolvimento.x, i).clamp(1e-9, 1.0 - 1e-9)))
        .collect();
    let x_platt = vec![vec![1.0; N_DESENVOLVIMENTO], etapa_dev];
    let b_platt = ajustar(&x_platt, &desenvolvimento.y, 1e-6);
    let p_platt: Vec<f64> = (0..N_COFRE)
        .map(|i| {
            let s = logito(p_cru[i].clamp(1e-9, 1.0 - 1e-9));
            sigmoide(b_platt[0] + b_platt[1] * s)
        })
        .collect();

    let taxa_dev = media(&desenvolvimento.y);
    let taxa_cofre = media(&cofre.y);
    println!(
        "modelo cru (treinado no periodo antigo de taxa mais alta): media prevista {:.4} ({:.0}% acima da taxa do cofre)",
        media(&p_cru),
        100.0 * (media(&p_cru) / taxa_cofre - 1.0)
    );
    println!(
        "  CITL = {:+.4} | ECE = {:.4} | Brier = {:.5} | habilidade = {:+.4}",
        citl(&p_cru, &cofre.y),
        ece(&p_cru, &cofre.y),
        brier(&p_cru, &cofre.y),
        habilidade(&p_cru, &cofre.y, taxa_dev)
    );
    println!(
        "modelo oficial com Platt (a = {:.2}, b = {:.2}): media prevista {:.4}",
        b_platt[1], b_platt[0], media(&p_platt)
    );
    println!(
        "  CITL = {:+.4} | ECE = {:.4} | Brier = {:.5} | habilidade = {:+.4}",
        citl(&p_platt, &cofre.y),
        ece(&p_platt, &cofre.y),
        brier(&p_platt, &cofre.y),
        habilidade(&p_platt, &cofre.y, taxa_dev)
    );
    println!(
        "climatologia fixa na taxa do passado: Brier = {:.5} (p(1-p) = {:.5})",
        cofre
            .y
            .iter()
            .map(|v| (taxa_dev - v).powi(2))
            .sum::<f64>()
            / N_COFRE as f64,
        taxa_dev * (1.0 - taxa_dev)
    );

    // ------------------------------------------------------------ o canário
    // A coluna do canário é igual ao rótulo e só se torna conhecível no instante do rótulo.
    // Com o relógio honesto ela não está publicada na emissão; com o relógio desligado, está.
    println!("\ncanario: uma coluna igual ao rotulo, publicada no instante do rotulo");
    for relogio_honesto in [true, false] {
        let mut x_treino = treino.x.clone();
        let mut x_cofre = cofre.x.clone();
        let mut rng_canario = Sm64::novo(SEMENTE ^ 0xCA11_A710);
        let copia = |y: &[f64], rng: &mut Sm64| -> Vec<f64> {
            // o canário é uma cópia do rótulo com 1% de trocas: um vazamento, não a resposta perfeita
            y.iter()
                .map(|v| if rng.uniforme() < 0.01 { 1.0 - v } else { *v })
                .collect()
        };
        if relogio_honesto {
            x_treino.push(vec![0.0; N_TREINO]);
            x_cofre.push(vec![0.0; N_COFRE]);
        } else {
            x_treino.push(copia(&treino.y, &mut rng_canario));
            x_cofre.push(copia(&cofre.y, &mut rng_canario));
        }
        let b = ajustar(&x_treino, &treino.y, 1e-4);
        let p: Vec<f64> = (0..N_COFRE).map(|i| prever(&b, &x_cofre, i)).collect();
        let disponivel = if relogio_honesto { 0 } else { N_COFRE };
        println!(
            "  relogio {}: canario disponivel em {} de {} emissoes | beta_canario = {:+.2} | ganho de habilidade = {:+.4}",
            if relogio_honesto { "honesto" } else { "desligado" },
            disponivel,
            N_COFRE,
            b[4],
            habilidade(&p, &cofre.y, taxa_dev) - habilidade(&p_cru, &cofre.y, taxa_dev)
        );
    }
}
