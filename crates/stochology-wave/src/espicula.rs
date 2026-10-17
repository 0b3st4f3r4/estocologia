//! Barramento de espiculas epistemicas: comunicacao assincrona entre as cinco lentes.
//!
//! Nao ha texto livre entre agentes. Cada evento e uma tupla imutavel de 40 bytes logicos
//! (lente, vertice, instante, amplitude, fase, selo da evidencia). Aprender a ordem causal entre
//! lentes e trabalho da regra STDP fasorial; reconhecer um padrao e trabalho do vocabulario de
//! espiculas, que compara intervalos e defasagens, nao instantes absolutos.

use stochology_hypergraph::Lente;

use crate::selo::{Selo, envolve_alfa, envolve_pi};

/// Numero de lentes operacionais.
pub const LENTES: usize = 5;

/// Posicao canonica da lente na matriz sinaptica.
pub fn indice(lente: Lente) -> usize {
    match lente {
        Lente::Fluxonomia => 0,
        Lente::Crononomia => 1,
        Lente::Escalonomia => 2,
        Lente::Morfonomia => 3,
        Lente::Crisonomia => 4,
    }
}

/// Lente a partir da posicao canonica.
pub fn lente_de(posicao: usize) -> Lente {
    match posicao % LENTES {
        0 => Lente::Fluxonomia,
        1 => Lente::Crononomia,
        2 => Lente::Escalonomia,
        3 => Lente::Morfonomia,
        _ => Lente::Crisonomia,
    }
}

/// Uma espicula no barramento.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Espicula {
    pub lente: Lente,
    pub vertice: u32,
    pub t_us: u64,
    pub amplitude: f64,
    pub fase: f64,
    pub hash_evidencia: u64,
}

impl Espicula {
    pub fn nova(
        lente: Lente,
        vertice: u32,
        t_us: u64,
        amplitude: f64,
        fase: f64,
        hash_evidencia: u64,
    ) -> Self {
        Self {
            lente,
            vertice,
            t_us,
            amplitude,
            fase,
            hash_evidencia,
        }
    }
}

/// Uma nota esperada dentro de uma palavra: lente, atraso e defasagem relativos a ancora.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Nota {
    pub lente: Lente,
    pub atraso_us: u64,
    pub defasagem: f64,
}

impl Nota {
    pub fn nova(lente: Lente, atraso_us: u64, defasagem: f64) -> Self {
        Self {
            lente,
            atraso_us,
            defasagem,
        }
    }
}

/// Limiar minimo de contribuicao de uma nota.
const CONTRIBUICAO_MINIMA: f64 = 0.10;

/// Uma palavra do vocabulario de espiculas.
#[derive(Debug, Clone, PartialEq)]
pub struct Palavra {
    pub id: &'static str,
    pub sentido: &'static str,
    /// Tolerancia temporal da coincidencia, em microssegundos.
    pub tau_us: f64,
    pub limiar: f64,
    pub notas: Vec<Nota>,
}

/// Palavra reconhecida num fluxo de espiculas.
#[derive(Debug, Clone, PartialEq)]
pub struct PalavraCumprida {
    pub id: &'static str,
    pub sentido: &'static str,
    pub ancora_us: u64,
    pub ressonancia: f64,
    pub fase_resultante: f64,
    pub selo: u64,
}

impl Palavra {
    /// Procura a palavra no fluxo, em qualquer instante absoluto.
    pub fn casa(&self, fluxo: &[Espicula]) -> Option<PalavraCumprida> {
        if self.notas.len() < 2 || fluxo.len() < self.notas.len() {
            return None;
        }
        let primeira = self.notas[0];
        let mut melhor: Option<PalavraCumprida> = None;

        for (i, ancora) in fluxo.iter().enumerate() {
            if ancora.lente != primeira.lente {
                continue;
            }
            let mut soma_re = ancora.amplitude;
            let mut soma_im = 0.0;
            let mut casadas = 1usize;
            let mut selo = Selo::novo();
            selo.escreve_u64(ancora.hash_evidencia);

            for nota in &self.notas[1..] {
                let mut melhor_nota = 0.0;
                let mut contrib = (0.0, 0.0);
                let mut evidencia = 0u64;

                for cand in &fluxo[(i + 1)..] {
                    if cand.lente != nota.lente || cand.t_us < ancora.t_us {
                        continue;
                    }
                    let atraso = (cand.t_us - ancora.t_us) as f64;
                    let erro = (atraso - nota.atraso_us as f64).abs();
                    let decaimento = (-erro / self.tau_us).exp();

                    let defasagem = envolve_pi(cand.fase - ancora.fase);
                    let erro_fase = envolve_pi(defasagem - nota.defasagem);

                    let efetiva = cand.amplitude * decaimento;
                    let nota_score = efetiva * erro_fase.cos();
                    if nota_score > melhor_nota {
                        melhor_nota = nota_score;
                        contrib = (efetiva * erro_fase.cos(), efetiva * erro_fase.sin());
                        evidencia = cand.hash_evidencia;
                    }
                }

                if melhor_nota > CONTRIBUICAO_MINIMA {
                    casadas += 1;
                    soma_re += contrib.0;
                    soma_im += contrib.1;
                    selo.escreve_u64(evidencia);
                }
            }

            if casadas != self.notas.len() {
                continue;
            }

            let m = self.notas.len() as f64;
            let ressonancia = (soma_re * soma_re + soma_im * soma_im).sqrt() / m;
            if ressonancia < self.limiar {
                continue;
            }

            let cumprida = PalavraCumprida {
                id: self.id,
                sentido: self.sentido,
                ancora_us: ancora.t_us,
                ressonancia,
                fase_resultante: soma_im.atan2(soma_re),
                selo: selo.fecha(),
            };
            if melhor
                .as_ref()
                .is_none_or(|b| cumprida.ressonancia > b.ressonancia)
            {
                melhor = Some(cumprida);
            }
        }
        melhor
    }
}

/// Hiperparametros da regra STDP fasorial.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfigStdp {
    pub q: f64,
    pub a_mais: f64,
    pub a_menos: f64,
    pub tau_mais_us: f64,
    pub tau_menos_us: f64,
    pub lr_alfa: f64,
    pub oja: f64,
    pub horizonte_us: u64,
}

impl Default for ConfigStdp {
    fn default() -> Self {
        Self {
            q: 0.5,
            a_mais: 0.18,
            a_menos: 0.12,
            tau_mais_us: 20_000.0,
            tau_menos_us: 20_000.0,
            lr_alfa: 0.30,
            oja: 0.04,
            horizonte_us: 60_000,
        }
    }
}

/// Matriz sinaptica 5x5 entre lentes, com o fluxo de espiculas que a treinou.
#[derive(Debug, Clone, PartialEq)]
pub struct BarramentoStdp {
    pub condutancia: [[f64; LENTES]; LENTES],
    pub alfa: [[f64; LENTES]; LENTES],
    pub fluxo: Vec<Espicula>,
}

impl BarramentoStdp {
    #[allow(clippy::needless_range_loop)]
    pub fn novo(inicial: f64) -> Self {
        let mut condutancia = [[0.0; LENTES]; LENTES];
        for i in 0..LENTES {
            for j in 0..LENTES {
                if i != j {
                    condutancia[i][j] = inicial;
                }
            }
        }
        Self {
            condutancia,
            alfa: [[0.0; LENTES]; LENTES],
            fluxo: Vec::new(),
        }
    }

    /// Publica uma espicula e aplica STDP contra o fluxo recente.
    pub fn publica(&mut self, espicula: Espicula, cfg: &ConfigStdp) {
        let dois_pi_q = 2.0 * std::f64::consts::PI * cfg.q;
        let pos = indice(espicula.lente);

        for passado in self.fluxo.iter().rev() {
            if espicula.t_us < passado.t_us {
                continue;
            }
            let dt = espicula.t_us - passado.t_us;
            if dt > cfg.horizonte_us {
                break;
            }
            if dt == 0 || passado.lente == espicula.lente {
                continue;
            }

            let pre = indice(passado.lente);
            let dt_f = dt as f64;

            // LTP causal: pre antes de pos.
            let k_mais = cfg.a_mais * (-dt_f / cfg.tau_mais_us).exp();
            let chegada = passado.fase + dois_pi_q * self.alfa[pre][pos];
            let dphi = envolve_pi(espicula.fase - chegada);
            let hebb = k_mais * passado.amplitude * espicula.amplitude * dphi.cos();
            let oja =
                cfg.oja * self.condutancia[pre][pos] * espicula.amplitude * espicula.amplitude;

            self.condutancia[pre][pos] = (self.condutancia[pre][pos] + hebb - oja).clamp(0.01, 2.0);
            let d_alfa = (cfg.lr_alfa / dois_pi_q)
                * k_mais
                * passado.amplitude
                * espicula.amplitude
                * dphi.sin();
            self.alfa[pre][pos] = envolve_alfa(self.alfa[pre][pos] + d_alfa);

            // LTD anticausal: a via reversa paga o custo.
            let k_menos = cfg.a_menos * (-dt_f / cfg.tau_menos_us).exp();
            let chegada_rev = espicula.fase + dois_pi_q * self.alfa[pos][pre];
            let dphi_rev = envolve_pi(passado.fase - chegada_rev);
            let ltd = k_menos * passado.amplitude * espicula.amplitude * dphi_rev.cos().abs();
            self.condutancia[pos][pre] = (self.condutancia[pos][pre] - ltd).clamp(0.01, 2.0);
        }

        self.fluxo.push(espicula);
    }

    /// Assimetria causal: eta(de -> para) - eta(para -> de).
    pub fn assimetria(&self, de: Lente, para: Lente) -> f64 {
        self.condutancia[indice(de)][indice(para)] - self.condutancia[indice(para)][indice(de)]
    }

    /// Selo canonico da matriz sinaptica.
    #[allow(clippy::needless_range_loop)]
    pub fn selo(&self) -> u64 {
        let mut selo = Selo::novo();
        for i in 0..LENTES {
            for j in 0..LENTES {
                selo.escreve_f64(self.condutancia[i][j]);
                selo.escreve_f64(self.alfa[i][j]);
            }
        }
        selo.fecha()
    }

    /// Exige assimetria positiva minima em cada elo da cadeia causal declarada.
    pub fn confere_causalidade(
        &self,
        cadeia: &[(Lente, Lente)],
        minimo: f64,
    ) -> Result<(), String> {
        for &(de, para) in cadeia {
            let a = self.assimetria(de, para);
            if a < minimo || a.is_nan() {
                return Err(format!(
                    "causalidade insuficiente em {} -> {}: assimetria {a:.4}",
                    de.nome(),
                    para.nome()
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    pub(crate) fn vocabulario() -> Vec<Palavra> {
        vec![
            Palavra {
                id: "W_CASCATA",
                sentido: "deformacao, gargalo, desvio de ciclo, amplificacao e ruptura",
                tau_us: 8_000.0,
                limiar: 0.70,
                notas: vec![
                    Nota::nova(Lente::Morfonomia, 0, 0.0),
                    Nota::nova(Lente::Fluxonomia, 10_000, 0.25),
                    Nota::nova(Lente::Crononomia, 22_000, 0.50),
                    Nota::nova(Lente::Escalonomia, 35_000, 0.80),
                    Nota::nova(Lente::Crisonomia, 50_000, 1.10),
                ],
            },
            Palavra {
                id: "W_FREIO",
                sentido: "inversao brusca de fase entre fluxo e forma",
                tau_us: 5_000.0,
                limiar: 0.70,
                notas: vec![
                    Nota::nova(Lente::Fluxonomia, 0, 0.0),
                    Nota::nova(Lente::Morfonomia, 6_000, std::f64::consts::PI),
                ],
            },
        ]
    }

    pub(crate) fn cascata(base: u64, embutida: bool) -> Vec<Espicula> {
        let fase = |k: usize| {
            if embutida {
                let fs = [0.10, 0.35, 0.60, 0.90, 1.20];
                fs[k]
            } else {
                let fs = [0.10, 0.90, 0.20, 1.60, 0.50];
                fs[k]
            }
        };
        let lentes = [
            Lente::Morfonomia,
            Lente::Fluxonomia,
            Lente::Crononomia,
            Lente::Escalonomia,
            Lente::Crisonomia,
        ];
        let tempos = [0u64, 10_200, 21_800, 35_100, 49_900];
        let amps = [0.90, 0.88, 0.85, 0.92, 0.96];
        (0..5)
            .map(|k| {
                Espicula::nova(
                    lentes[k],
                    10 + k as u32,
                    base + tempos[k],
                    amps[k],
                    fase(k),
                    0xA000 + base + k as u64,
                )
            })
            .collect()
    }

    #[test]
    fn reconhece_cascata_em_qualquer_instante() {
        let voc = vocabulario();
        let fluxo = cascata(1_000, true);
        let achada = voc[0].casa(&fluxo).expect("cascata");
        assert!(
            achada.ressonancia > 0.80,
            "ressonancia {}",
            achada.ressonancia
        );
        let deslocado: Vec<Espicula> = fluxo
            .iter()
            .map(|e| Espicula {
                t_us: e.t_us + 5_000_000,
                ..*e
            })
            .collect();
        let outra = voc[0].casa(&deslocado).expect("cascata deslocada");
        assert!((outra.ressonancia - achada.ressonancia).abs() < 1e-9);
        assert_eq!(outra.selo, achada.selo);
    }

    #[test]
    fn intervalos_errados_nao_casam() {
        let voc = vocabulario();
        let fluxo: Vec<Espicula> = cascata(1_000, true)
            .into_iter()
            .zip([0u64, 2_000, 4_000, 6_000, 8_000])
            .map(|(e, t)| Espicula {
                t_us: 1_000 + t,
                ..e
            })
            .collect();
        assert!(voc[0].casa(&fluxo).is_none());
    }

    #[test]
    fn stdp_cria_assimetria_causal() {
        let mut barramento = BarramentoStdp::novo(0.25);
        let cfg = ConfigStdp::default();
        for ep in 0..5u64 {
            for e in cascata(ep * 200_000, true) {
                barramento.publica(e, &cfg);
            }
        }
        let cadeia = [
            (Lente::Morfonomia, Lente::Fluxonomia),
            (Lente::Fluxonomia, Lente::Crononomia),
            (Lente::Crononomia, Lente::Escalonomia),
            (Lente::Escalonomia, Lente::Crisonomia),
        ];
        for &(de, para) in &cadeia {
            assert!(
                barramento.assimetria(de, para) > 0.05,
                "assimetria {} -> {} = {}",
                de.nome(),
                para.nome(),
                barramento.assimetria(de, para)
            );
        }
        barramento
            .confere_causalidade(&cadeia, 0.05)
            .expect("cadeia causal");
    }

    #[test]
    fn selo_do_barramento_e_estavel() {
        let cfg = ConfigStdp::default();
        let mut a = BarramentoStdp::novo(0.25);
        let mut b = BarramentoStdp::novo(0.25);
        for e in cascata(0, true) {
            a.publica(e, &cfg);
            b.publica(e, &cfg);
        }
        assert_eq!(a.selo(), b.selo());
    }

    #[test]
    fn indice_e_lente_ida_e_volta() {
        for l in Lente::TODAS {
            assert_eq!(lente_de(indice(l)), l);
        }
    }
}
