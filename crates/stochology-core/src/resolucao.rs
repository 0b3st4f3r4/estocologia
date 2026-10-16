//! Horizonte R: resolucao da pergunta.
//!
//! O eixo empirico do projeto (H0 a H3) diz *onde* a pergunta e feita: semente, ciclo completo,
//! transferencia entre dominios, malha ao vivo. Este modulo acrescenta um segundo eixo, ortogonal:
//! *quao fina* e a pergunta do decisor, e quanto custa perguntar grosso.
//!
//! A hipotese de trabalho: com o mesmo recurso total, refinar a pergunta nao e retorica — muda o
//! numero e muda a acao. O instrumento abaixo mede a perda de resolver a mesma pergunta no agregado.

use crate::rng::Rng;

/// Grau de resolucao da pergunta do decisor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Resolucao {
    /// "Em media, o servico da conta?" — a pergunta feita ao dia medio.
    R0Agregada,
    /// "Cada dia, o servico da conta?" — a pergunta feita a distribuicao dos dias.
    R1Dia,
    /// "Cada hora, o servico da conta?" — a pergunta feita ao pico.
    R2Hora,
    /// "Cada hora, e para quem?" — a pergunta que traz a subpergunta e o subgrupo.
    R3Subpergunta,
}

impl Resolucao {
    pub const TODAS: [Resolucao; 4] = [
        Resolucao::R0Agregada,
        Resolucao::R1Dia,
        Resolucao::R2Hora,
        Resolucao::R3Subpergunta,
    ];

    pub fn codigo(self) -> &'static str {
        match self {
            Resolucao::R0Agregada => "R0-agregada",
            Resolucao::R1Dia => "R1-dia",
            Resolucao::R2Hora => "R2-hora",
            Resolucao::R3Subpergunta => "R3-subpergunta",
        }
    }

    /// Quanto mais fina a pergunta, maior o numero. A ordem importa: comparar resolucoes so faz
    /// sentido quando uma refina a outra.
    pub fn finura(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerdaDeResolucao {
    pub dias: usize,
    pub chegada_baixa: f64,
    pub chegada_alta: f64,
    pub capacidade_agregada: f64,
    pub espera_agregada: f64,
    pub espera_fina: f64,
    pub perda: f64,
    pub acima30_agregada: f64,
    pub acima30_fina: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct Medida {
    soma_espera: f64,
    acima: usize,
    clientes: usize,
}

impl Medida {
    fn somar(&mut self, outra: Medida) {
        self.soma_espera += outra.soma_espera;
        self.acima += outra.acima;
        self.clientes += outra.clientes;
    }

    fn espera_media(&self) -> f64 {
        if self.clientes == 0 {
            0.0
        } else {
            self.soma_espera / self.clientes as f64
        }
    }

    fn fracao_acima30(&self) -> f64 {
        if self.clientes == 0 {
            0.0
        } else {
            self.acima as f64 / self.clientes as f64
        }
    }
}

/// Um dia de fila, com os dois bracos pareados: as mesmas chegadas e as mesmas bases de servico,
/// mudando so a taxa de atendimento. Sem isso a comparacao mediria tambem a diferenca de sorteio.
fn dia_pareado(rng: &mut Rng, horas: usize, lam: f64, mu_agregada: f64, mu_fina: f64) -> (Medida, Medida) {
    let mut agregada = Medida::default();
    let mut fina = Medida::default();
    let mut espera_agregada = 0.0_f64;
    let mut espera_fina = 0.0_f64;
    // fila sem memoria: W_{n+1} = max(0, W_n + servico - intervalo entre chegadas)
    for _ in 0..(lam * horas as f64).round() as usize {
        let u = ((rng.proximo() >> 11) as f64 + 1.0) / ((1u64 << 53) as f64 + 1.0);
        let intervalo = -u.ln() / lam;
        let base = {
            let v = ((rng.proximo() >> 11) as f64 + 1.0) / ((1u64 << 53) as f64 + 1.0);
            -v.ln()
        };
        espera_agregada = (espera_agregada + base / mu_agregada - intervalo).max(0.0);
        espera_fina = (espera_fina + base / mu_fina - intervalo).max(0.0);
        agregada.soma_espera += espera_agregada * 60.0;
        fina.soma_espera += espera_fina * 60.0;
        agregada.clientes += 1;
        fina.clientes += 1;
        if espera_agregada * 60.0 > 30.0 {
            agregada.acima += 1;
        }
        if espera_fina * 60.0 > 30.0 {
            fina.acima += 1;
        }
    }
    (agregada, fina)
}

/// Mede a perda de resolver no agregado a mesma pergunta de capacidade.
///
/// Os dois bracos gastam o **mesmo recurso total**: o agregado usa a capacidade media todos os dias;
/// o fino usa a capacidade daquele dia. Muda so a resolucao da pergunta.
pub fn medir(
    semente: u64,
    dias: usize,
    horas_por_dia: usize,
    chegada_baixa: f64,
    chegada_alta: f64,
) -> PerdaDeResolucao {
    let media = (chegada_baixa + chegada_alta) / 2.0;
    let mut rng = Rng::novo(semente);
    let mut agregada = Medida::default();
    let mut fina = Medida::default();

    for dia in 0..dias {
        let cheio = dia % 2 == 1;
        let lam = if cheio { chegada_alta } else { chegada_baixa };
        let mu_fina = if cheio { chegada_alta } else { chegada_baixa };
        let (a, f) = dia_pareado(&mut rng, horas_por_dia, lam, media, mu_fina);
        agregada.somar(a);
        fina.somar(f);
    }

    PerdaDeResolucao {
        dias,
        chegada_baixa,
        chegada_alta,
        capacidade_agregada: media,
        espera_agregada: agregada.espera_media(),
        espera_fina: fina.espera_media(),
        perda: agregada.espera_media() - fina.espera_media(),
        acima30_agregada: agregada.fracao_acima30(),
        acima30_fina: fina.fracao_acima30(),
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn a_ordem_das_resolucoes_e_a_da_finura() {
        let finuras: Vec<u8> = Resolucao::TODAS.iter().map(|r| r.finura()).collect();
        assert_eq!(finuras, vec![0, 1, 2, 3]);
        assert!(Resolucao::R2Hora > Resolucao::R1Dia);
    }

    #[test]
    fn com_o_mesmo_recurso_a_pergunta_fina_espera_menos() {
        let p = medir(1909, 200, 10, 5.0, 11.0);
        assert!(
            p.espera_fina < p.espera_agregada,
            "fina {:.2} contra agregada {:.2}",
            p.espera_fina,
            p.espera_agregada
        );
        assert!(p.perda > 0.0);
        assert!(
            p.acima30_fina < p.acima30_agregada,
            "acima de 30 min: fina {:.3} contra agregada {:.3}",
            p.acima30_fina,
            p.acima30_agregada
        );
    }

    #[test]
    fn sem_variacao_nao_ha_perda_de_resolucao() {
        let p = medir(1909, 100, 10, 8.0, 8.0);
        assert!(p.perda.abs() < 1e-9, "perda {:.6}", p.perda);
    }
}
