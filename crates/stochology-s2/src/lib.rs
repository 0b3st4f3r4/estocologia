//! stochology-s2 — o deliberativo: LATS e o motor de estado de crenca.
//!
//! O Sistema 1 (triagem deterministica) vive em `stochology-core/src/s1.rs`. Este crate e o Sistema 2
//! que o PLAN.md §4 e o capitulo 13 declaravam e nao existia: busca em arvore com estimativa
//! (LATS) sobre um mundo declarado, e o estado de crenca sobre o hipergrafo.
//!
//! Duas regras do contrato entram aqui em codigo: a decisao e deterministica dada a semente (Phi1), e
//! nenhuma probabilidade do estado de crenca bate em 0% nem em 100%.
//!
//! Declarado e nao implementado: a ligacao direta com `stochology-hypergraph` (o estado de crenca
//! aceita qualquer nome de hipotese hoje) e o orcamento de computacao por tempo de parede.

use stochology_core::Rng;

/// Piso e teto do contrato: nenhuma probabilidade e 0 nem 1.
pub const PISO: f64 = 0.01;
pub const TETO: f64 = 0.99;

/// O mundo declarado sobre o qual a deliberacao acontece.
pub trait Mundo {
    fn acoes(&self, estado: &str) -> Vec<String>;
    fn passo(&self, estado: &str, acao: &str) -> String;
    fn avaliar(&self, estado: &str) -> f64;
    fn terminal(&self, estado: &str) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Decisao {
    pub acao: String,
    pub valor: f64,
    pub nos_visitados: usize,
    pub trilha: Vec<String>,
}

/// Busca em arvore com estimativa: selecao por UCB, rollout deterministico, media da folha.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lats {
    pub profundidade: usize,
    pub orcamento: usize,
    pub exploracao: f64,
    pub semente: u64,
}

impl Default for Lats {
    fn default() -> Self {
        Lats { profundidade: 3, orcamento: 40, exploracao: 1.0, semente: 2026 }
    }
}

impl Lats {
    pub fn deliberar<M: Mundo>(&self, mundo: &M, raiz: &str) -> Option<Decisao> {
        let acoes = mundo.acoes(raiz);
        if acoes.is_empty() {
            return None;
        }
        let mut rng = Rng::novo(self.semente);
        let mut estatisticas = vec![(0u32, 0.0_f64); acoes.len()];
        for _ in 0..self.orcamento.max(1) {
            let total: u32 = estatisticas.iter().map(|(n, _)| *n).sum();
            let mut escolhida = 0usize;
            let mut melhor = f64::NEG_INFINITY;
            for (i, (n, soma)) in estatisticas.iter().enumerate() {
                let ucb = if *n == 0 {
                    f64::INFINITY
                } else {
                    soma / *n as f64
                        + self.exploracao * (((total as f64 + 1.0).ln()) / *n as f64).sqrt()
                };
                if ucb > melhor {
                    melhor = ucb;
                    escolhida = i;
                }
            }
            let mut estado = mundo.passo(raiz, &acoes[escolhida]);
            let mut trilha = vec![acoes[escolhida].clone()];
            for _ in 1..self.profundidade.max(1) {
                if mundo.terminal(&estado) {
                    break;
                }
                let proximas = mundo.acoes(&estado);
                if proximas.is_empty() {
                    break;
                }
                let i = rng.uniforme(proximas.len() as u64) as usize;
                trilha.push(proximas[i].clone());
                estado = mundo.passo(&estado, &proximas[i]);
            }
            let valor = mundo.avaliar(&estado);
            estatisticas[escolhida].0 += 1;
            estatisticas[escolhida].1 += valor;
        }
        let (indice, (visitas, soma)) = estatisticas
            .iter()
            .enumerate()
            .max_by_key(|(i, (n, _))| (*n, std::cmp::Reverse(*i)))
            .map(|(i, e)| (i, *e))?;
        Some(Decisao {
            acao: acoes[indice].clone(),
            valor: if visitas == 0 { 0.0 } else { soma / visitas as f64 },
            nos_visitados: visitas as usize,
            trilha: vec![acoes[indice].clone()],
        })
    }
}

/// O que um agente ganancioso faz: a acao de maior valor imediato, sem olhar adiante.
pub fn ganancioso<M: Mundo>(mundo: &M, raiz: &str) -> Option<Decisao> {
    let acoes = mundo.acoes(raiz);
    let mut melhor: Option<Decisao> = None;
    for acao in acoes {
        let estado = mundo.passo(raiz, &acao);
        let valor = mundo.avaliar(&estado);
        let candidata = Decisao { acao, valor, nos_visitados: 1, trilha: Vec::new() };
        if melhor.as_ref().map(|m| valor > m.valor).unwrap_or(true) {
            melhor = Some(candidata);
        }
    }
    melhor
}

/// Estado de crenca: peso por hipotese, com o piso e o teto do contrato.
#[derive(Debug, Clone, PartialEq)]
pub struct EstadoCrenca {
    pesos: Vec<(String, f64)>,
}

impl EstadoCrenca {
    pub fn novo(hipoteses: &[&str]) -> Self {
        let peso = 1.0 / hipoteses.len().max(1) as f64;
        EstadoCrenca {
            pesos: hipoteses.iter().map(|h| ((*h).to_string(), peso)).collect(),
        }
    }

    pub fn probabilidade(&self, hipotese: &str) -> Option<f64> {
        let soma: f64 = self.pesos.iter().map(|(_, p)| *p).sum();
        self.pesos
            .iter()
            .find(|(h, _)| h == hipotese)
            .map(|(_, p)| if soma > 0.0 { (p / soma).clamp(PISO, TETO) } else { 0.0 })
    }

    /// Atualizacao bayesiana: multiplica pelo peso da evidencia e reapresa no intervalo do contrato.
    pub fn observar(&mut self, evidencia: &[(&str, f64)]) {
        for (hipotese, verossimilhanca) in evidencia {
            if let Some((_, peso)) = self.pesos.iter_mut().find(|(h, _)| h == hipotese) {
                *peso *= verossimilhanca.max(0.0);
            }
        }
        let soma: f64 = self.pesos.iter().map(|(_, p)| *p).sum();
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
        let soma: f64 = self.pesos.iter().map(|(_, p)| *p).sum();
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

    pub fn hipoteses(&self) -> Vec<&str> {
        self.pesos.iter().map(|(h, _)| h.as_str()).collect()
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    /// Mundo de brinquedo: o premio imediato e menor que o premio de dois passos.
    struct DoisCaminhos;

    impl Mundo for DoisCaminhos {
        fn acoes(&self, estado: &str) -> Vec<String> {
            match estado {
                "raiz" => vec!["imediato".into(), "adiado".into()],
                "adiado" => vec!["avanca".into()],
                _ => Vec::new(),
            }
        }

        fn passo(&self, estado: &str, acao: &str) -> String {
            match (estado, acao) {
                ("raiz", "imediato") => "fim_imediato".into(),
                ("raiz", "adiado") => "adiado".into(),
                ("adiado", "avanca") => "fim_adiado".into(),
                _ => estado.to_string(),
            }
        }

        fn avaliar(&self, estado: &str) -> f64 {
            match estado {
                "fim_imediato" => 0.35,
                "adiado" => 0.20,
                "fim_adiado" => 0.90,
                _ => 0.0,
            }
        }

        fn terminal(&self, estado: &str) -> bool {
            estado.starts_with("fim_")
        }
    }

    #[test]
    fn a_deliberacao_bate_o_ganancioso() {
        let mundo = DoisCaminhos;
        let lats = Lats { profundidade: 2, orcamento: 40, exploracao: 1.0, semente: 2026 };
        let decisao = lats.deliberar(&mundo, "raiz").expect("decide");
        let guloso = ganancioso(&mundo, "raiz").expect("decide");
        assert_eq!(guloso.acao, "imediato");
        assert_eq!(decisao.acao, "adiado");
        assert!(decisao.valor > guloso.valor, "{} contra {}", decisao.valor, guloso.valor);
    }

    #[test]
    fn a_decisao_e_deterministica_na_semente() {
        let mundo = DoisCaminhos;
        let lats = Lats::default();
        let primeira = lats.deliberar(&mundo, "raiz").expect("decide");
        let segunda = lats.deliberar(&mundo, "raiz").expect("decide");
        assert_eq!(primeira, segunda);
        let outra = Lats { semente: 7, ..lats }.deliberar(&mundo, "raiz").expect("decide");
        assert_eq!(outra.acao, primeira.acao);
    }

    #[test]
    fn mundo_sem_acao_nao_decide() {
        struct Vazio;
        impl Mundo for Vazio {
            fn acoes(&self, _: &str) -> Vec<String> { Vec::new() }
            fn passo(&self, e: &str, _: &str) -> String { e.to_string() }
            fn avaliar(&self, _: &str) -> f64 { 0.0 }
            fn terminal(&self, _: &str) -> bool { true }
        }
        assert!(Lats::default().deliberar(&Vazio, "raiz").is_none());
        assert!(ganancioso(&Vazio, "raiz").is_none());
    }

    #[test]
    fn o_estado_de_crenca_nunca_bate_no_zero_nem_no_um() {
        let mut crenca = EstadoCrenca::novo(&["a", "b", "c"]);
        for _ in 0..200 {
            crenca.observar(&[("a", 3.0)]);
        }
        let pa = crenca.probabilidade("a").expect("existe");
        let pb = crenca.probabilidade("b").expect("existe");
        assert!(pa <= TETO + 1e-12 && pa >= PISO - 1e-12, "a em {pa}");
        assert!(pb <= TETO + 1e-12 && pb >= PISO - 1e-12, "b em {pb}");
        assert_eq!(crenca.mais_provavel(), Some("a"));
        assert!(crenca.entropia() > 0.0);
    }

    #[test]
    fn evidencia_invalida_nao_zera_o_estado() {
        let mut crenca = EstadoCrenca::novo(&["a", "b"]);
        crenca.observar(&[("a", 0.0), ("b", 0.0)]);
        let pa = crenca.probabilidade("a").expect("existe");
        assert!((pa - 0.5).abs() < 1e-12, "sem evidencia util volta ao uniforme: {pa}");
    }
}
