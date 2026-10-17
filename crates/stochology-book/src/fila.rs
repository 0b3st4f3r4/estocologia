//! H0 real em Rust: a fila com desistencia e as quatro regras de escala (R0 a R3).
//!
//! Le `dados-reais/bank_calls.csv` (ou a base apontada por `ESTOCOLOGIA_DADOS`), monta a
//! escala no treino (marco a junho) e a aplica ao cofre (julho a outubro) com **fila com
//! desistencia**: atendimento exponencial de 4 min e paciencia exponencial de 3 min — os dois
//! valores do registro —, chegadas reais espalhadas dentro de cada intervalo de 5 min da base,
//! FCFS e abandonos. Sem relogio, sem rede, sem dependencia externa: SplitMix64 de `banco.rs`,
//! semente fixa (`SEMENTE = 2003`, a do registro).
//!
//! A grade e a do instrumento aposentado (`h0-real/estocologia/central.py`, ainda no historico
//! do git no commit `4bb7181`): 169 intervalos de 5 min por dia (07:00 as 21:00), escala
//! constante em cada grupo de 6 intervalos (meia hora, 29 grupos com a cauda de um intervalo),
//! e o mesmo motor FCFS exato — quem sofreria espera maior que a propria paciencia desiste, e
//! quem nao desiste ocupa o atendente de menor tempo livre; atendente que sai de turno termina
//! a chamada em curso.
//!
//! As quatro regras, como o registro as define:
//!
//! | regra | o que e |
//! | --- | --- |
//! | R0 | a alegacao: Erlang C fatia a fatia sobre o perfil medio do treino |
//! | R1 | perfil do dia da semana (custo zero) |
//! | R2 | R1 com a demanda multiplicada por `1 + Z_MARGEM * cv`, `Z_MARGEM = 1.2816` |
//! | R3 | R0 mais `extra` atendentes por igual, com `extra` tal que o custo iguale o de R2 |
//!
//! onde `cv` e o desvio padrao do residuo do total do dia (total do dia dividido pela media do
//! total do seu dia da semana, no treino) — o 6% do capitulo 16 — e `Z_MARGEM` e o quantil 90%
//! da normal: e o "p90" que o registro poe no nome de R2.
//!
//! Toda medida sai na unidade do dia: agente-horas por dia, nivel de servico ponderado por
//! chamada, dias abaixo da meta de 80% e a cauda da espera (mediana e p90). Ha dois motores de
//! fila: o `Motor::Exato` (a reconstrucao do instrumento aposentado) e o `Motor::Eventos`
//! (M/M/c+M evento a evento, chegadas de Poisson fatia a fatia), declarado como verificacao
//! independente de que a decisao nao e artefato do motor.
//!
//! # Declaracao de divergencia (registro antigo x este instrumento)
//!
//! O registro (`h0-real/README.md`, capitulo 16) publica para o cofre de 81 dias: R0 com 2 264
//! agente-horas por dia, 86% e 26% de dias falhos; R1 com 2 261, 90% e 12%; R3 com 2 433, 94% e
//! 7,4%; R2 com 2 428, 96% e 3,7%. Este modulo e uma **reconstrucao em Rust** daquele
//! experimento, com o gerador de numeros trocado (SplitMix64 contra PCG64) e sem o numpy: os
//! sorteios de atendimento e paciencia nao caem nos mesmos lugares, de modo que a fila nao e
//! identica chamada por chamada. O que este modulo afirma e o **desenho** (as quatro regras, a
//! grade, o motor) e a **ordem** (R2 melhor que R1, R1 melhor que R0, R3 entre as duas, R2 e R3
//! com o mesmo custo), com os numeros medidos impressos por `rodar()` e conferidos por faixas
//! generosas em `faixas_do_registro_e_divergencia_declarada`. Medido aqui (semente 2003, uma
//! replica por dia, motor Exato), com o publicado ao lado:
//!
//! | regra | agente-horas/dia | nivel de servico | dias abaixo da meta |
//! | --- | --- | --- | --- |
//! | R0 | 2 264 (2 264) | 83,9% (86%) | 30,9% (26%) |
//! | R1 | 2 261 (2 261) | 84,3% (90%) | 17,3% (12%) |
//! | R2 | 2 428 (2 428) | 91,5% (96%) | 4,9% (3,7%) |
//! | R3 | 2 433 (2 433) | 90,2% (94%) | 11,1% (7,4%) |
//!
//! O que **reproduz**: o custo das quatro regras casa por casa (2 264, 2 261, 2 428, 2 433), o
//! pico de 234 atendentes de R0, os 12 atendentes por igual que R3 acrescenta, o CV residual de
//! 5,98% contra os 6% do capitulo, a queda de dias falhos de R0 para R2 (31% para 5%) e a ordem
//! R2 < R3 <= R1 <= R0. Reproduzem tambem dois numeros do registro que nao sao das quatro regras
//! e que por isso valem como prova do motor: a espera media por chamada de R0 sem desistencia,
//! 1 040 s contra os 956 s publicados, e o mundo da alegacao, 96,3% de nivel de servico e nenhum
//! dia falho em 81, contra os 97% e 0% publicados.
//!
//! O que **diverge**: o nivel de servico sai de 2 a 6 pontos abaixo do publicado, e a fracao de
//! dias falhos de R1 e de R3 sai acima. O caso mais afastado e R1: o registro publica 90% e 12%,
//! e aqui os dois motores medem 84,3% (Exato) e 87,7% (Eventos), com a queda de dias falhos de
//! R0 para R1 reproduzida (31% para 17%, contra 26% para 12%). O ganho de R1 sobre R0 na media,
//! que o registro poe em 4 pontos, sai em menos de 1 ponto: no motor Exato o perfil do dia da
//! semana troca folga entre os dias e corta os dias ruins, sem mover a media — o que sobra de
//! espera e do que ja nao vira desistencia. Com tres replicas por dia o numero de R0 fica em
//! 84,7% (erro padrao de 0,6 ponto sobre os 81 dias), o que nao fecha a distancia. A divergencia
//! fica declarada, com os dois valores, sem arredondar um para o outro: o gerador de numeros e
//! outro (SplitMix64 contra PCG64) e a fila nao e identica chamada por chamada.
//!
//! O instrumento aposentado contava como "atendida em ate 20 s" tambem quem desistiu com
//! paciencia sorteada abaixo de 20 s (a espera do desistente e a propria paciencia). A
//! reconstrucao faz o mesmo, para nao trocar o alvo no meio do caminho; `desistencias_por_100`
//! sai ao lado, para quem quiser ler a conta sem esse vies.

use crate::banco::{self, META};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

/// Semente fixa do instrumento: a do registro (Phi1 — simulabilidade).
pub const SEMENTE: u64 = 2003;
/// Atendimento exponencial com media de 4 minutos, como o registro.
pub const AHT_FILA_S: f64 = 240.0;
/// Paciencia exponencial com media de 3 minutos, como o registro.
pub const PACIENCIA_S: f64 = 180.0;
/// Alvo de espera, em segundos.
pub const ALVO_FILA_S: f64 = 20.0;
/// Intervalo da base, em segundos.
pub const BIN_S: f64 = 300.0;
/// Intervalos de 5 min por dia: 07:00 as 21:00, o mesmo recorte do instrumento aposentado.
pub const BINS: usize = 169;
/// Intervalos por grupo de escala (meia hora).
pub const POR_GRUPO: usize = 6;
/// Grupos de escala por dia: 28 meias horas mais a cauda de um intervalo.
pub const GRUPOS: usize = BINS.div_ceil(POR_GRUPO);
/// Duracao de um grupo de escala cheio, em segundos.
pub const GRUPO_S: f64 = BIN_S * POR_GRUPO as f64;
/// Quantil 90% da normal: a margem de R2 e `1 + Z_MARGEM * cv`.
pub const Z_MARGEM: f64 = 1.2816;
/// Teto do gerador uniforme: 2^53, para caber exato em f64.
const DOIS53: f64 = 9_007_199_254_740_992.0;

/// As quatro regras de escala.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Regra {
    R0,
    R1,
    R2,
    R3,
}

impl Regra {
    /// As quatro, na ordem do registro.
    pub const TODAS: [Regra; 4] = [Regra::R0, Regra::R1, Regra::R2, Regra::R3];

    pub fn nome(self) -> &'static str {
        match self {
            Regra::R0 => "R0",
            Regra::R1 => "R1",
            Regra::R2 => "R2",
            Regra::R3 => "R3",
        }
    }
}

/// Motor da fila.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motor {
    /// Reconstrucao do instrumento aposentado: FCFS exato com as chegadas observadas.
    Exato,
    /// Verificacao independente: M/M/c+M evento a evento, chegadas de Poisson.
    Eventos,
}

/// Parametros declarados da fila.
#[derive(Debug, Clone, Copy)]
pub struct Parametros {
    pub aht_s: f64,
    pub paciencia_s: f64,
    pub alvo_s: f64,
    pub meta: f64,
}

impl Default for Parametros {
    fn default() -> Self {
        Parametros {
            aht_s: AHT_FILA_S,
            paciencia_s: PACIENCIA_S,
            alvo_s: ALVO_FILA_S,
            meta: META,
        }
    }
}

/// Resolve o caminho do dado: base opcional em ESTOCOLOGIA_DADOS, senao o repositorio.
pub fn caminho_de_dados(padrao: &str) -> String {
    if let Ok(base) = std::env::var("ESTOCOLOGIA_DADOS") {
        return format!("{}/{}", base.trim_end_matches('/'), padrao);
    }
    for prefixo in ["", "../", "../../"] {
        let caminho = format!("{prefixo}{padrao}");
        if std::path::Path::new(&caminho).exists() {
            return caminho;
        }
    }
    padrao.to_string()
}

/// Mes de uma data ISO (YYYY-MM-DD).
pub fn mes_da(data: &str) -> u32 {
    data.get(5..7).and_then(|m| m.parse().ok()).unwrap_or(0)
}

/// Dia da semana de uma data ISO: 0 = segunda, 6 = domingo (algoritmo de Sakamoto).
pub fn dia_da_semana(data: &str) -> usize {
    let ano: i32 = data.get(0..4).and_then(|x| x.parse().ok()).unwrap_or(0);
    let m: i32 = data.get(5..7).and_then(|x| x.parse().ok()).unwrap_or(1);
    let d: i32 = data.get(8..10).and_then(|x| x.parse().ok()).unwrap_or(1);
    const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if m < 3 { ano - 1 } else { ano };
    let w = (y + y / 4 - y / 100 + y / 400 + T[(m - 1).clamp(0, 11) as usize] + d).rem_euclid(7);
    // Sakamoto devolve 0 = domingo; aqui 0 = segunda.
    ((w + 6).rem_euclid(7)) as usize
}

/// Um dia da base na grade de 5 min.
#[derive(Debug, Clone)]
pub struct DiaBins {
    pub data: String,
    pub bins: [f64; BINS],
    pub total: f64,
}

/// Le a base (rownames,DateTime,Calls) na grade de 5 min de 07:00 a 21:00.
pub fn carregar_bins(caminho: &str) -> Vec<DiaBins> {
    let texto = std::fs::read_to_string(caminho).expect("nao li o CSV do banco");
    let mut por_dia: std::collections::BTreeMap<String, [f64; BINS]> =
        std::collections::BTreeMap::new();
    for linha in texto.lines().skip(1) {
        let campos: Vec<&str> = linha.split(',').collect();
        if campos.len() < 3 {
            continue;
        }
        let carimbo = campos[1];
        if carimbo.len() < 16 {
            continue;
        }
        let data = &carimbo[..10];
        let hora: usize = carimbo[11..13].parse().unwrap_or(0);
        let minuto: usize = carimbo[14..16].parse().unwrap_or(0);
        if !(7..=21).contains(&hora) {
            continue;
        }
        let i = ((hora - 7) * 60 + minuto) / 5;
        if i >= BINS {
            continue;
        }
        let chamadas: f64 = campos[2].parse().unwrap_or(0.0);
        por_dia.entry(data.to_string()).or_insert([0.0; BINS])[i] += chamadas;
    }
    por_dia
        .into_iter()
        .map(|(data, bins)| DiaBins {
            data,
            bins,
            total: bins.iter().sum(),
        })
        .filter(|d| d.total > 0.0)
        .collect()
}

/// Treino (marco a junho) e cofre (julho a outubro), como o registro.
pub fn separar_treino_cofre(dias: &[DiaBins]) -> (Vec<DiaBins>, Vec<DiaBins>) {
    let treino = dias
        .iter()
        .filter(|d| (3..=6).contains(&mes_da(&d.data)))
        .cloned()
        .collect();
    let cofre = dias
        .iter()
        .filter(|d| (7..=10).contains(&mes_da(&d.data)))
        .cloned()
        .collect();
    (treino, cofre)
}

/// Uniforme em (0, 1).
fn uniforme01(rng: &mut banco::Rng) -> f64 {
    (rng.uniforme(1u64 << 53) as f64 + 0.5) / DOIS53
}

/// Exponencial com a media pedida (infinito quando a media e infinita: ninguem desiste).
fn exponencial(rng: &mut banco::Rng, media: f64) -> f64 {
    if !media.is_finite() || media <= 0.0 {
        return f64::INFINITY;
    }
    -media * uniforme01(rng).ln()
}

/// Semente de um dia e uma replica, pela finalizacao do SplitMix64.
fn semente_do_dia(base: u64, dia: usize, replica: usize) -> u64 {
    let mut z = (base
        ^ (dia as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ (replica as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F))
    .wrapping_sub(0x9E37_79B9_7F4A_7C15);
    stochology_core::rng::passo_splitmix64(&mut z)
}

/// Primeiro e ultimo intervalo de cada grupo de escala.
fn faixa_do_grupo(j: usize) -> (usize, usize) {
    (j * POR_GRUPO, ((j + 1) * POR_GRUPO).min(BINS))
}

/// Agente-horas de um dia: cada grupo vale a sua duracao em horas.
pub fn agente_horas(c: &[u32; GRUPOS]) -> f64 {
    (0..GRUPOS)
        .map(|j| {
            let (a, b) = faixa_do_grupo(j);
            c[j] as f64 * ((b - a) as f64 * BIN_S / 3600.0)
        })
        .sum()
}

/// Menor escala que cumpre a meta em cada grupo, pela formula da alegacao (Erlang C) sobre o
/// perfil de chegadas por intervalo de 5 min, com a demanda multiplicada por `fator`.
pub fn escala_do_perfil(perfil: &[f64; BINS], fator: f64, p: &Parametros) -> [u32; GRUPOS] {
    let mut c = [1_u32; GRUPOS];
    for j in 0..GRUPOS {
        let (a, b) = faixa_do_grupo(j);
        let n = (b - a) as f64;
        let por_segundo = perfil[a..b].iter().sum::<f64>() / (n * BIN_S);
        c[j] = banco::agentes_para_meta(fator * por_segundo * 3600.0, p.aht_s, p.alvo_s, p.meta);
    }
    c
}

/// Perfil medio do treino, em chamadas por intervalo de 5 min.
pub fn perfil_medio(treino: &[DiaBins]) -> [f64; BINS] {
    assert!(!treino.is_empty(), "treino vazio");
    let mut media = [0.0_f64; BINS];
    for d in treino {
        for i in 0..BINS {
            media[i] += d.bins[i];
        }
    }
    media.map(|x| x / treino.len() as f64)
}

/// Perfil de cada dia da semana (0 = segunda) e quantos dias o sustentam. Dia da semana sem
/// dia no treino cai no perfil medio (declarado: a base nao tem sabado nem domingo).
pub fn perfis_por_dia_da_semana(treino: &[DiaBins]) -> ([[f64; BINS]; 7], [usize; 7]) {
    let mut soma = [[0.0_f64; BINS]; 7];
    let mut contagem = [0usize; 7];
    for d in treino {
        let w = dia_da_semana(&d.data);
        contagem[w] += 1;
        for i in 0..BINS {
            soma[w][i] += d.bins[i];
        }
    }
    let media = perfil_medio(treino);
    let mut perfis = [[0.0_f64; BINS]; 7];
    for w in 0..7 {
        for i in 0..BINS {
            perfis[w][i] = if contagem[w] == 0 {
                media[i]
            } else {
                soma[w][i] / contagem[w] as f64
            };
        }
    }
    (perfis, contagem)
}

/// Desvio padrao amostral.
fn dp(xs: &[f64]) -> f64 {
    if xs.len() < 2 {
        return 0.0;
    }
    let m = xs.iter().sum::<f64>() / xs.len() as f64;
    (xs.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (xs.len() as f64 - 1.0)).sqrt()
}

/// O CV residual do registro: desvio padrao da razao entre o total do dia e a media do total
/// do seu dia da semana, no treino (o "6%" do capitulo 16). E o que a margem de R2 escala.
pub fn cv_residual_diario(treino: &[DiaBins]) -> f64 {
    let (perfis, contagem) = perfis_por_dia_da_semana(treino);
    let mut razoes = Vec::new();
    for d in treino {
        let w = dia_da_semana(&d.data);
        if contagem[w] == 0 {
            continue;
        }
        let base: f64 = perfis[w].iter().sum();
        if base > 0.0 {
            razoes.push(d.total / base);
        }
    }
    dp(&razoes)
}

/// Fila minima de tempos de conclusao de atendimento (motor de eventos).
#[derive(Debug, Clone, Copy, PartialEq)]
struct Tempo(f64);

impl Eq for Tempo {}

impl PartialOrd for Tempo {
    fn partial_cmp(&self, outro: &Self) -> Option<Ordering> {
        Some(self.cmp(outro))
    }
}

impl Ord for Tempo {
    fn cmp(&self, outro: &Self) -> Ordering {
        // invertido: o BinaryHeap da std e max-heap
        outro.0.total_cmp(&self.0)
    }
}

/// Fila minima de prazos de paciencia, com o identificador da chamada (motor de eventos).
#[derive(Debug, Clone, Copy)]
struct Prazo {
    t: f64,
    id: u32,
}

impl PartialEq for Prazo {
    fn eq(&self, outro: &Self) -> bool {
        self.t.to_bits() == outro.t.to_bits() && self.id == outro.id
    }
}

impl Eq for Prazo {}

impl PartialOrd for Prazo {
    fn partial_cmp(&self, outro: &Self) -> Option<Ordering> {
        Some(self.cmp(outro))
    }
}

impl Ord for Prazo {
    fn cmp(&self, outro: &Self) -> Ordering {
        outro.t.total_cmp(&self.t).then(outro.id.cmp(&self.id))
    }
}

/// O que um dia de cofre rendeu. As esperas sao as de todas as chamadas oferecidas: quem
/// desiste entra com a propria paciencia, como no instrumento aposentado.
#[derive(Debug, Clone, Copy, Default)]
pub struct MedidaDia {
    pub chegadas: u64,
    pub dentro_do_alvo: u64,
    pub desistencias: u64,
    pub em_espera_no_fim: u64,
    pub espera_media_s: f64,
    pub espera_mediana_s: f64,
    pub espera_p90_s: f64,
    pub espera_media_de_quem_desiste_s: f64,
}

impl MedidaDia {
    /// Nivel de servico: fracao das chamadas oferecidas atendidas em ate o alvo.
    pub fn nivel_de_servico(&self) -> f64 {
        if self.chegadas == 0 {
            1.0
        } else {
            self.dentro_do_alvo as f64 / self.chegadas as f64
        }
    }

    /// Desistencias em cada 100 chamadas oferecidas.
    pub fn desistencias_por_100(&self) -> f64 {
        if self.chegadas == 0 {
            0.0
        } else {
            100.0 * self.desistencias as f64 / self.chegadas as f64
        }
    }
}

/// Quantil por posto mais proximo, sobre um vetor ja ordenado.
fn quantil(ordenada: &[f64], q: f64) -> f64 {
    if ordenada.is_empty() {
        return 0.0;
    }
    let posto = ((q * ordenada.len() as f64).ceil() as usize).max(1) - 1;
    ordenada[posto.min(ordenada.len() - 1)]
}

/// Fecha as estatisticas de um dia a partir das esperas de todas as chamadas oferecidas e da
/// marca de quem desistiu.
fn resumir(
    esperas: &mut [f64],
    desistentes: &[f64],
    em_espera_no_fim: u64,
    p: &Parametros,
) -> MedidaDia {
    let chegadas = esperas.len() as u64;
    let dentro = esperas.iter().filter(|e| **e <= p.alvo_s).count() as u64;
    let media = if esperas.is_empty() {
        0.0
    } else {
        esperas.iter().sum::<f64>() / esperas.len() as f64
    };
    esperas.sort_by(f64::total_cmp);
    MedidaDia {
        chegadas,
        dentro_do_alvo: dentro,
        desistencias: desistentes.len() as u64,
        em_espera_no_fim,
        espera_media_s: media,
        espera_mediana_s: quantil(esperas, 0.5),
        espera_p90_s: quantil(esperas, 0.9),
        espera_media_de_quem_desiste_s: if desistentes.is_empty() {
            0.0
        } else {
            desistentes.iter().sum::<f64>() / desistentes.len() as f64
        },
    }
}

/// Motor exato do instrumento aposentado: as chegadas observadas do dia, espalhadas dentro de
/// cada intervalo de 5 min, atendimento exponencial, paciencia exponencial e FCFS sem
/// preempcao. Quem sofreria espera maior que a propria paciencia desiste e nao ocupa ninguem;
/// quem nao desiste ocupa o atendente de menor tempo livre (o de menor indice nos empates).
pub fn fila_exata(dia: &DiaBins, c: &[u32; GRUPOS], p: &Parametros, semente: u64) -> MedidaDia {
    let mut rng = banco::Rng::novo(semente);
    let n: usize = dia.bins.iter().map(|x| x.round()).sum::<f64>() as usize;

    let mut chegadas = Vec::with_capacity(n);
    for i in 0..BINS {
        let k = dia.bins[i].round() as usize;
        for _ in 0..k {
            chegadas.push(i as f64 * BIN_S + uniforme01(&mut rng) * BIN_S);
        }
    }
    let servico: Vec<f64> = (0..n).map(|_| exponencial(&mut rng, p.aht_s)).collect();
    let paciencia: Vec<f64> = (0..n)
        .map(|_| {
            if p.paciencia_s.is_finite() {
                exponencial(&mut rng, p.paciencia_s)
            } else {
                f64::INFINITY
            }
        })
        .collect();

    // em que grupo de escala o atendente k esta de turno, a partir de cada grupo
    let k_max = c.iter().copied().max().unwrap_or(0) as usize;
    let mut nxt = vec![-1_i32; k_max * GRUPOS];
    for k in 0..k_max {
        let mut prox = -1_i32;
        for j in (0..GRUPOS).rev() {
            if (k as u32) < c[j] {
                prox = j as i32;
            }
            nxt[k * GRUPOS + j] = prox;
        }
    }
    let mut livre = vec![0.0_f64; k_max];

    let mut esperas = Vec::with_capacity(n);
    let mut desistentes = Vec::new();
    for i in 0..n {
        let a = chegadas[i];
        let mut melhor = f64::INFINITY;
        let mut escolhido = usize::MAX;
        for k in 0..k_max {
            let t0 = if livre[k] > a { livre[k] } else { a };
            let j = (t0 / GRUPO_S) as usize;
            let v = if j >= GRUPOS {
                if (k as u32) < c[GRUPOS - 1] {
                    t0
                } else {
                    f64::INFINITY
                }
            } else if (k as u32) < c[j] {
                t0
            } else {
                let x = nxt[k * GRUPOS + j];
                if x >= 0 {
                    x as f64 * GRUPO_S
                } else {
                    f64::INFINITY
                }
            };
            if v < melhor {
                melhor = v;
                escolhido = k;
            }
            if v == a {
                // atendente ja livre no instante da chegada: e ele, e ninguem mais barato
                break;
            }
        }
        if !melhor.is_finite() || melhor - a > paciencia[i] {
            // quem desiste espera a propria paciencia; sem atendente disponivel no dia, a
            // espera fica infinita (o instrumento aposentado gravava o mesmo, via -1 do numpy)
            let e = if melhor.is_finite() {
                paciencia[i]
            } else {
                f64::INFINITY
            };
            esperas.push(e);
            desistentes.push(e);
        } else {
            esperas.push(melhor - a);
            livre[escolhido] = melhor + servico[i];
        }
    }
    resumir(&mut esperas, &desistentes, 0, p)
}

/// Le a frente da fila a proxima chamada que ainda espera (as que desistiram ficam para tras e
/// sao descartadas na leitura).
fn proximo_da_fila(fila: &mut VecDeque<u32>, estado: &[u8]) -> Option<u32> {
    while let Some(id) = fila.pop_front() {
        if estado[id as usize] == 0 {
            return Some(id);
        }
    }
    None
}

/// Motor de verificacao: M/M/c+M evento a evento sobre a mesma grade. Chegadas de Poisson com
/// a taxa observada de cada intervalo de 5 min, escala constante no grupo, atendimento e
/// paciencia exponenciais. A fila atravessa os intervalos; quem ainda espera as 21:00 fica
/// declarado em `em_espera_no_fim`.
pub fn fila_eventos(dia: &DiaBins, c: &[u32; GRUPOS], p: &Parametros, semente: u64) -> MedidaDia {
    let mut rng = banco::Rng::novo(semente);
    let mut estado: Vec<u8> = Vec::new(); // 0 espera, 1 em atendimento, 2 desistiu
    let mut chegada: Vec<f64> = Vec::new();
    let mut esperas: Vec<f64> = Vec::new();
    let mut desistentes: Vec<f64> = Vec::new();
    let mut fila: VecDeque<u32> = VecDeque::new();
    let mut servicos: BinaryHeap<Tempo> = BinaryHeap::new();
    let mut prazos: BinaryHeap<Prazo> = BinaryHeap::new();
    let mut t = 0.0_f64;
    let mut proxima_chegada: f64;

    for i in 0..BINS {
        let capacidade = c[i / POR_GRUPO] as usize;
        let lam = dia.bins[i] / BIN_S;
        proxima_chegada =
            t + exponencial(&mut rng, if lam > 0.0 { 1.0 / lam } else { f64::INFINITY });
        let fim = (i + 1) as f64 * BIN_S;
        loop {
            while servicos.len() < capacidade {
                match proximo_da_fila(&mut fila, &estado) {
                    Some(id) => {
                        servicos.push(Tempo(t + exponencial(&mut rng, p.aht_s)));
                        estado[id as usize] = 1;
                        esperas.push(t - chegada[id as usize]);
                    }
                    None => break,
                }
            }
            // proximo evento: 0 atendimento, 1 desistencia, 2 chegada, 3 fim do intervalo
            let mut evento = 3u8;
            let mut quando = fim;
            if let Some(s) = servicos.peek()
                && s.0 < quando
            {
                quando = s.0;
                evento = 0;
            }
            if let Some(d) = prazos.peek()
                && d.t < quando
            {
                quando = d.t;
                evento = 1;
            }
            if proxima_chegada < quando {
                quando = proxima_chegada;
                evento = 2;
            }
            match evento {
                0 => {
                    servicos.pop();
                }
                1 => {
                    let d = prazos.pop().expect("prazo na fila");
                    if estado[d.id as usize] == 0 {
                        estado[d.id as usize] = 2;
                        let e = quando - chegada[d.id as usize];
                        esperas.push(e);
                        desistentes.push(e);
                    }
                }
                2 => {
                    t = quando;
                    let id = estado.len() as u32;
                    estado.push(0);
                    chegada.push(t);
                    fila.push_back(id);
                    prazos.push(Prazo {
                        t: t + exponencial(&mut rng, p.paciencia_s),
                        id,
                    });
                    proxima_chegada = t + exponencial(
                        &mut rng,
                        if lam > 0.0 { 1.0 / lam } else { f64::INFINITY },
                    );
                }
                _ => break,
            }
            if evento != 2 {
                t = quando;
            }
        }
    }
    let em_espera_no_fim = estado.iter().filter(|e| **e == 0).count() as u64;
    resumir(&mut esperas, &desistentes, em_espera_no_fim, p)
}

/// As quatro escalas, grupo a grupo, e a margem residual que R2 usa.
#[derive(Debug, Clone)]
pub struct Escalas {
    pub r0: [u32; GRUPOS],
    pub r1: [[u32; GRUPOS]; 7],
    pub r2: [[u32; GRUPOS]; 7],
    pub r3: [u32; GRUPOS],
    pub extra_r3: u32,
    pub cv_residual: f64,
    pub fator_r2: f64,
    pub dias_por_dia_da_semana: [usize; 7],
}

impl Escalas {
    /// A escala que a regra aplica naquele dia.
    pub fn escala(&self, regra: Regra, dia: &DiaBins) -> &[u32; GRUPOS] {
        let w = dia_da_semana(&dia.data);
        match regra {
            Regra::R0 => &self.r0,
            Regra::R1 => &self.r1[w],
            Regra::R2 => &self.r2[w],
            Regra::R3 => &self.r3,
        }
    }
}

/// Monta as quatro escalas: R0 no perfil medio do treino, R1 no perfil do dia da semana, R2
/// com a demanda multiplicada por `1 + Z_MARGEM * cv` e R3 com R0 mais o menor numero de
/// atendentes por igual que iguala o custo de R2 no cofre.
pub fn montar_escalas(treino: &[DiaBins], cofre: &[DiaBins], p: &Parametros) -> Escalas {
    let cv = cv_residual_diario(treino);
    let fator = 1.0 + Z_MARGEM * cv;
    let r0 = escala_do_perfil(&perfil_medio(treino), 1.0, p);
    let (perfis, contagem) = perfis_por_dia_da_semana(treino);
    let mut r1 = [[1_u32; GRUPOS]; 7];
    let mut r2 = [[1_u32; GRUPOS]; 7];
    for w in 0..7 {
        r1[w] = escala_do_perfil(&perfis[w], 1.0, p);
        r2[w] = escala_do_perfil(&perfis[w], fator, p);
    }
    let custo_r2 = if cofre.is_empty() {
        0.0
    } else {
        cofre
            .iter()
            .map(|d| agente_horas(&r2[dia_da_semana(&d.data)]))
            .sum::<f64>()
            / cofre.len() as f64
    };
    let mut extra = 0_u32;
    while extra < 1_000 && agente_horas(&r0.map(|x| x + extra)) < custo_r2 {
        extra += 1;
    }
    Escalas {
        r0,
        r1,
        r2,
        r3: r0.map(|x| x + extra),
        extra_r3: extra,
        cv_residual: cv,
        fator_r2: fator,
        dias_por_dia_da_semana: contagem,
    }
}

/// O que uma regra rendeu no cofre.
#[derive(Debug, Clone, Copy)]
pub struct MedidaRegra {
    pub regra: Regra,
    pub motor: Motor,
    pub agente_horas_dia: f64,
    pub pico: u32,
    pub nivel_de_servico: f64,
    pub nivel_medio_dos_dias: f64,
    pub dias_abaixo_da_meta: f64,
    pub desistencias_por_100: f64,
    pub espera_media_s: f64,
    pub espera_mediana_s: f64,
    pub espera_p90_s: f64,
    pub em_espera_no_fim: f64,
}

/// Mede uma regra no cofre: uma fila por dia e por replica, com semente fixa.
pub fn medir_regra(
    cofre: &[DiaBins],
    esc: &Escalas,
    regra: Regra,
    semente: u64,
    replicas: usize,
    motor: Motor,
    p: &Parametros,
) -> MedidaRegra {
    let replicas = replicas.max(1);
    let mut soma_chamadas = 0.0;
    let mut soma_dentro = 0.0;
    let mut soma_desistencias = 0.0;
    let mut soma_espera = 0.0;
    let mut esperas_medias: Vec<f64> = Vec::new();
    let mut esperas_medianas: Vec<f64> = Vec::new();
    let mut esperas_p90: Vec<f64> = Vec::new();
    let mut em_espera = 0.0;
    let mut soma_nivel_dias = 0.0;
    let mut dias_abaixo = 0.0;
    let mut horas = 0.0;
    let mut pico = 0_u32;

    for (d, dia) in cofre.iter().enumerate() {
        let escala = esc.escala(regra, dia);
        horas += agente_horas(escala);
        pico = pico.max(escala.iter().copied().max().unwrap_or(0));
        let mut nivel_do_dia = 0.0;
        for r in 0..replicas {
            let semente_dia = semente_do_dia(semente, d, r);
            let m = match motor {
                Motor::Exato => fila_exata(dia, escala, p, semente_dia),
                Motor::Eventos => fila_eventos(dia, escala, p, semente_dia),
            };
            soma_chamadas += m.chegadas as f64;
            soma_dentro += m.dentro_do_alvo as f64;
            soma_desistencias += m.desistencias as f64;
            soma_espera += m.espera_media_s;
            em_espera += m.em_espera_no_fim as f64;
            esperas_medias.push(m.espera_media_s);
            esperas_medianas.push(m.espera_mediana_s);
            esperas_p90.push(m.espera_p90_s);
            nivel_do_dia += m.nivel_de_servico();
        }
        nivel_do_dia /= replicas as f64;
        soma_nivel_dias += nivel_do_dia;
        if nivel_do_dia < p.meta {
            dias_abaixo += 1.0;
        }
    }
    let n = cofre.len() as f64;
    MedidaRegra {
        regra,
        motor,
        agente_horas_dia: horas / n,
        pico,
        nivel_de_servico: soma_dentro / soma_chamadas,
        nivel_medio_dos_dias: soma_nivel_dias / n,
        dias_abaixo_da_meta: dias_abaixo / n,
        desistencias_por_100: 100.0 * soma_desistencias / soma_chamadas,
        espera_media_s: soma_espera / (n * replicas as f64),
        espera_mediana_s: quantil(&esperas_medianas, 0.5),
        espera_p90_s: quantil(&esperas_p90, 0.5),
        em_espera_no_fim: em_espera / (n * replicas as f64),
    }
}

/// O relatorio completo do H0 real.
#[derive(Debug, Clone)]
pub struct Relatorio {
    pub treino: usize,
    pub cofre: usize,
    pub chamadas_cofre: f64,
    pub replicas: usize,
    pub semente: u64,
    pub cv_residual: f64,
    pub fator_r2: f64,
    pub extra_r3: u32,
    pub dias_por_dia_da_semana: [usize; 7],
    pub motor: Motor,
    pub medidas: Vec<MedidaRegra>,
}

/// Carrega a base, separa treino e cofre e mede as quatro regras num motor de fila.
pub fn medir_regras(
    caminho: &str,
    semente: u64,
    replicas: usize,
    motor: Motor,
    p: &Parametros,
) -> Relatorio {
    let dias = carregar_bins(caminho);
    let (treino, cofre) = separar_treino_cofre(&dias);
    assert!(
        !treino.is_empty() && !cofre.is_empty(),
        "treino ou cofre vazio na base"
    );
    let escalas = montar_escalas(&treino, &cofre, p);
    let medidas = Regra::TODAS
        .iter()
        .map(|r| medir_regra(&cofre, &escalas, *r, semente, replicas, motor, p))
        .collect();
    Relatorio {
        treino: treino.len(),
        cofre: cofre.len(),
        chamadas_cofre: cofre.iter().map(|d| d.total).sum(),
        replicas,
        semente,
        cv_residual: escalas.cv_residual,
        fator_r2: escalas.fator_r2,
        extra_r3: escalas.extra_r3,
        dias_por_dia_da_semana: escalas.dias_por_dia_da_semana,
        motor,
        medidas,
    }
}

/// Imprime a cabeca do relatorio, comum aos dois motores.
fn imprimir_cabeca(rel: &Relatorio, p: &Parametros) {
    println!("== H0 real :: a fila com desistencia e as quatro regras de escala (R0 a R3) ==");
    println!(
        "treino {} dias (mar-jun) | cofre {} dias (jul-out) | {:.0} chamadas no cofre",
        rel.treino, rel.cofre, rel.chamadas_cofre
    );
    println!(
        "atendimento exponencial {:.0} s | paciencia exponencial {:.0} s | alvo {:.0} s | meta {:.0}% | semente {} | {} replicas por dia",
        p.aht_s,
        p.paciencia_s,
        p.alvo_s,
        p.meta * 100.0,
        rel.semente,
        rel.replicas
    );
    println!(
        "CV residual do dia {:.4} | margem de R2 = 1 + {:.4} x CV = {:.4} | R3 = R0 + {} atendentes por igual",
        rel.cv_residual, Z_MARGEM, rel.fator_r2, rel.extra_r3
    );
    println!(
        "dias no treino por dia da semana {:?}",
        rel.dias_por_dia_da_semana
    );
}

/// Imprime a tabela de um motor.
fn imprimir_tabela(medidas: &[MedidaRegra]) {
    println!(
        "{:<5} {:>17} {:>6} {:>17} {:>22} {:>19} {:>15} {:>12} {:>11}",
        "regra",
        "agente-horas/dia",
        "pico",
        "nivel de servico",
        "dias abaixo da meta",
        "desistencia/100",
        "espera media s",
        "espera med. s",
        "espera p90 s"
    );
    for m in medidas {
        println!(
            "{:<5} {:>17.0} {:>6} {:>16.1}% {:>21.1}% {:>19.1} {:>15.0} {:>12.0} {:>11.0}",
            m.regra.nome(),
            m.agente_horas_dia,
            m.pico,
            m.nivel_de_servico * 100.0,
            m.dias_abaixo_da_meta * 100.0,
            m.desistencias_por_100,
            m.espera_media_s,
            m.espera_mediana_s,
            m.espera_p90_s
        );
    }
}

/// Imprime o H0 real inteiro: as quatro regras nos dois motores de fila.
pub fn rodar() {
    let caminho = caminho_de_dados("dados-reais/bank_calls.csv");
    let p = Parametros::default();
    let exato = medir_regras(&caminho, SEMENTE, 3, Motor::Exato, &p);
    imprimir_cabeca(&exato, &p);
    println!();
    println!("motor Exato (chegadas observadas, FCFS com desistencia) — a reconstrucao");
    imprimir_tabela(&exato.medidas);
    println!();
    println!(
        "motor Eventos (M/M/c+M evento a evento, chegadas de Poisson) — verificacao independente"
    );
    let eventos = medir_regras(&caminho, SEMENTE, 3, Motor::Eventos, &p);
    imprimir_tabela(&eventos.medidas);
}

#[cfg(test)]
mod testes {
    use super::*;
    use std::sync::OnceLock;

    /// A base do repositorio; sem ela os testes de medida ficam sem dado e se declaram pulados.
    fn caminho() -> Option<String> {
        let c = caminho_de_dados("dados-reais/bank_calls.csv");
        if std::path::Path::new(&c).exists() {
            Some(c)
        } else {
            None
        }
    }

    /// Uma medida so, reaproveitada pelos testes: semente fixa, uma replica por dia, como o
    /// instrumento aposentado.
    fn relatorio() -> Option<&'static Relatorio> {
        static REL: OnceLock<Option<Relatorio>> = OnceLock::new();
        REL.get_or_init(|| {
            let c = caminho()?;
            Some(medir_regras(
                &c,
                SEMENTE,
                1,
                Motor::Exato,
                &Parametros::default(),
            ))
        })
        .as_ref()
    }

    /// Dia sintetico com a mesma contagem em todos os intervalos, para forcar sobrecarga.
    fn dia_sobrecarregado(por_intervalo: f64) -> DiaBins {
        let bins = [por_intervalo; BINS];
        DiaBins {
            data: "2003-07-01".to_string(),
            bins,
            total: bins.iter().sum(),
        }
    }

    #[test]
    fn dia_da_semana_de_datas_conhecidas() {
        assert_eq!(dia_da_semana("2000-01-01"), 5, "sabado");
        assert_eq!(dia_da_semana("2024-01-01"), 0, "segunda");
        assert_eq!(
            dia_da_semana("2003-03-03"),
            0,
            "a primeira segunda do treino"
        );
        assert_eq!(dia_da_semana("2003-08-31"), 6, "domingo");
    }

    #[test]
    fn grade_e_o_recorte_do_instrumento() {
        // 169 intervalos de 5 min de 07:00 a 21:00, 28 meias horas mais a cauda de um intervalo
        assert_eq!(BINS, 169);
        assert_eq!(POR_GRUPO, 6);
        assert_eq!(GRUPOS, 29);
        assert_eq!(faixa_do_grupo(27), (162, 168));
        assert_eq!(faixa_do_grupo(28), (168, 169));
        // agente-horas de uma escala constante: 14,0833 h de turno por atendente
        let uniforme = [10_u32; GRUPOS];
        let esperado = 10.0 * BINS as f64 * BIN_S / 3600.0;
        assert!(
            (agente_horas(&uniforme) - esperado).abs() < 1e-9,
            "{}",
            agente_horas(&uniforme)
        );
    }

    #[test]
    fn fila_com_desistencia_estabiliza() {
        let p = Parametros::default();
        // 1000 chegadas por intervalo de 5 min contra 40 atendentes, que atendem 15/hora cada:
        // a carga oferecida e o dobro da capacidade, o pior caso para uma fila sem desistencia
        let dia = dia_sobrecarregado(1000.0);
        let escala = [40_u32; GRUPOS];

        // 1. o motor exato: com paciencia, a espera para no teto da paciencia
        let com = fila_exata(&dia, &escala, &p, 7);
        assert!(
            com.desistencias_por_100() > 50.0,
            "desistencia {}",
            com.desistencias_por_100()
        );
        assert!(
            com.espera_p90_s <= 3.0 * p.paciencia_s,
            "p90 da espera {} contra paciencia {}",
            com.espera_p90_s,
            p.paciencia_s
        );
        let sem = Parametros {
            paciencia_s: f64::INFINITY,
            ..p
        };
        let sem_desistir = fila_exata(&dia, &escala, &sem, 7);
        assert_eq!(sem_desistir.desistencias, 0);
        assert!(
            sem_desistir.espera_media_s > 10.0 * com.espera_media_s,
            "sem desistencia {:.0} s contra {:.0} s",
            sem_desistir.espera_media_s,
            com.espera_media_s
        );

        // 2. o motor de eventos: a fila estabiliza com paciencia e cresce sem ela
        let ev_com = fila_eventos(&dia, &escala, &p, 7);
        let ev_sem = fila_eventos(&dia, &escala, &sem, 7);
        // o excedente e de 3 chamadas por segundo contra 0,167 atendida por segundo, e a fila
        // que estabiliza assenta em (excedente) x (paciencia) = cerca de 570 esperando
        assert!(
            (200..=2000).contains(&ev_com.em_espera_no_fim),
            "fila assentada em {}",
            ev_com.em_espera_no_fim
        );
        assert!(ev_com.desistencias_por_100() > 50.0);
        assert!(
            ev_sem.em_espera_no_fim > 10_000,
            "sem desistencia a fila devia acumular, sobrou {}",
            ev_sem.em_espera_no_fim
        );
        // a fila assentada e proporcional a paciencia: com 90 s de paciencia a espera de
        // equilibrio cai para cerca da metade, que e a assinatura de uma fila com saida
        let curta = Parametros {
            paciencia_s: 90.0,
            ..p
        };
        let ev_curta = fila_eventos(&dia, &escala, &curta, 7);
        let razao = ev_curta.em_espera_no_fim as f64 / ev_com.em_espera_no_fim as f64;
        assert!(
            (0.3..=0.7).contains(&razao),
            "razao das filas assentadas {razao}"
        );
        assert!(ev_sem.em_espera_no_fim > 5 * ev_com.em_espera_no_fim);
    }

    #[test]
    fn monotonicidade_r2_melhor_que_r0() {
        let Some(rel) = relatorio() else {
            eprintln!("sem a base");
            return;
        };
        let (r0, r1, r2, r3) = (
            &rel.medidas[0],
            &rel.medidas[1],
            &rel.medidas[2],
            &rel.medidas[3],
        );
        assert!(
            r2.dias_abaixo_da_meta < r0.dias_abaixo_da_meta,
            "R2 {:.3} dias falhos contra R0 {:.3}",
            r2.dias_abaixo_da_meta,
            r0.dias_abaixo_da_meta
        );
        assert!(r2.nivel_de_servico > r0.nivel_de_servico);
        assert!(r2.agente_horas_dia > r0.agente_horas_dia);
        assert!(
            r1.dias_abaixo_da_meta <= r0.dias_abaixo_da_meta,
            "R1 pior que a alegacao"
        );
        assert!(
            r3.dias_abaixo_da_meta <= r0.dias_abaixo_da_meta,
            "R3 pior que a alegacao"
        );
        assert!(
            r2.dias_abaixo_da_meta <= r3.dias_abaixo_da_meta,
            "R2 nao venceu R3"
        );
        // R2 e R3 custam quase o mesmo, e por isso a comparacao e de alocacao, nao de gasto
        let razao = r3.agente_horas_dia / r2.agente_horas_dia;
        assert!((0.98..=1.02).contains(&razao), "R3/R2 = {razao}");
    }

    #[test]
    fn a_margem_de_r2_e_o_p90_do_cv_residual() {
        let Some(c) = caminho() else {
            return;
        };
        let p = Parametros::default();
        let dias = carregar_bins(&c);
        let (treino, cofre) = separar_treino_cofre(&dias);
        let esc = montar_escalas(&treino, &cofre, &p);
        // o CV residual do capitulo 16 e de 6%; o instrumento mede perto disso
        assert!(
            esc.cv_residual > 0.04 && esc.cv_residual < 0.08,
            "cv {}",
            esc.cv_residual
        );
        let esperado = 1.0 + Z_MARGEM * esc.cv_residual;
        assert!((esc.fator_r2 - esperado).abs() < 1e-12);
        assert!(
            (1.05..=1.10).contains(&esc.fator_r2),
            "fator {}",
            esc.fator_r2
        );
        // R3 e "R0 mais atendentes por igual": a diferenca e constante, e e o extra medido
        assert!(
            esc.extra_r3 >= 8 && esc.extra_r3 <= 16,
            "extra {}",
            esc.extra_r3
        );
        for j in 0..GRUPOS {
            assert_eq!(esc.r3[j] - esc.r0[j], esc.extra_r3);
        }
        // a margem e alocada onde esta o risco: o pico de R2 passa o de R1, que passa o de R0
        let pico = |c: &[u32; GRUPOS]| c.iter().copied().max().unwrap_or(0);
        for w in 0..5 {
            assert!(
                pico(&esc.r2[w]) > pico(&esc.r1[w]),
                "R2 sem folga no pico do dia {w}"
            );
        }
        assert!(pico(&esc.r1[0]) > pico(&esc.r0));
        // R1 e "custo zero": a media dos cinco dias uteis fica dentro de 1% do custo de R0
        let media_r1: f64 = (0..5).map(|w| agente_horas(&esc.r1[w])).sum::<f64>() / 5.0;
        let zero = (media_r1 - agente_horas(&esc.r0)).abs() / agente_horas(&esc.r0);
        assert!(
            zero < 0.01,
            "R1 custa {media_r1:.0} contra {:.0}",
            agente_horas(&esc.r0)
        );
        // e o perfil por dia da semana nao e o perfil medio repetido
        assert_ne!(esc.r1[0], esc.r1[3]);
    }

    #[test]
    fn instrumento_roda_com_semente_fixa() {
        let Some(c) = caminho() else {
            return;
        };
        let p = Parametros::default();
        let dias = carregar_bins(&c);
        let (treino, cofre) = separar_treino_cofre(&dias);
        let esc = montar_escalas(&treino, &cofre, &p);
        let dia = &cofre[0];
        let escala = esc.escala(Regra::R0, dia);
        let a = fila_exata(dia, escala, &p, 42);
        let b = fila_exata(dia, escala, &p, 42);
        assert_eq!(a.chegadas, b.chegadas);
        assert_eq!(a.dentro_do_alvo, b.dentro_do_alvo);
        assert_eq!(a.desistencias, b.desistencias);
        assert_eq!(a.espera_media_s.to_bits(), b.espera_media_s.to_bits());
        assert_eq!(a.espera_p90_s.to_bits(), b.espera_p90_s.to_bits());
        // e o mesmo motor de eventos, tambem
        let c1 = fila_eventos(dia, escala, &p, 42);
        let c2 = fila_eventos(dia, escala, &p, 42);
        assert_eq!(c1.chegadas, c2.chegadas);
        assert_eq!(c1.dentro_do_alvo, c2.dentro_do_alvo);
        assert_eq!(c1.espera_media_s.to_bits(), c2.espera_media_s.to_bits());
        // semente vizinha nao muda a medida do dia alem do ruido do proprio instrumento
        let d1 = fila_exata(dia, escala, &p, 43);
        assert!((d1.nivel_de_servico() - a.nivel_de_servico()).abs() < 0.15);
        // duas rodadas do relatorio inteiro com a mesma semente dao o mesmo numero
        let r1 = medir_regras(&c, SEMENTE, 1, Motor::Exato, &p);
        let r2 = medir_regras(&c, SEMENTE, 1, Motor::Exato, &p);
        assert_eq!(
            r1.medidas[0].nivel_de_servico.to_bits(),
            r2.medidas[0].nivel_de_servico.to_bits()
        );
    }

    #[test]
    fn faixas_do_registro_e_divergencia_declarada() {
        let Some(rel) = relatorio() else {
            eprintln!("sem a base");
            return;
        };
        // o cofre: 81 dias uteis de julho a outubro, 2,65 milhoes de chamadas oferecidas
        assert_eq!(rel.treino, 83);
        assert_eq!(rel.cofre, 81);
        assert!((2_600_000.0..=2_700_000.0).contains(&rel.chamadas_cofre));
        // a grade do instrumento inclui o intervalo das 21:00; a leitura de meias horas do
        // banco.rs o descarta, e por isso a contagem daqui e 5 596 chamadas maior
        assert!(rel.chamadas_cofre > 2_649_354.0);

        for m in &rel.medidas {
            assert!((2100.0..=2600.0).contains(&m.agente_horas_dia), "{m:?}");
            assert!((0.0..=1.0).contains(&m.nivel_de_servico));
            assert!((0.0..=1.0).contains(&m.dias_abaixo_da_meta));
        }
        let r0 = rel.medidas[0];
        let r1 = rel.medidas[1];
        let r2 = rel.medidas[2];
        let r3 = rel.medidas[3];

        // Agente-horas: o custo do registro e reproduzido regra a regra (2 264, 2 261, 2 428,
        // 2 433), dentro de 1%.
        for (m, publicado) in [(r0, 2264.0), (r1, 2261.0), (r2, 2428.0), (r3, 2433.0)] {
            let erro = (m.agente_horas_dia - publicado).abs() / publicado;
            assert!(
                erro < 0.01,
                "{} custa {:.0} contra {:.0}",
                m.regra.nome(),
                m.agente_horas_dia,
                publicado
            );
        }
        assert_eq!(
            rel.extra_r3, 12,
            "R3 e R0 + 12 atendentes por igual no registro"
        );

        // Nivel de servico e dias falhos: aqui a divergencia esta declarada. O registo publica
        // 86/26 (R0), 90/12 (R1), 96/3,7 (R2) e 94/7,4 (R3); a faixa abaixo e generosa de
        // proposito, porque o gerador de numeros e outro e os sorteios caem em outro lugar.
        let ns = |m: &MedidaRegra| m.nivel_de_servico * 100.0;
        assert!((80.0..=90.0).contains(&ns(&r0)), "NS de R0 {:.1}", ns(&r0));
        assert!(
            ns(&r1) >= ns(&r0) - 1.0,
            "R1 {:.1} contra R0 {:.1}",
            ns(&r1),
            ns(&r0)
        );
        assert!(ns(&r2) >= 89.0, "NS de R2 {:.1}", ns(&r2));
        assert!(ns(&r3) >= 88.0, "NS de R3 {:.1}", ns(&r3));
        assert!(
            (0.20..=0.42).contains(&r0.dias_abaixo_da_meta),
            "R0 falha {:.3}",
            r0.dias_abaixo_da_meta
        );
        assert!(
            (0.08..=0.28).contains(&r1.dias_abaixo_da_meta),
            "R1 falha {:.3}",
            r1.dias_abaixo_da_meta
        );
        assert!(
            (0.00..=0.10).contains(&r2.dias_abaixo_da_meta),
            "R2 falha {:.3}",
            r2.dias_abaixo_da_meta
        );
        assert!(
            (0.03..=0.18).contains(&r3.dias_abaixo_da_meta),
            "R3 falha {:.3}",
            r3.dias_abaixo_da_meta
        );
    }

    #[test]
    fn impressao_de_trabalho() {
        let Some(rel) = relatorio() else {
            eprintln!("sem a base");
            return;
        };
        let p = Parametros::default();
        imprimir_cabeca(rel, &p);
        println!("motor Exato (chegadas observadas, FCFS com desistencia)");
        imprimir_tabela(&rel.medidas);
    }
}
