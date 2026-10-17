//! Operador local de superposicao: o Phi de uma hiperaresta, avaliado como fasor.
//!
//! Cada vertice entra com amplitude, fase, condutancia topologica e atraso de propagacao. A
//! hiperaresta nao soma escalares: soma fasores. A energia resultante tem dois termos, a soma
//! incoerente das energias e o termo cruzado de interferencia, e o indice de coerencia decide
//! qual dos dois regimes de deliberacao responde.

use crate::complexo::Complexo;

/// Limiares do portao cognitivo de uma hiperaresta.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Limiares {
    /// Abaixo disto nao ha sinal suficiente: regime quiescente.
    pub ativacao: f64,
    /// Coerencia a partir da qual a hiperaresta propaga direto (Sistema 1).
    pub ressonancia: f64,
    /// Coerencia abaixo da qual, havendo energia, a hiperaresta abre o Sistema 2.
    pub critico: f64,
    /// Soma de amplitudes minimas para que a antifase conte como conflito e nao como ruido.
    pub soma_critica: f64,
}

impl Default for Limiares {
    fn default() -> Self {
        Self {
            ativacao: 0.15,
            ressonancia: 0.85,
            critico: 0.35,
            soma_critica: 0.80,
        }
    }
}

/// Regime de deliberacao disparado pela hiperaresta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Regime {
    /// Sinal insuficiente.
    Quiescente,
    /// Fases alinhadas: propagacao direta pelo Sistema 1.
    Ressonancia,
    /// Fases proximas sem alinhamento: batimento, monitorar Crononomia.
    Batimento,
    /// Energia presente anulada por antifase: abrir a hiperaresta no Sistema 2.
    Conflito,
}

impl Regime {
    pub fn nome(self) -> &'static str {
        match self {
            Regime::Quiescente => "quiescente",
            Regime::Ressonancia => "ressonancia",
            Regime::Batimento => "batimento",
            Regime::Conflito => "conflito",
        }
    }
}

/// Uma perna da hiperaresta: o estado do vertice ja transportado ate a hiperaresta.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Acoplamento {
    pub amplitude: f64,
    pub fase: f64,
    pub condutancia: f64,
    pub atraso: f64,
}

impl Acoplamento {
    pub fn novo(amplitude: f64, fase: f64, condutancia: f64, atraso: f64) -> Self {
        Self {
            amplitude,
            fase,
            condutancia,
            atraso,
        }
    }

    /// Amplitude efetiva depois da conducao.
    pub fn efetiva(&self) -> f64 {
        self.condutancia * self.amplitude
    }

    /// Fase efetiva depois do atraso de propagacao.
    pub fn fase_efetiva(&self) -> f64 {
        self.fase + self.atraso
    }

    pub fn fasor(&self) -> Complexo {
        Complexo::polar(self.efetiva(), self.fase_efetiva())
    }
}

/// Diagnostico completo da hiperaresta num instante.
#[derive(Debug, Clone, PartialEq)]
pub struct Campo {
    pub fasor: Complexo,
    /// E(h) = |Psi_h|^2, a energia observavel.
    pub energia: f64,
    /// Soma incoerente das energias, sem termo cruzado.
    pub energia_classica: f64,
    /// Termo de interferencia: energia menos energia classica.
    pub interferencia: f64,
    /// C(h) = |Psi_h| / soma das amplitudes efetivas, em [0, 1].
    pub coerencia: f64,
    pub soma_amplitudes: f64,
    pub regime: Regime,
}

/// Avalia o operador de superposicao de uma hiperaresta.
pub fn superpoe(acoplamentos: &[Acoplamento], limiares: &Limiares) -> Campo {
    let mut fasor = Complexo::ZERO;
    let mut soma = 0.0;
    let mut classica = 0.0;

    for a in acoplamentos {
        fasor = fasor + a.fasor();
        let amp = a.efetiva();
        soma += amp;
        classica += amp * amp;
    }

    let energia = fasor.modulo2();
    let coerencia = if soma > 1e-12 {
        (fasor.modulo() / soma).clamp(0.0, 1.0)
    } else {
        0.0
    };

    let regime = if soma < limiares.ativacao {
        Regime::Quiescente
    } else if coerencia >= limiares.ressonancia {
        Regime::Ressonancia
    } else if coerencia <= limiares.critico && soma >= limiares.soma_critica {
        Regime::Conflito
    } else {
        Regime::Batimento
    };

    Campo {
        fasor,
        energia,
        energia_classica: classica,
        interferencia: energia - classica,
        coerencia,
        soma_amplitudes: soma,
        regime,
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use std::f64::consts::PI;

    fn par(fase_a: f64, fase_b: f64, amp: f64) -> Vec<Acoplamento> {
        vec![
            Acoplamento::novo(amp, fase_a, 1.0, 0.0),
            Acoplamento::novo(amp, fase_b, 1.0, 0.0),
        ]
    }

    #[test]
    fn em_fase_ha_ganho_superaditivo() {
        let c = superpoe(&par(0.10, 0.15, 0.8), &Limiares::default());
        assert!(
            c.energia > 1.8 * c.energia_classica,
            "energia {} classica {}",
            c.energia,
            c.energia_classica
        );
        assert!(c.interferencia > 0.0);
        assert_eq!(c.regime, Regime::Ressonancia);
    }

    #[test]
    fn em_antifase_a_energia_colapsa_e_abre_o_sistema_2() {
        let c = superpoe(&par(0.0, PI * 0.96, 0.85), &Limiares::default());
        assert!(c.energia < 0.05, "energia {}", c.energia);
        assert!(c.interferencia < 0.0);
        assert_eq!(c.regime, Regime::Conflito);
    }

    #[test]
    fn sinal_fraco_fica_quiescente() {
        let c = superpoe(&par(0.0, 0.0, 0.05), &Limiares::default());
        assert_eq!(c.regime, Regime::Quiescente);
        assert!((c.coerencia - 1.0).abs() < 1e-12);
    }

    #[test]
    fn atraso_de_propagacao_entra_na_fase() {
        let mut acs = par(0.0, 0.0, 0.5);
        acs[1].atraso = PI;
        let c = superpoe(&acs, &Limiares::default());
        assert!(c.energia < 1e-12);
    }
}
