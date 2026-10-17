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
fn dia_pareado(
    rng: &mut Rng,
    horas: usize,
    lam: f64,
    mu_agregada: f64,
    mu_fina: f64,
) -> (Medida, Medida) {
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

// ---------------------------------------------------------------------------------------------
// Previsões 3 e 5 do eixo R
// ---------------------------------------------------------------------------------------------

/// A meta de serviço da escala: o plano de Erlang C deixa no máximo 20% dos clientes em fila.
pub const META_DA_ESCALA: f64 = 0.80;
/// O critério de dia falho, declarado antes de rodar: o dia falha quando **mais de 10%** dos
/// clientes esperam mais de 30 min.
pub const DIA_FALHO_ACIMA30: f64 = 0.10;
/// O limiar da previsão 3, escrito no documento do eixo: mais de **um quarto** dos dias falhos.
pub const FRACAO_DE_DIAS_FALHOS_DECLARADA: f64 = 0.25;

/// Um dia simulado com um número inteiro de agentes, pela recursão de fila sem memória.
fn dia_com_agentes(rng: &mut Rng, horas: usize, lam: f64, agentes: u32) -> Medida {
    let mut medida = Medida::default();
    let mut espera = 0.0_f64;
    let mu = agentes.max(1) as f64;
    for _ in 0..(lam * horas as f64).round() as usize {
        let u = ((rng.proximo() >> 11) as f64 + 1.0) / ((1u64 << 53) as f64 + 1.0);
        let intervalo = -u.ln() / lam;
        let v = ((rng.proximo() >> 11) as f64 + 1.0) / ((1u64 << 53) as f64 + 1.0);
        let base = -v.ln();
        espera = (espera + base / mu - intervalo).max(0.0);
        medida.soma_espera += espera * 60.0;
        medida.clientes += 1;
        if espera * 60.0 > 30.0 {
            medida.acima += 1;
        }
    }
    medida
}

/// Um arranjo de oscilação: a amplitude, a escala de R0 e o que aconteceu nos dias.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PontoDeOscilacao {
    /// O dia fraco chega a `media - amplitude` e o forte a `media + amplitude`.
    pub amplitude: f64,
    pub dias: usize,
    /// Quantos dias o plano de R0 (a escala do dia médio) falhou.
    pub dias_falhos: usize,
    pub fracao_de_dias_falhos: f64,
    /// A escala de R0, em agentes, dimensionada pelo dia médio.
    pub agentes_r0: u32,
    /// A perda de resolução: espera média de R0 menos a de R1, em minutos.
    pub perda: f64,
    pub acima30_r0: f64,
    pub acima30_r1: f64,
}

/// O que a previsão 3 devolve.
#[derive(Debug, Clone, PartialEq)]
pub struct LimiarDeOscilacao {
    pub pontos: Vec<PontoDeOscilacao>,
    /// A menor amplitude em que a decisão de R0 falha em mais de um quarto dos dias.
    pub limiar: Option<f64>,
    /// A perda de resolução nessa amplitude: o preço, em minutos, de perguntar grosso.
    pub perda_no_limiar: Option<f64>,
    /// Se a fração de dias falhos cresce com a amplitude.
    pub monotona: bool,
}

/// Mede a previsão 3: para cada amplitude, quantos dias a decisão tomada em R0 falha, e quanto custa
/// refinar. Os dois braços gastam o mesmo recurso total: a escala de R0 é a do dia médio, a de R1 é a
/// do dia, e o dia par é forte e o ímpar é fraco.
pub fn limiar_de_oscilacao(
    semente: u64,
    dias: usize,
    horas_por_dia: usize,
    media: f64,
    amplitudes: &[f64],
) -> LimiarDeOscilacao {
    let mut rng = Rng::novo(semente);
    let agentes_r0 = crate::erlang::escala_minima(media, 1.0, META_DA_ESCALA);
    let mut pontos = Vec::new();
    for &amplitude in amplitudes {
        let mut falhos = 0usize;
        let mut r0 = Medida::default();
        let mut r1 = Medida::default();
        for dia in 0..dias {
            let forte = dia % 2 == 1;
            let lam = if forte {
                media + amplitude
            } else {
                (media - amplitude).max(0.0)
            };
            let a = dia_com_agentes(&mut rng, horas_por_dia, lam, agentes_r0);
            let agentes_r1 = crate::erlang::escala_minima(lam.max(1e-9), 1.0, META_DA_ESCALA);
            let f = dia_com_agentes(&mut rng, horas_por_dia, lam, agentes_r1);
            if a.fracao_acima30() > DIA_FALHO_ACIMA30 {
                falhos += 1;
            }
            r0.somar(a);
            r1.somar(f);
        }
        pontos.push(PontoDeOscilacao {
            amplitude,
            dias,
            dias_falhos: falhos,
            fracao_de_dias_falhos: falhos as f64 / dias.max(1) as f64,
            agentes_r0,
            perda: r0.espera_media() - r1.espera_media(),
            acima30_r0: r0.fracao_acima30(),
            acima30_r1: r1.fracao_acima30(),
        });
    }
    let cruzamento = pontos
        .iter()
        .find(|p| p.fracao_de_dias_falhos > FRACAO_DE_DIAS_FALHOS_DECLARADA);
    LimiarDeOscilacao {
        limiar: cruzamento.map(|p| p.amplitude),
        perda_no_limiar: cruzamento.map(|p| p.perda),
        monotona: pontos
            .windows(2)
            .all(|par| par[1].fracao_de_dias_falhos >= par[0].fracao_de_dias_falhos - 1e-12),
        pontos,
    }
}

/// A escala de R0 **calibrada**: o menor número de agentes em que a demanda plana do dia médio faz
/// falhar no máximo `alvo_de_dias_falhos` dos dias. É a segunda especificação do desenho — a
/// primeira, o plano de Erlang C da meta de 80%, deixava 1 dia em 5 falho já com a demanda plana.
/// A calibração entrou como especificação nova, declarada e contada no K, e não como conserto.
pub fn escala_calibrada(
    semente: u64,
    dias: usize,
    horas_por_dia: usize,
    media: f64,
    alvo_de_dias_falhos: f64,
) -> u32 {
    let inicial = crate::erlang::escala_minima(media, 1.0, META_DA_ESCALA).max(1);
    for agentes in inicial..(inicial + 40) {
        let mut rng = Rng::novo(semente);
        let mut falhos = 0usize;
        for _ in 0..dias {
            let m = dia_com_agentes(&mut rng, horas_por_dia, media, agentes);
            if m.fracao_acima30() > DIA_FALHO_ACIMA30 {
                falhos += 1;
            }
        }
        if (falhos as f64 / dias.max(1) as f64) <= alvo_de_dias_falhos {
            return agentes;
        }
    }
    inicial + 40
}

/// A previsão 3 medida com um plano de R0 dado (em agentes) — é o que permite comparar as duas
/// especificações do desenho lado a lado.
pub fn limiar_de_oscilacao_com_plano(
    semente: u64,
    dias: usize,
    horas_por_dia: usize,
    media: f64,
    amplitudes: &[f64],
    agentes_r0: u32,
) -> LimiarDeOscilacao {
    let mut rng = Rng::novo(semente);
    let mut pontos = Vec::new();
    for &amplitude in amplitudes {
        let mut falhos = 0usize;
        let mut r0 = Medida::default();
        let mut r1 = Medida::default();
        for dia in 0..dias {
            let forte = dia % 2 == 1;
            let lam = if forte {
                media + amplitude
            } else {
                (media - amplitude).max(0.0)
            };
            let a = dia_com_agentes(&mut rng, horas_por_dia, lam, agentes_r0);
            let agentes_r1 = crate::erlang::escala_minima(lam.max(1e-9), 1.0, META_DA_ESCALA);
            let f = dia_com_agentes(&mut rng, horas_por_dia, lam, agentes_r1);
            if a.fracao_acima30() > DIA_FALHO_ACIMA30 {
                falhos += 1;
            }
            r0.somar(a);
            r1.somar(f);
        }
        pontos.push(PontoDeOscilacao {
            amplitude,
            dias,
            dias_falhos: falhos,
            fracao_de_dias_falhos: falhos as f64 / dias.max(1) as f64,
            agentes_r0,
            perda: r0.espera_media() - r1.espera_media(),
            acima30_r0: r0.fracao_acima30(),
            acima30_r1: r1.fracao_acima30(),
        });
    }
    let cruzamento = pontos
        .iter()
        .find(|p| p.fracao_de_dias_falhos > FRACAO_DE_DIAS_FALHOS_DECLARADA);
    LimiarDeOscilacao {
        limiar: cruzamento.map(|p| p.amplitude),
        perda_no_limiar: cruzamento.map(|p| p.perda),
        monotona: pontos
            .windows(2)
            .all(|par| par[1].fracao_de_dias_falhos >= par[0].fracao_de_dias_falhos - 1e-12),
        pontos,
    }
}

/// A checagem da previsão 5 sobre as frações diárias.
#[derive(Debug, Clone, PartialEq)]
pub struct ChecagemDaImplicacao {
    /// Todos os dias passam e o agregado não: a previsão 5 seria falsificada aqui.
    pub violacoes: usize,
    /// O agregado passa e algum dia não: prova que o contrário da previsão não vale.
    pub contraexemplos: usize,
    pub dias: usize,
    pub agregado: f64,
    /// O agregado refeito como média ponderada dos dias: é a conta da conservação.
    pub agregado_pela_conservacao: f64,
    pub identidade_conferida: bool,
}

/// Confere a conservação do agregado: a soma dos dias tem de reconstruir o agregado medido.
pub fn conservacao_do_agregado(por_dia: &[(usize, usize)], agregado: (usize, usize)) -> bool {
    let acima: usize = por_dia.iter().map(|(a, _)| *a).sum();
    let clientes: usize = por_dia.iter().map(|(_, n)| *n).sum();
    acima == agregado.0 && clientes == agregado.1
}

/// Confere a implicação da previsão 5 num conjunto de dias: cada dia com (acima, clientes).
pub fn checar_implicacao(dias: &[(usize, usize)], limiar: f64) -> ChecagemDaImplicacao {
    let clientes: usize = dias.iter().map(|(_, n)| *n).sum();
    let acima: usize = dias.iter().map(|(a, _)| *a).sum();
    let agregado = if clientes == 0 {
        0.0
    } else {
        acima as f64 / clientes as f64
    };
    let ponderada = if clientes == 0 {
        0.0
    } else {
        dias.iter()
            .map(|(a, n)| {
                if *n == 0 {
                    0.0
                } else {
                    *a as f64 / *n as f64 * *n as f64
                }
            })
            .sum::<f64>()
            / clientes as f64
    };
    let todos_ok = dias
        .iter()
        .all(|(a, n)| *n == 0 || (*a as f64 / *n as f64) <= limiar);
    let algum_falho = dias
        .iter()
        .any(|(a, n)| *n > 0 && (*a as f64 / *n as f64) > limiar);
    let mut violacoes = 0usize;
    let mut contraexemplos = 0usize;
    if todos_ok && agregado > limiar {
        violacoes += 1;
    }
    if algum_falho && agregado <= limiar {
        contraexemplos += 1;
    }
    ChecagemDaImplicacao {
        violacoes,
        contraexemplos,
        dias: dias.len(),
        agregado,
        agregado_pela_conservacao: ponderada,
        identidade_conferida: (agregado - ponderada).abs() < 1e-12,
    }
}

/// O que a previsão 5 devolve sobre um conjunto de perfis de demanda.
#[derive(Debug, Clone, PartialEq)]
pub struct Transferencia {
    pub perfis: usize,
    /// Perfis em que R1 sobreviveu (nenhum dia falho) e R0 não: a previsão seria falsificada aqui.
    pub violacoes: usize,
    /// Perfis em que R0 sobreviveu e R1 não: o contrário da previsão.
    pub contraexemplos: usize,
    /// Se a soma dos dias reconstruiu o agregado em todos os perfis.
    pub conservacao_conferida: bool,
    pub limiar: f64,
}

/// Mede a previsão 5: a mesma decisão tomada em R1 (cada dia) e em R0 (o agregado).
pub fn transferencia(
    semente: u64,
    dias: usize,
    horas_por_dia: usize,
    perfis: &[(f64, f64)],
) -> Transferencia {
    let mut rng = Rng::novo(semente);
    let mut violacoes = 0usize;
    let mut contraexemplos = 0usize;
    let mut conservacao = true;
    for &(media, amplitude) in perfis {
        let agentes_r0 = crate::erlang::escala_minima(media, 1.0, META_DA_ESCALA);
        let mut por_dia: Vec<(usize, usize)> = Vec::new();
        let mut r0 = Medida::default();
        for dia in 0..dias {
            let forte = dia % 2 == 1;
            let lam = if forte {
                media + amplitude
            } else {
                (media - amplitude).max(0.0)
            };
            let a = dia_com_agentes(&mut rng, horas_por_dia, lam, agentes_r0);
            por_dia.push((a.acima, a.clientes));
            r0.somar(a);
        }
        let checagem = checar_implicacao(&por_dia, DIA_FALHO_ACIMA30);
        violacoes += checagem.violacoes;
        contraexemplos += checagem.contraexemplos;
        if !conservacao_do_agregado(&por_dia, (r0.acima, r0.clientes))
            || !checagem.identidade_conferida
        {
            conservacao = false;
        }
    }
    Transferencia {
        perfis: perfis.len(),
        violacoes,
        contraexemplos,
        conservacao_conferida: conservacao,
        limiar: DIA_FALHO_ACIMA30,
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

    // ---------------------------------------------------------------- previsões 3 e 5 do eixo R

    #[test]
    fn com_o_plano_calibrado_o_limiar_existe_e_a_fracao_cresce() {
        let amplitudes = [0.0, 1.0, 2.0, 3.0];
        let agentes = escala_calibrada(1909, 200, 10, 8.0, 0.10);
        let l = limiar_de_oscilacao_com_plano(1909, 200, 10, 8.0, &amplitudes, agentes);
        assert!(
            l.monotona,
            "a fração tem de crescer com a amplitude: {:?}",
            l.pontos
        );
        assert!(
            l.pontos[0].fracao_de_dias_falhos <= 0.10,
            "o plano é calibrado na demanda plana: {:.3}",
            l.pontos[0].fracao_de_dias_falhos
        );
        let ultimo = l.pontos.last().expect("último ponto");
        assert!(
            ultimo.fracao_de_dias_falhos > FRACAO_DE_DIAS_FALHOS_DECLARADA,
            "com amplitude {:.1} a falha passa de um quarto: {:.3}",
            ultimo.amplitude,
            ultimo.fracao_de_dias_falhos
        );
        assert!(
            l.limiar.is_some(),
            "o ponto existe e o instrumento o encontra"
        );
        assert!(l.limiar.expect("limiar") > 0.0, "o ponto não é o zero");
    }

    #[test]
    fn com_o_plano_de_erlang_a_previsao_3_nao_se_sustenta() {
        // A primeira especificação do desenho: a escala de Erlang C do dia médio. Ela falha em um
        // quinto dos dias já com a demanda plana, então o "ponto" não marca nada — e a fração não é
        // monótona no grid. É a prova de que o instrumento sabe reprovar a previsão.
        let amplitudes = [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];
        let l = limiar_de_oscilacao(1909, 200, 10, 8.0, &amplitudes);
        assert!(
            l.pontos[0].fracao_de_dias_falhos > 0.15,
            "com a demanda plana a escala de Erlang já falha um dia em cinco: {:.3}",
            l.pontos[0].fracao_de_dias_falhos
        );
        assert!(
            !l.monotona,
            "e a fração não cresce de forma monótona neste desenho"
        );
        let perda_no_limiar = l.perda_no_limiar.expect("o cruzamento existe no grid");
        assert!(
            perda_no_limiar < 0.0,
            "e no cruzamento refinar custa, em vez de pagar: {perda_no_limiar:.2} min"
        );
    }

    #[test]
    fn abaixo_do_limiar_refinar_compra_menos_do_que_acima() {
        let amplitudes = [0.5, 1.5, 2.5, 3.5];
        let l = limiar_de_oscilacao(1909, 40, 10, 8.0, &amplitudes);
        let perda_abaixo = l.pontos[0].perda;
        let perda_acima = l.pontos[3].perda;
        assert!(
            perda_acima > perda_abaixo,
            "a perda cresce com a oscilação: {perda_acima:.2} contra {perda_abaixo:.2}"
        );
    }

    #[test]
    fn a_transferencia_nao_tem_violacao_e_tem_contraexemplo() {
        let perfis = [(8.0, 0.5), (8.0, 1.0), (8.0, 2.0), (8.0, 3.0)];
        let t = transferencia(1909, 40, 10, &perfis);
        assert_eq!(
            t.violacoes, 0,
            "um resultado que sobrevive a R1 sobrevive a R0"
        );
        assert!(t.contraexemplos > 0, "e o contrário não vale: {:?}", t);
        assert!(
            t.conservacao_conferida,
            "a soma dos dias reconstrói o agregado"
        );
    }

    #[test]
    fn a_conservacao_reprova_um_agregado_que_nao_bate() {
        let por_dia = vec![(5usize, 100usize), (4, 100), (3, 100)];
        assert!(conservacao_do_agregado(&por_dia, (12, 300)));
        assert!(!conservacao_do_agregado(&por_dia, (12, 299)), "n de menos");
        assert!(
            !conservacao_do_agregado(&por_dia, (11, 300)),
            "acima de menos"
        );
    }

    #[test]
    fn a_checagem_da_implicacao_acha_o_contraexemplo_e_nao_a_violacao() {
        let dias = vec![(1usize, 100usize), (30, 100), (5, 100), (2, 100)];
        let c = checar_implicacao(&dias, DIA_FALHO_ACIMA30);
        assert_eq!(c.violacoes, 0);
        assert_eq!(c.contraexemplos, 1);
        assert!(
            (c.agregado - 0.095).abs() < 1e-12,
            "agregado {:.4}",
            c.agregado
        );
        assert!(c.identidade_conferida);
        let limpos = vec![(5usize, 100usize), (6, 100)];
        let c2 = checar_implicacao(&limpos, DIA_FALHO_ACIMA30);
        assert_eq!(c2.violacoes, 0);
        assert_eq!(c2.contraexemplos, 0);
    }
}
