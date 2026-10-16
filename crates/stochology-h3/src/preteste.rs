//! Pre-teste de calibracao do painel SHELF: 20 previsoes resolvidas, com os quatro criterios de entrada.
//!
//! O protocolo de `fundo/p12-shelf` fixa o desenho: 20 itens binarios ja resolvidos, cada um com
//! `p em (0, 1)`, Brier contra a climatologia do proprio pool, curva em caixas de massa igual, ECE e
//! decomposicao de Murphy, e quatro criterios de aprovacao. Este modulo e o instrumento que mede isso.
//!
//! Duas escolhas declaradas:
//!
//! - **A decomposicao usa um balde por valor distinto de `p`.** E o que faz a identidade
//!   `REL - RES + UNC = Brier` fechar exatamente; com caixas de massa igual ela fecha so por aproximacao,
//!   porque dentro de uma caixa os `p` diferem.
//! - **O limiar do ECE nao sai do gosto: sai do nulo.** `nulo_do_pre_teste` simula o especialista
//!   perfeitamente calibrado no mesmo desenho e devolve a distribuicao do ECE que ele obteria. Um limiar
//!   abaixo do p90 desse nulo reprova quem esta calibrado, e o numero fica medido em `labs --preteste`.
//!
//! Declarado e nao implementado: o preenchimento dos 20 itens por dominio (o pool e trabalho de mesa, com
//! fonte conferida) e a leitura dos controles de ancora e de ordem, que dependem da mesa do painel.

use stochology_core::Rng;

/// Caixas da curva declaradas pelo protocolo.
pub const BALDES_DO_PROTOCOLO: usize = 10;

/// Limiar declarado do protocolo para o ECE.
pub const ECE_MAXIMO_DO_PROTOCOLO: f64 = 0.10;

/// Caixas do **portão ajustado** (declaração 1 do ponto 12, 2026-10-16): 20 itens em 5 caixas de massa
/// igual dão 4 itens por caixa, e o critério 3 volta a morder.
pub const BALDES_AJUSTADOS: usize = 5;

/// Limiar do portão ajustado: o p90 medido do ECE de um especialista perfeitamente calibrado nesse
/// desenho (0,2316), arredondado para cima. Ver `labs --preteste`.
pub const ECE_MAXIMO_AJUSTADO: f64 = 0.25;

/// Tamanho minimo de caixa para a regra da secao sistematica morder.
pub const CAIXA_N_MINIMO: usize = 4;

/// Desvio maximo tolerado numa caixa grande.
pub const CAIXA_DESVIO_MAXIMO: f64 = 0.25;

fn uniforme(rng: &mut Rng) -> f64 {
    ((rng.proximo() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
}

/// Uma caixa da curva: quantos itens, a media prevista e a taxa observada.
#[derive(Debug, Clone, PartialEq)]
pub struct Caixa {
    pub indice: usize,
    pub n: usize,
    pub media_prevista: f64,
    pub taxa_observada: f64,
}

impl Caixa {
    pub fn desvio(&self) -> f64 {
        self.taxa_observada - self.media_prevista
    }
}

/// A decomposicao de Murphy: confiabilidade, resolucao e incerteza da climatologia.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Murphy {
    pub rel: f64,
    pub res: f64,
    pub unc: f64,
}

/// O motivo declarado de uma reprovacao no pre-teste.
#[derive(Debug, Clone, PartialEq)]
pub enum Reprovacao {
    HabilidadeNaoPositiva { habilidade: f64 },
    EceAcimaDoLimite { ece: f64, limite: f64 },
    CaixaSistematica { indice: usize, n: usize, desvio: f64 },
    CurvaNaoMonotona { media_da_caixa_baixa: f64, media_da_caixa_alta: f64 },
}

/// O que o pre-teste devolve: as medidas e o veredito.
#[derive(Debug, Clone, PartialEq)]
pub struct PreTeste {
    pub n: usize,
    pub brier: f64,
    pub brier_clima: f64,
    pub habilidade: f64,
    pub ece: f64,
    pub caixas: Vec<Caixa>,
    pub murphy: Murphy,
    pub aprovado: bool,
    pub reprovacoes: Vec<Reprovacao>,
}

/// Indices com `p` fora de (0, 1): o protocolo recusa 0 e 1 exatos.
pub fn probabilidades_invalidas(confianca: &[f64]) -> Vec<usize> {
    confianca
        .iter()
        .enumerate()
        .filter(|(_, p)| !(0.0 < **p && **p < 1.0))
        .map(|(i, _)| i)
        .collect()
}

/// Caixas de massa igual: ordena por `p` e corta em `baldes` grupos do mesmo tamanho.
pub fn caixas_de_massa_igual(confianca: &[f64], acerto: &[bool], baldes: usize) -> Vec<Caixa> {
    if confianca.len() != acerto.len() || confianca.is_empty() || baldes == 0 {
        return Vec::new();
    }
    let mut ordem: Vec<usize> = (0..confianca.len()).collect();
    ordem.sort_by(|a, b| confianca[*a].partial_cmp(&confianca[*b]).unwrap_or(std::cmp::Ordering::Equal));
    let mut caixas = Vec::with_capacity(baldes);
    let n = ordem.len();
    for indice in 0..baldes {
        let inicio = indice * n / baldes;
        let fim = (indice + 1) * n / baldes;
        if inicio >= fim {
            continue;
        }
        let grupo = &ordem[inicio..fim];
        let media_prevista = grupo.iter().map(|i| confianca[*i]).sum::<f64>() / grupo.len() as f64;
        let acertos = grupo.iter().filter(|i| acerto[**i]).count() as f64;
        caixas.push(Caixa {
            indice,
            n: grupo.len(),
            media_prevista,
            taxa_observada: acertos / grupo.len() as f64,
        });
    }
    caixas
}

/// Erro de calibracao esperado nas caixas de massa igual.
pub fn ece_de_massa_igual(confianca: &[f64], acerto: &[bool], baldes: usize) -> f64 {
    let caixas = caixas_de_massa_igual(confianca, acerto, baldes);
    if caixas.is_empty() {
        return f64::NAN;
    }
    let n = confianca.len() as f64;
    caixas.iter().map(|c| (c.n as f64 / n) * c.desvio().abs()).sum()
}

/// O criterio 4 do protocolo, na letra dele: a media prevista da caixa de cima acima da de baixo.
///
/// **Limite medido:** com caixas de massa igual as caixas sao cortadas por `p` ordenado, de modo que a media
/// prevista da caixa de cima e maior que a de baixo **por construcao**. Este criterio nao sabe reprovar — o
/// teste `o_criterio_4_do_protocolo_nao_sabe_reprovar` prova isso — e por isso o diagnostico que morde e
/// `curva_observada_e_fracamente_monotona`, relatado ao lado do criterio e nao no lugar dele.
pub fn curva_fracamente_monotona(caixas: &[Caixa]) -> bool {
    match (caixas.first(), caixas.last()) {
        (Some(baixa), Some(alta)) => alta.media_prevista > baixa.media_prevista,
        _ => false,
    }
}

/// Diagnostico declarado: a taxa **observada** da caixa de cima acima da de baixo. Este sim pode falhar,
/// e e o que le a forma da curva quando o especialista troca as pontas.
pub fn curva_observada_e_fracamente_monotona(caixas: &[Caixa]) -> bool {
    match (caixas.first(), caixas.last()) {
        (Some(baixa), Some(alta)) => alta.taxa_observada > baixa.taxa_observada,
        _ => false,
    }
}

/// Decomposicao de Murphy com um balde por valor distinto de `p` (faz a identidade fechar exata).
pub fn decomposicao_de_murphy(confianca: &[f64], acerto: &[bool]) -> Murphy {
    if confianca.is_empty() || confianca.len() != acerto.len() {
        return Murphy { rel: 0.0, res: 0.0, unc: 0.0 };
    }
    let n = confianca.len() as f64;
    let media: f64 = acerto.iter().filter(|y| **y).count() as f64 / n;
    let unc = media * (1.0 - media);
    let mut valores: Vec<f64> = confianca.to_vec();
    valores.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    valores.dedup();
    let mut rel = 0.0;
    let mut res = 0.0;
    for valor in valores {
        let itens: Vec<usize> = (0..confianca.len()).filter(|i| confianca[*i] == valor).collect();
        if itens.is_empty() {
            continue;
        }
        let peso = itens.len() as f64 / n;
        let taxa = itens.iter().filter(|i| acerto[**i]).count() as f64 / itens.len() as f64;
        rel += peso * (valor - taxa).powi(2);
        res += peso * (taxa - media).powi(2);
    }
    Murphy { rel, res, unc }
}

fn brier(confianca: &[f64], acerto: &[bool]) -> f64 {
    confianca
        .iter()
        .zip(acerto.iter())
        .map(|(p, y)| (p - if *y { 1.0 } else { 0.0 }).powi(2))
        .sum::<f64>()
        / confianca.len() as f64
}

/// Roda os quatro criterios do protocolo **como escritos** (10 caixas, ECE <= 0,10).
///
/// Declarado: este portao foi medido e substituido pelo ajustado (`BALDES_AJUSTADOS`,
/// `ECE_MAXIMO_AJUSTADO`), porque o nulo calibrado o reprova em 99,92% dos casos.
pub fn avaliar(confianca: &[f64], acerto: &[bool]) -> PreTeste {
    avaliar_com(confianca, acerto, BALDES_DO_PROTOCOLO, ECE_MAXIMO_DO_PROTOCOLO)
}

/// Roda os quatro criterios com o desenho de caixas e o limiar declarados pelo chamador.
pub fn avaliar_com(
    confianca: &[f64],
    acerto: &[bool],
    baldes: usize,
    ece_maximo: f64,
) -> PreTeste {
    let n = confianca.len();
    let b = brier(confianca, acerto);
    let media: f64 = acerto.iter().filter(|y| **y).count() as f64 / n.max(1) as f64;
    let brier_clima = media * (1.0 - media);
    let habilidade = if brier_clima == 0.0 { 0.0 } else { 1.0 - b / brier_clima };
    let caixas = caixas_de_massa_igual(confianca, acerto, baldes);
    let ece = ece_de_massa_igual(confianca, acerto, baldes);
    let mut reprovacoes = Vec::new();
    if !(habilidade > 0.0) {
        reprovacoes.push(Reprovacao::HabilidadeNaoPositiva { habilidade });
    }
    if !(ece <= ece_maximo) {
        reprovacoes.push(Reprovacao::EceAcimaDoLimite { ece, limite: ece_maximo });
    }
    for caixa in &caixas {
        if caixa.n >= CAIXA_N_MINIMO && caixa.desvio().abs() > CAIXA_DESVIO_MAXIMO {
            reprovacoes.push(Reprovacao::CaixaSistematica {
                indice: caixa.indice,
                n: caixa.n,
                desvio: caixa.desvio(),
            });
        }
    }
    if !curva_fracamente_monotona(&caixas) {
        let (baixa, alta) = (caixas.first(), caixas.last());
        reprovacoes.push(Reprovacao::CurvaNaoMonotona {
            media_da_caixa_baixa: baixa.map(|c| c.media_prevista).unwrap_or(0.0),
            media_da_caixa_alta: alta.map(|c| c.media_prevista).unwrap_or(0.0),
        });
    }
    PreTeste {
        n,
        brier: b,
        brier_clima,
        habilidade,
        ece,
        caixas,
        murphy: decomposicao_de_murphy(confianca, acerto),
        aprovado: reprovacoes.is_empty(),
        reprovacoes,
    }
}

/// A distribuicao do ECE, da habilidade e do Brier de um especialista perfeitamente calibrado.
#[derive(Debug, Clone, PartialEq)]
pub struct Nulo {
    pub repeticoes: usize,
    pub itens: usize,
    pub baldes: usize,
    pub ece_p50: f64,
    pub ece_p90: f64,
    pub ece_p95: f64,
    pub ece_maximo: f64,
    pub habilidade_p50: f64,
    pub habilidade_p10: f64,
    pub brier_p50: f64,
    pub quantos_passam: f64,
}

fn quantil_de(valores: &mut [f64], q: f64) -> f64 {
    if valores.is_empty() {
        return 0.0;
    }
    valores.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let posicao = q.clamp(0.0, 1.0) * (valores.len() as f64 - 1.0);
    let baixo = posicao.floor() as usize;
    let alto = posicao.ceil() as usize;
    if baixo == alto {
        return valores[baixo];
    }
    let peso = posicao - baixo as f64;
    valores[baixo] * (1.0 - peso) + valores[alto] * peso
}

/// O nulo declarado do pre-teste: `p` sai da priori uniforme declarada e o desfecho e o sorteio de Bernoulli
/// dela. Roda o mesmo desenho (n itens, caixas de massa igual) e devolve a distribuicao das medidas.
pub fn nulo_do_pre_teste(semente: u64, repeticoes: usize, itens: usize, baldes: usize) -> Nulo {
    let mut rng = Rng::novo(semente);
    let mut eces = Vec::with_capacity(repeticoes);
    let mut habilidades = Vec::with_capacity(repeticoes);
    let mut briers = Vec::with_capacity(repeticoes);
    let mut passam = 0usize;
    for _ in 0..repeticoes {
        let confianca: Vec<f64> = (0..itens).map(|_| uniforme(&mut rng)).collect();
        let acerto: Vec<bool> = confianca.iter().map(|p| uniforme(&mut rng) < *p).collect();
        let b = brier(&confianca, &acerto);
        let media: f64 = acerto.iter().filter(|y| **y).count() as f64 / itens as f64;
        let clima = media * (1.0 - media);
        let habilidade = if clima == 0.0 { 0.0 } else { 1.0 - b / clima };
        eces.push(ece_de_massa_igual(&confianca, &acerto, baldes));
        habilidades.push(habilidade);
        briers.push(b);
        if habilidade > 0.0 && ece_de_massa_igual(&confianca, &acerto, baldes) <= ECE_MAXIMO_DO_PROTOCOLO {
            passam += 1;
        }
    }
    Nulo {
        repeticoes,
        itens,
        baldes,
        ece_p50: quantil_de(&mut eces.clone(), 0.50),
        ece_p90: quantil_de(&mut eces.clone(), 0.90),
        ece_p95: quantil_de(&mut eces.clone(), 0.95),
        ece_maximo: eces.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        habilidade_p50: quantil_de(&mut habilidades.clone(), 0.50),
        habilidade_p10: quantil_de(&mut habilidades.clone(), 0.10),
        brier_p50: quantil_de(&mut briers.clone(), 0.50),
        quantos_passam: passam as f64 / repeticoes as f64,
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn a_identidade_de_murphy_fecha_exata() {
        let mut rng = Rng::novo(2026);
        for _ in 0..20 {
            let confianca: Vec<f64> = (0..40).map(|_| uniforme(&mut rng)).collect();
            let acerto: Vec<bool> = confianca.iter().map(|p| uniforme(&mut rng) < *p).collect();
            let m = decomposicao_de_murphy(&confianca, &acerto);
            let b = brier(&confianca, &acerto);
            assert!((m.rel - m.res + m.unc - b).abs() < 1e-12, "identidade: {m:?} contra {b}");
        }
    }

    #[test]
    fn um_pre_teste_calibrado_passa_nos_quatro_criterios() {
        // desenho de conferencia: 10 faixas de 4 itens, com a taxa observada igual a media prevista.
        let mut confianca = Vec::new();
        let mut acerto = Vec::new();
        for faixa in 0..10 {
            let p = 0.05 + faixa as f64 * 0.10;
            let acertos = (p * 4.0).round() as usize;
            for i in 0..4 {
                confianca.push(p);
                acerto.push(i < acertos);
            }
        }
        let r = avaliar(&confianca, &acerto);
        // a granularidade de 4 itens por caixa ja custa cerca de 0,06 de ECE a quem esta calibrado:
        // e um piso do desenho, e nao um erro do especialista.
        assert!(r.ece < 0.08, "ece do desenho perfeito: {}", r.ece);
        assert!(r.habilidade > 0.2, "habilidade do desenho perfeito: {}", r.habilidade);
        assert!(curva_fracamente_monotona(&r.caixas));
        assert!(curva_observada_e_fracamente_monotona(&r.caixas));
        assert!(r.aprovado, "reprovacoes: {:?}", r.reprovacoes);
    }

    #[test]
    fn o_criterio_4_do_protocolo_nao_sabe_reprovar() {
        // Qualquer conjunto de itens, calibrado ou invertido, passa no criterio 4 como ele esta escrito:
        // as caixas sao cortadas por p ordenado, entao a media de cima supera a de baixo sempre.
        let confianca: Vec<f64> = (0..20).map(|i| 0.95 - i as f64 * 0.045).collect();
        let invertido: Vec<bool> = (0..20).map(|i| i % 3 != 0).collect();
        let caixas = caixas_de_massa_igual(&confianca, &invertido, BALDES_DO_PROTOCOLO);
        assert!(curva_fracamente_monotona(&caixas), "o criterio 4 passa mesmo com a previsao invertida");
    }

    #[test]
    fn o_exagerado_reprova_por_ece_e_por_caixa() {
        let confianca: Vec<f64> = (0..40).map(|i| 0.9 + (i % 5) as f64 * 0.02).collect();
        let acerto: Vec<bool> = (0..40).map(|i| i % 4 == 0).collect();
        let r = avaliar(&confianca, &acerto);
        assert!(!r.aprovado);
        assert!(r.reprovacoes.iter().any(|x| matches!(x, Reprovacao::EceAcimaDoLimite { .. })));
        assert!(r.reprovacoes.iter().any(|x| matches!(x, Reprovacao::CaixaSistematica { .. })));
        assert!(r.habilidade < 0.0, "prever alto e errar derruba a habilidade: {}", r.habilidade);
    }

    #[test]
    fn probabilidade_invertida_reprova_por_monotonia() {
        // previsao invertida: o especialista da p alta onde o desfecho e nao, e p baixa onde e sim
        let confianca: Vec<f64> = (0..20).map(|i| 0.025 + i as f64 * 0.05).collect();
        let acerto: Vec<bool> = confianca.iter().map(|p| *p < 0.5).collect();
        let r = avaliar(&confianca, &acerto);
        // o criterio 4 nao morde (e inerte); quem denuncia a curva trocada e o diagnostico da observada.
        assert!(r.reprovacoes.iter().all(|x| !matches!(x, Reprovacao::CurvaNaoMonotona { .. })));
        assert!(!curva_observada_e_fracamente_monotona(&r.caixas));
        assert!(r.caixas.first().expect("tem caixa").taxa_observada > r.caixas.last().expect("tem caixa").taxa_observada);
    }

    #[test]
    fn o_desenho_do_protocolo_poe_dois_itens_por_caixa() {
        let caixas = caixas_de_massa_igual(&(0..20).map(|i| (i as f64 + 0.5) / 20.0).collect::<Vec<f64>>(),
            &vec![true; 20], BALDES_DO_PROTOCOLO);
        assert_eq!(caixas.len(), 10);
        assert!(caixas.iter().all(|c| c.n == 2), "com 20 itens e 10 caixas, n = 2: {caixas:?}");
        assert!(caixas.iter().all(|c| c.n < CAIXA_N_MINIMO), "o criterio 3 fica vacuoso neste desenho");
    }

    #[test]
    fn o_p_e_a_unidade_do_protocolo() {
        assert_eq!(probabilidades_invalidas(&[0.5, 0.0, 1.0, 0.01]), vec![1, 2]);
        assert!(probabilidades_invalidas(&[0.5, 0.25]).is_empty());
    }

    #[test]
    fn o_nulo_do_pre_teste_cresce_quando_a_amostra_encolhe() {
        let vinte = nulo_do_pre_teste(2026, 2_000, 20, BALDES_DO_PROTOCOLO);
        let duzentos = nulo_do_pre_teste(2026, 2_000, 200, BALDES_DO_PROTOCOLO);
        assert!(vinte.ece_p50 > duzentos.ece_p50, "{} contra {}", vinte.ece_p50, duzentos.ece_p50);
        assert!(vinte.ece_p90 >= vinte.ece_p50);
        assert!(vinte.ece_p95 >= vinte.ece_p90);
        assert!(vinte.habilidade_p50 > 0.0, "calibrado tem habilidade positiva: {}", vinte.habilidade_p50);
        assert!(vinte.quantos_passam < 1.0, "o limiar de 0,10 nao deixa todo mundo passar: {}", vinte.quantos_passam);
    }

    #[test]
    fn o_nulo_e_deterministico() {
        let a = nulo_do_pre_teste(7, 500, 20, BALDES_DO_PROTOCOLO);
        let b = nulo_do_pre_teste(7, 500, 20, BALDES_DO_PROTOCOLO);
        assert_eq!(a, b);
    }
}
