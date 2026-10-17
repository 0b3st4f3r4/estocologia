//! O motor de estado de crenca: o que o Sistema 2 acredita, com piso e teto do contrato.

use stochology_core::{AgenteA, Aprovacao, Ficha, SEVERIDADE_MINIMA};

/// Piso epistemologico: nenhuma probabilidade desce a 0%.
pub const PISO: f64 = 0.01;

/// Teto epistemologico: nenhuma probabilidade sobe a 100%.
pub const TETO: f64 = 0.99;

/// Estado de crenca: peso por hipotese, preso em [PISO, TETO] em toda leitura.
#[derive(Debug, Clone, PartialEq)]
pub struct EstadoCrenca {
    pesos: Vec<(String, f64)>,
}

impl EstadoCrenca {
    pub fn novo(hipoteses: &[&str]) -> Self {
        EstadoCrenca::de_nomes(hipoteses.iter().map(|h| (*h).to_string()).collect())
    }

    pub fn de_nomes(hipoteses: Vec<String>) -> Self {
        let peso = 1.0 / hipoteses.len().max(1) as f64;
        EstadoCrenca {
            pesos: hipoteses.into_iter().map(|h| (h, peso)).collect(),
        }
    }

    pub fn hipoteses(&self) -> Vec<&str> {
        self.pesos.iter().map(|(h, _)| h.as_str()).collect()
    }

    fn soma(&self) -> f64 {
        self.pesos.iter().map(|(_, p)| *p).sum()
    }

    pub fn probabilidade(&self, hipotese: &str) -> Option<f64> {
        let soma = self.soma();
        self.pesos
            .iter()
            .find(|(h, _)| h == hipotese)
            .map(|(_, p)| {
                if soma > 0.0 {
                    (p / soma).clamp(PISO, TETO)
                } else {
                    0.0
                }
            })
    }

    /// Atualizacao bayesiana: multiplica pelo peso da evidencia e reapresa no intervalo do contrato.
    pub fn observar(&mut self, evidencia: &[(&str, f64)]) {
        for (hipotese, verossimilhanca) in evidencia {
            if let Some((_, peso)) = self.pesos.iter_mut().find(|(h, _)| h == hipotese) {
                *peso *= verossimilhanca.max(0.0);
            }
        }
        self.reapresar();
    }

    /// Evidencia de ficha: reforca as hipoteses que casam com a camada e com o tipo de incerteza.
    pub fn observar_ficha(&mut self, ficha: &Ficha, peso: f64) {
        let alvos: Vec<(String, f64)> = vec![
            (ficha.camada.codigo().to_string(), peso),
            (ficha.incerteza.codigo().to_string(), peso),
        ];
        for (hipotese, w) in alvos {
            if let Some((_, p)) = self.pesos.iter_mut().find(|(h, _)| *h == hipotese) {
                *p *= w.max(0.0);
            }
        }
        self.reapresar();
    }

    /// Evidencia de auditoria: sobe o peso de quem reprovou a ficha.
    pub fn observar_aprovacao(&mut self, aprovacao: &Aprovacao, peso: f64) {
        let reprovados: Vec<(String, f64)> = aprovacao
            .itens
            .iter()
            .filter(|(_, s)| *s < SEVERIDADE_MINIMA)
            .map(|(a, _)| (a.nome().to_string(), peso))
            .collect();
        for (hipotese, w) in reprovados {
            if let Some((_, p)) = self.pesos.iter_mut().find(|(h, _)| *h == hipotese) {
                *p *= w.max(0.0);
            }
        }
        self.reapresar();
    }

    fn reapresar(&mut self) {
        let soma = self.soma();
        if soma <= 0.0 {
            let uniforme = 1.0 / self.pesos.len().max(1) as f64;
            for (_, peso) in self.pesos.iter_mut() {
                *peso = uniforme;
            }
            return;
        }
        for (_, peso) in self.pesos.iter_mut() {
            *peso = (*peso / soma).clamp(PISO, TETO);
        }
    }

    pub fn entropia(&self) -> f64 {
        let soma = self.soma();
        if soma <= 0.0 {
            return 0.0;
        }
        -self
            .pesos
            .iter()
            .map(|(_, p)| {
                let q = (p / soma).clamp(PISO, TETO);
                q * q.ln()
            })
            .sum::<f64>()
    }

    pub fn mais_provavel(&self) -> Option<&str> {
        self.pesos
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(h, _)| h.as_str())
    }

    /// As hipoteses em ordem decrescente de peso; empate desempata pela ordem declarada.
    pub fn ranking(&self) -> Vec<(String, f64)> {
        let mut itens: Vec<usize> = (0..self.pesos.len()).collect();
        itens.sort_by(|i, j| {
            self.pesos[*j]
                .1
                .partial_cmp(&self.pesos[*i].1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(i.cmp(j))
        });
        itens
            .into_iter()
            .map(|i| {
                (
                    self.pesos[i].0.clone(),
                    self.probabilidade(&self.pesos[i].0).unwrap_or(0.0),
                )
            })
            .collect()
    }
}

/// A crenca do estresse adversarial: o espaco de hipoteses sao as oito faixas A1 a A8.
pub fn crenca_da_banca() -> EstadoCrenca {
    EstadoCrenca::de_nomes(
        AgenteA::TODOS
            .iter()
            .map(|a| a.nome().to_string())
            .collect(),
    )
}

#[cfg(test)]
mod testes {
    use super::*;
    use stochology_core::{Camada, Ficha, TipoIncerteza};

    #[test]
    fn o_estado_de_crenca_nunca_bate_no_zero_nem_no_um() {
        let mut crenca = EstadoCrenca::novo(&["a", "b", "c"]);
        for _ in 0..200 {
            crenca.observar(&[("a", 3.0)]);
        }
        let pa = crenca.probabilidade("a").expect("existe");
        let pb = crenca.probabilidade("b").expect("existe");
        assert!((PISO - 1e-12..=TETO + 1e-12).contains(&pa), "a em {pa}");
        assert!((PISO - 1e-12..=TETO + 1e-12).contains(&pb), "b em {pb}");
        assert_eq!(crenca.mais_provavel(), Some("a"));
        assert!(crenca.entropia() > 0.0);
    }

    #[test]
    fn evidencia_invalida_nao_zera_o_estado() {
        let mut crenca = EstadoCrenca::novo(&["a", "b"]);
        crenca.observar(&[("a", 0.0), ("b", 0.0)]);
        let pa = crenca.probabilidade("a").expect("existe");
        assert!(
            (pa - 0.5).abs() < 1e-12,
            "sem evidencia util volta ao uniforme: {pa}"
        );
    }

    #[test]
    fn a_evidencia_de_ficha_move_a_crenca_para_a_camada_da_ficha() {
        let mut crenca = EstadoCrenca::novo(&["L1", "L2", "L3", "aleatoria", "epistemica"]);
        let ficha = Ficha::nova("s2.1", Camada::Agente, TipoIncerteza::Epistemica);
        for _ in 0..5 {
            crenca.observar_ficha(&ficha, 3.0);
        }
        // as duas hipoteses que casam com a ficha (L3 e epistemica) sobem juntas, e empatam
        let topo = crenca.mais_provavel().expect("tem topo");
        assert!(topo == "L3" || topo == "epistemica", "topo veio {topo}");
        assert!(
            crenca.probabilidade("L3").expect("existe")
                > crenca.probabilidade("L1").expect("existe")
        );
        assert!(
            crenca.probabilidade("epistemica").expect("existe")
                > crenca.probabilidade("aleatoria").expect("existe")
        );
        let ranking = crenca.ranking();
        assert!(ranking[0].1 > ranking.last().expect("tem ultimo").1);
    }

    #[test]
    fn o_ranking_desempata_pela_ordem_declarada() {
        let crenca = EstadoCrenca::novo(&["primeiro", "segundo", "terceiro"]);
        let ranking = crenca.ranking();
        assert_eq!(ranking[0].0, "primeiro");
        assert_eq!(ranking[2].0, "terceiro");
    }
}
