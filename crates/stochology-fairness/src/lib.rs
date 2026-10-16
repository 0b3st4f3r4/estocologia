//! stochology-fairness — justiça por subgrupo: paridade, oportunidades e calibração.
//!
//! O contrato cobra que a decisao declare sobre quem ela pesa. Aqui vivem as tres medidas que o
//! programa usa: a paridade demografica (taxa de positivo por grupo), a igualdade de oportunidades
//! (sensibilidade por grupo) e a calibracao por regiao, que reusa a ECE do H3 — inclusive a limitacao
//! declarada de que a tolerancia da ECE depende de n.
//!
//! Declarado e nao implementado: os pesos por dano (quanto custa um falso negativo em cada grupo) e a
//! escuta de quem paga pelo erro, que e trabalho de campo (`governanca/GRUPOS.md`).

use stochology_h3::ece;

#[derive(Debug, Clone, PartialEq)]
pub struct Grupo {
    pub nome: String,
    pub n: usize,
    pub positivos: usize,
    pub verdadeiros_positivos: usize,
    pub falsos_negativos: usize,
}

impl Grupo {
    pub fn novo(
        nome: &str,
        n: usize,
        positivos: usize,
        verdadeiros_positivos: usize,
        falsos_negativos: usize,
    ) -> Self {
        Grupo {
            nome: nome.to_string(),
            n,
            positivos,
            verdadeiros_positivos,
            falsos_negativos,
        }
    }

    /// Taxa de positivo no grupo: quem foi tratado como caso.
    pub fn taxa_de_positivos(&self) -> f64 {
        if self.n == 0 {
            return 0.0;
        }
        self.positivos as f64 / self.n as f64
    }

    /// Sensibilidade: entre os que de fato eram caso, quantos foram atendidos.
    pub fn sensibilidade(&self) -> f64 {
        let casos = self.verdadeiros_positivos + self.falsos_negativos;
        if casos == 0 {
            return 0.0;
        }
        self.verdadeiros_positivos as f64 / casos as f64
    }
}

fn amplitude(valores: Vec<f64>) -> f64 {
    if valores.len() < 2 {
        return 0.0;
    }
    let maior = valores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let menor = valores.iter().cloned().fold(f64::INFINITY, f64::min);
    maior - menor
}

/// Paridade demografica: a distancia entre a maior e a menor taxa de positivo.
pub fn paridade_demografica(grupos: &[Grupo]) -> f64 {
    amplitude(grupos.iter().map(Grupo::taxa_de_positivos).collect())
}

/// Igualdade de oportunidades: a distancia entre a maior e a menor sensibilidade.
pub fn igualdade_de_oportunidades(grupos: &[Grupo]) -> f64 {
    amplitude(grupos.iter().map(Grupo::sensibilidade).collect())
}

/// ECE de uma regiao declarada da amostra (o subgrupo que paga pelo erro).
pub fn ece_do_grupo(confianca: &[f64], acerto: &[bool], baldes: usize, regiao: &[bool]) -> f64 {
    if confianca.len() != acerto.len() || confianca.len() != regiao.len() {
        return f64::NAN;
    }
    let mut p = Vec::new();
    let mut y = Vec::new();
    for i in 0..confianca.len() {
        if regiao[i] {
            p.push(confianca[i]);
            y.push(acerto[i]);
        }
    }
    ece(&p, &y, baldes)
}

/// Tolerancia declarada da ECE para n pontos: cai com a raiz de n. Nao existe numero unico: existe
/// esta regra, que e o que o capitulo 15 e 20 deixam no lugar de um valor fixo.
pub fn tolerancia_do_ece(n: usize) -> f64 {
    if n == 0 {
        return f64::INFINITY;
    }
    1.5 / (n as f64).sqrt()
}

#[cfg(test)]
mod testes {
    use super::*;
    use stochology_core::Rng;

    #[test]
    fn grupos_iguais_nao_tem_amplitude() {
        let a = Grupo::novo("a", 100, 30, 24, 6);
        let b = Grupo::novo("b", 200, 60, 48, 12);
        assert!(paridade_demografica(&[a.clone(), b.clone()]).abs() < 1e-12);
        assert!(igualdade_de_oportunidades(&[a, b]).abs() < 1e-12);
        assert_eq!(paridade_demografica(&[]), 0.0);
    }

    #[test]
    fn a_paridade_mede_o_gap_injetado() {
        let a = Grupo::novo("a", 100, 40, 30, 10);
        let b = Grupo::novo("b", 100, 20, 10, 10);
        assert!((paridade_demografica(&[a.clone(), b.clone()]) - 0.20).abs() < 1e-12);
        assert!((igualdade_de_oportunidades(&[a, b]) - 0.25).abs() < 1e-12);
    }

    #[test]
    fn a_calibracao_acha_a_regiao_descalibrada() {
        let mut rng = Rng::novo(2026);
        let mut confianca = Vec::new();
        let mut acerto = Vec::new();
        let mut regiao = Vec::new();
        for i in 0..20_000 {
            let u = ((rng.proximo() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
            let p = u;
            let pertence = i % 2 == 0;
            let sorte = ((rng.proximo() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
            // o grupo de dentro acerta menos do que promete; o de fora e calibrado
            let acertou = if pertence { sorte < p * 0.6 } else { sorte < p };
            confianca.push(p);
            acerto.push(acertou);
            regiao.push(pertence);
        }
        let dentro = ece_do_grupo(&confianca, &acerto, 20, &regiao);
        let fora: Vec<bool> = regiao.iter().map(|r| !r).collect();
        let de_fora = ece_do_grupo(&confianca, &acerto, 20, &fora);
        assert!(de_fora < 0.03, "fora e calibrado: {de_fora}");
        assert!(dentro > 0.08, "dentro e descalibrado: {dentro}");
    }

    #[test]
    fn a_tolerancia_da_ece_cai_com_a_raiz_de_n() {
        let base = tolerancia_do_ece(100);
        assert!((tolerancia_do_ece(400) - base / 2.0).abs() < 1e-12);
        assert!((tolerancia_do_ece(1_600) - base / 4.0).abs() < 1e-12);
        assert!(tolerancia_do_ece(0).is_infinite());
    }
}
