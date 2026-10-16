//! O ajuste Beta dos três quartis — a família do domínio \`h1-real/c\` (uma frequência).
//!
//! O protocolo de \`fundo/p12-shelf\` diz que o H1 real ajusta **Beta** aos três quartis centrais, porque a
//! quantidade é uma frequência em (0, 1) e a Normal não respeita o suporte. Aqui vivem a função Beta incompleta
//! regularizada (fração continuada de Lentz, padrão Numerical Recipes), o quantil por bisseção e o ajuste por
//! busca determinística em (log a, log b).
//!
//! Declarado e não implementado: intervalo de confiança para os parâmetros e o ajuste por máxima verossimilhança —
//! o critério aqui é o erro quadrático nos três quantis, como o protocolo declara.

/// Logaritmo do gama, por Lanczos (z > 0) com reflexão para z < 0,5.
pub fn ln_gama(z: f64) -> f64 {
    const G: [f64; 9] = [
        0.999_999_999_999_809_93,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    if z < 0.5 {
        // Gamma(z) Gamma(1-z) = pi / sin(pi z), tudo em logaritmo
        (std::f64::consts::PI / (std::f64::consts::PI * z).sin()).ln() - ln_gama(1.0 - z)
    } else {
        let z = z - 1.0;
        let mut x = G[0];
        for (i, g) in G.iter().enumerate().skip(1) {
            x += g / (z + i as f64);
        }
        let t = z + 7.5;
        0.5 * (2.0 * std::f64::consts::PI).ln() + (z + 0.5) * t.ln() - t + x.ln()
    }
}

fn betacf(a: f64, b: f64, x: f64) -> f64 {
    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;
    let mut c = 1.0_f64;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < 1e-30 {
        d = 1e-30;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..=300 {
        let mf = m as f64;
        let m2 = 2.0 * mf;
        // passo par
        let aa = mf * (b - mf) * x / ((qam + m2) * (a + m2));
        d = 1.0 + aa * d;
        if d.abs() < 1e-30 {
            d = 1e-30;
        }
        c = 1.0 + aa / c;
        if c.abs() < 1e-30 {
            c = 1e-30;
        }
        d = 1.0 / d;
        h *= d * c;
        // passo ímpar
        let aa = -(a + mf) * (qab + mf) * x / ((a + m2) * (a + m2 + 1.0));
        d = 1.0 + aa * d;
        if d.abs() < 1e-30 {
            d = 1e-30;
        }
        c = 1.0 + aa / c;
        if c.abs() < 1e-30 {
            c = 1e-30;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < 1e-12 {
            break;
        }
    }
    h
}

/// A função Beta incompleta regularizada \`I_x(a, b)\`.
pub fn beta_incompleta(a: f64, b: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let ln_bt = ln_gama(a + b) - ln_gama(a) - ln_gama(b) + a * x.ln() + b * (1.0 - x).ln();
    let bt = ln_bt.exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        bt * betacf(a, b, x) / a
    } else {
        1.0 - bt * betacf(b, a, 1.0 - x) / b
    }
}

/// O quantil da Beta: \`x\` tal que \`I_x(a, b) = p\`, por bisseção determinística.
pub fn beta_quantil(p: f64, a: f64, b: f64) -> f64 {
    let (mut baixo, mut alto) = (0.0_f64, 1.0_f64);
    for _ in 0..200 {
        let meio = 0.5 * (baixo + alto);
        if beta_incompleta(a, b, meio) < p {
            baixo = meio;
        } else {
            alto = meio;
        }
    }
    0.5 * (baixo + alto)
}

/// Os dois parâmetros ajustados e o erro máximo nos três quartis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrioriBeta {
    pub a: f64,
    pub b: f64,
    pub erro_max_quartis: f64,
}

/// Ajusta a Beta aos três quartis centrais, por grade grosseira e refino local — determinístico.
pub fn ajustar_beta(q25: f64, q50: f64, q75: f64) -> PrioriBeta {
    let erro = |a: f64, b: f64| -> f64 {
        [(q25, 0.25), (q50, 0.5), (q75, 0.75)]
            .iter()
            .map(|(q, p)| (beta_quantil(*p, a, b) - q).abs())
            .fold(0.0_f64, f64::max)
    };
    let (mut melhor_a, mut melhor_b, mut melhor_erro) = (1.0_f64, 1.0_f64, erro(1.0, 1.0));
    for ia in 0..34 {
        let a = 0.2 * 1.3_f64.powi(ia);
        for ib in 0..34 {
            let b = 0.2 * 1.3_f64.powi(ib);
            let e = erro(a, b);
            if e < melhor_erro {
                melhor_a = a;
                melhor_b = b;
                melhor_erro = e;
            }
        }
    }
    for _ in 0..14 {
        let (a, b) = (melhor_a, melhor_b);
        for da in [-2.0, -1.0, 0.0, 1.0, 2.0] {
            for db in [-2.0, -1.0, 0.0, 1.0, 2.0] {
                let ca = (a * (1.0 + 0.04 * da)).clamp(0.01, 5_000.0);
                let cb = (b * (1.0 + 0.04 * db)).clamp(0.01, 5_000.0);
                let e = erro(ca, cb);
                if e < melhor_erro {
                    melhor_a = ca;
                    melhor_b = cb;
                    melhor_erro = e;
                }
            }
        }
    }
    PrioriBeta { a: melhor_a, b: melhor_b, erro_max_quartis: melhor_erro }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn o_log_gama_bate_com_fatoriais() {
        for n in 1..=10u32 {
            let obtido = ln_gama(n as f64 + 1.0);
            let esperado = (1..=n).map(|k| (k as f64).ln()).sum::<f64>();
            assert!((obtido - esperado).abs() < 1e-9, "ln Gama({n}+1): {obtido} contra {esperado}");
        }
    }

    #[test]
    fn a_incompleta_bate_com_a_uniforme() {
        for x in [0.1_f64, 0.35, 0.5, 0.9] {
            let v = beta_incompleta(1.0, 1.0, x);
            assert!((v - x).abs() < 1e-9, "I_{x}(1,1) = {v}");
        }
    }

    #[test]
    fn a_incompleta_bate_com_a_forma_fechada_da_beta_2_2() {
        // I_x(2,2) = 3x^2 - 2x^3
        for x in [0.2_f64, 0.5, 0.8] {
            let v = beta_incompleta(2.0, 2.0, x);
            let exato = 3.0 * x * x - 2.0 * x * x * x;
            assert!((v - exato).abs() < 1e-9, "I_{x}(2,2): {v} contra {exato}");
        }
    }

    #[test]
    fn o_quantil_recupera_os_quartis_conhecidos() {
        let q25 = beta_quantil(0.25, 2.0, 5.0);
        let q50 = beta_quantil(0.50, 2.0, 5.0);
        let q75 = beta_quantil(0.75, 2.0, 5.0);
        assert!(q25 < q50 && q50 < q75, "{q25} {q50} {q75}");
        assert!((q50 - 0.2634).abs() < 0.005, "mediana da Beta(2,5): {q50}");
        assert!((beta_quantil(0.5, 1.0, 1.0) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn o_ajuste_recupera_a_beta_que_gerou_os_quartis() {
        let p = ajustar_beta(
            beta_quantil(0.25, 2.0, 5.0),
            beta_quantil(0.50, 2.0, 5.0),
            beta_quantil(0.75, 2.0, 5.0),
        );
        assert!((p.a - 2.0).abs() < 0.2, "a = {}", p.a);
        assert!((p.b - 5.0).abs() < 0.5, "b = {}", p.b);
        assert!(p.erro_max_quartis < 0.01, "erro {}", p.erro_max_quartis);
    }

    #[test]
    fn o_ajuste_da_uniforme_fica_simetrico() {
        let p = ajustar_beta(0.25, 0.50, 0.75);
        assert!((p.a - p.b).abs() < 0.1, "a {} b {}", p.a, p.b);
        assert!(p.erro_max_quartis < 0.01, "erro {}", p.erro_max_quartis);
    }

    #[test]
    fn o_ajuste_e_deterministico() {
        assert_eq!(ajustar_beta(0.1, 0.3, 0.6), ajustar_beta(0.1, 0.3, 0.6));
    }
}
