//! Elicitação SHELF: o ajuste da priori pelos quartis e os controles do protocolo.
//!
//! O protocolo de `fundo/p12-shelf` elicita mediana, quartis e extremos, e depois ajusta uma Normal por
//! mínimos quadrados em q25/q50/q75 (a família dos quatro domínios gaussianos). Aqui vivem o ajuste, a
//! largura 90% elicitada contra a implicada (o controle de excesso de confiança, razão ≥ 0,75), o erro
//! máximo nos quartis e a leitura da âncora.
//!
//! Declarado e não implementado: o ajuste Beta do domínio `h1-real/c` (frequência) e a escala de razão de
//! chances do domínio `h2/direito_prisoes`, que entram quando esses domínios forem elicitados.

/// z da normal padrão em 0,25 e em 0,75: o ajuste usa os três quartis centrais.
pub const Z_QUARTIL: f64 = 0.674_489_750_196_081_7;

/// z da normal padrão em 0,95: entra na largura 90% implicada.
pub const Z_CENTRAL_90: f64 = 1.644_853_626_951_472_2;

/// Razão mínima declarada pelo protocolo (controle de excesso de confiança).
pub const RAZAO_LARGURA_MINIMA: f64 = 0.75;

/// Mediana esperada na leitura da âncora com S = a mediana do próprio especialista.
pub const ANCORA_ESPERADA: f64 = 0.50;

/// Tolerância declarada da âncora.
pub const ANCORA_TOLERANCIA: f64 = 0.10;

/// Deslocamento máximo tolerado da mediana, em unidades do dp da própria priori.
pub const DESLOCAMENTO_MAXIMO_DP: f64 = 0.20;

/// A priori Normal que os três quartis centrais implicam.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrioriNormal {
    pub media: f64,
    pub dp: f64,
    /// O maior desvio entre um quartil elicitado e o quartil que a própria Normal ajustada prevê.
    pub erro_max_quartis: f64,
}

/// Ajusta a Normal por mínimos quadrados nos três quartis centrais.
pub fn ajustar_normal(q25: f64, q50: f64, q75: f64) -> PrioriNormal {
    // pontos (z, q): (-z, q25), (0, q50), (+z, q75); o mínimo quadrado dá inclinação e intercepto
    let zs = [-Z_QUARTIL, 0.0, Z_QUARTIL];
    let qs = [q25, q50, q75];
    let n = 3.0_f64;
    let soma_z: f64 = zs.iter().sum();
    let soma_q: f64 = qs.iter().sum();
    let soma_zz: f64 = zs.iter().map(|z| z * z).sum();
    let soma_zq: f64 = zs.iter().zip(qs.iter()).map(|(z, q)| z * q).sum();
    let dp = (soma_zq - soma_z * soma_q / n) / (soma_zz - soma_z * soma_z / n);
    let media = soma_q / n - dp * soma_z / n;
    let erro_max_quartis = zs
        .iter()
        .zip(qs.iter())
        .map(|(z, q)| (q - (media + dp * z)).abs())
        .fold(0.0_f64, f64::max);
    PrioriNormal { media, dp, erro_max_quartis }
}

/// Largura 90% elicitada: a distância entre os dois extremos.
pub fn largura_90_elicitada(q05: f64, q95: f64) -> f64 {
    q95 - q05
}

/// Largura 90% que a própria Normal ajustada implica.
pub fn largura_90_implicada(dp: f64) -> f64 {
    2.0 * Z_CENTRAL_90 * dp
}

/// A razão do controle de excesso de confiança.
pub fn razao_largura_90(largura_elicitada: f64, dp: f64) -> f64 {
    let implicada = largura_90_implicada(dp);
    if implicada <= 0.0 {
        return f64::NAN;
    }
    largura_elicitada / implicada
}

pub fn excesso_de_confianca_ok(razao: f64) -> bool {
    razao >= RAZAO_LARGURA_MINIMA
}

/// A âncora: `P(theta > S)` com `S` igual à mediana do próprio especialista deve dar 0,50 ± 0,10.
pub fn ancora_na_mediana_ok(p: f64) -> bool {
    (p - ANCORA_ESPERADA).abs() <= ANCORA_TOLERANCIA
}

/// O deslocamento da mediana, em unidades do dp, quando se move a mediana do especialista.
pub fn deslocamento_em_dp(mediana_nova: f64, mediana_original: f64, dp: f64) -> f64 {
    if dp <= 0.0 {
        return f64::NAN;
    }
    (mediana_nova - mediana_original).abs() / dp
}

pub fn deslocamento_dentro_do_limite(deslocamento_dp: f64) -> bool {
    deslocamento_dp <= DESLOCAMENTO_MAXIMO_DP
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn recupera_a_normal_que_gerou_os_quartis() {
        let (media, dp) = (4.5_f64, 1.853_253_2_f64);
        let p = ajustar_normal(
            media - Z_QUARTIL * dp,
            media,
            media + Z_QUARTIL * dp,
        );
        assert!((p.media - media).abs() < 1e-12, "media {}", p.media);
        assert!((p.dp - dp).abs() < 1e-12, "dp {}", p.dp);
        assert!(p.erro_max_quartis < 1e-12, "erro {}", p.erro_max_quartis);
    }

    #[test]
    fn quartis_assimetricos_deixam_erro_visivel() {
        let p = ajustar_normal(3.0, 4.5, 5.75);
        assert!(p.erro_max_quartis > 0.0);
        assert!(p.erro_max_quartis < 0.3, "erro {}", p.erro_max_quartis);
        assert!(p.dp > 0.0);
    }

    #[test]
    fn a_largura_implicada_bate_a_elicitada_quando_os_extremos_sao_da_normal() {
        let dp = 1.5_f64;
        let q05 = 4.5 - Z_CENTRAL_90 * dp;
        let q95 = 4.5 + Z_CENTRAL_90 * dp;
        let razao = razao_largura_90(largura_90_elicitada(q05, q95), dp);
        assert!((razao - 1.0).abs() < 1e-12, "razao {razao}");
        assert!(excesso_de_confianca_ok(razao));
    }

    #[test]
    fn extremos_estreitos_reprovam_no_controle() {
        let dp = 2.0_f64;
        let implicada = largura_90_implicada(dp);
        let estreita = 0.5 * implicada;
        let razao = razao_largura_90(estreita, dp);
        assert!(!excesso_de_confianca_ok(razao), "razao {razao}");
        assert!((razao - 0.5).abs() < 1e-12);
    }

    #[test]
    fn a_ancora_e_o_deslocamento_tem_limites_declarados() {
        assert!(ancora_na_mediana_ok(0.50));
        assert!(ancora_na_mediana_ok(0.59));
        assert!(!ancora_na_mediana_ok(0.65));
        assert!(deslocamento_dentro_do_limite(0.19));
        assert!(!deslocamento_dentro_do_limite(0.21));
        assert!((deslocamento_em_dp(4.9, 4.5, 2.0) - 0.2).abs() < 1e-12);
    }

    #[test]
    fn o_ajuste_simetrico_da_media_no_q50() {
        let p = ajustar_normal(3.25, 4.5, 5.75);
        assert!((p.media - 4.5).abs() < 1e-12, "media {}", p.media);
        let esperado = (Z_QUARTIL * (5.75 - 3.25)) / (2.0 * Z_QUARTIL * Z_QUARTIL);
        assert!((p.dp - esperado).abs() < 1e-12);
    }
}
