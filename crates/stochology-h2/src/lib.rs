//! stochology-h2 — as sete rodadas de invariância causal e a multiplicidade.
//!
//! Uma rodada e um alvo, um conjunto de candidatos e os ambientes: devolve quantos subconjuntos
//! existem (2^n), quantos sao aceitos e qual area sobrevive. Em cima disso rodam o contador de
//! tentativas K e a correcao de Bonferroni que o H2g exige.
//!
//! Declarado e nao implementado: as sete rodadas reais (tunel de luz, farmacologia, genetica de
//! levedura, STAR) seguem como registro — o metodo e o que esta aqui; a base externa nao.

pub mod genetica;

use stochology_icp::{Ambiente, Erro, area, contar_conjuntos, familia_aceita, normal};

/// Uma rodada: um alvo, os candidatos e o que sobrou de area.
#[derive(Debug, Clone, PartialEq)]
pub struct Rodada {
    pub nome: String,
    pub alvo: usize,
    pub candidatos: Vec<usize>,
    pub conjuntos: u64,
    pub aceitos: usize,
    pub area: Vec<usize>,
}

pub fn rodada(
    nome: &str,
    ambientes: &[Ambiente],
    alvo: usize,
    candidatos: &[usize],
    alfa: f64,
) -> Result<Rodada, Erro> {
    let familia = familia_aceita(ambientes, alvo, candidatos, alfa)?;
    Ok(Rodada {
        nome: nome.to_string(),
        alvo,
        candidatos: candidatos.to_vec(),
        conjuntos: contar_conjuntos(candidatos.len()),
        aceitos: familia.len(),
        area: area(&familia),
    })
}

/// Multiplicidade declarada: quantas areas sobrevivem antes e depois de Bonferroni.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Multiplicidade {
    pub k: usize,
    pub alfa: f64,
    pub alfa_corrigido: f64,
    pub areas_antes: usize,
    pub areas_depois: usize,
}

pub fn multiplicidade(p_valores: &[f64], alfa: f64) -> Multiplicidade {
    let k = p_valores.len();
    let alfa_corrigido = if k == 0 { alfa } else { alfa / k as f64 };
    Multiplicidade {
        k,
        alfa,
        alfa_corrigido,
        areas_antes: p_valores.iter().filter(|p| **p <= alfa).count(),
        areas_depois: p_valores.iter().filter(|p| **p <= alfa_corrigido).count(),
    }
}

/// Contador de tentativas: quantas especificacoes distintas foram rodadas, sem contar repetidas.
pub fn contador_k(tentativas: &[String]) -> usize {
    let mut vistas: Vec<String> = Vec::new();
    for tentativa in tentativas {
        let limpa = tentativa
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        if !vistas.contains(&limpa) {
            vistas.push(limpa);
        }
    }
    vistas.len()
}

/// Nulo simulado da rodada: quantas vezes o conjunto e aceito quando os rotulos de ambiente sao
/// embaralhados. Serve para separar invariância real de artefato do teste.
pub fn nulo_por_embaralhamento(
    ambientes: &[Ambiente],
    alvo: usize,
    preditores: &[usize],
    alfa: f64,
    semente: u64,
    repeticoes: usize,
) -> Result<f64, Erro> {
    let mut rng = stochology_core::Rng::novo(semente);
    let linhas = ambientes
        .first()
        .map(Ambiente::linhas)
        .ok_or(Erro::SemLinhas)?;
    let colunas: Vec<Vec<f64>> = (0..ambientes[0].colunas.len())
        .map(|j| {
            ambientes
                .iter()
                .flat_map(|a| a.coluna(j).cloned().unwrap_or_default())
                .collect()
        })
        .collect();
    let mut aceitos = 0usize;
    for _ in 0..repeticoes.max(1) {
        // embaralha a ordem das linhas em todas as colunas, com a mesma permutacao
        let ordem = permutacao(&mut rng, colunas[0].len());
        let mut novos: Vec<Ambiente> = Vec::new();
        for (indice, ambiente) in ambientes.iter().enumerate() {
            let mut blocos: Vec<Vec<f64>> = vec![Vec::new(); colunas.len()];
            for i in 0..linhas {
                let origem = ordem[indice * linhas + i];
                for (j, coluna) in colunas.iter().enumerate() {
                    blocos[j].push(coluna[origem]);
                }
            }
            novos.push(Ambiente::novo(&ambiente.nome, blocos)?);
        }
        if stochology_icp::conjunto_aceito(&novos, alvo, preditores, alfa)? {
            aceitos += 1;
        }
    }
    Ok(aceitos as f64 / repeticoes.max(1) as f64)
}

fn permutacao(rng: &mut stochology_core::Rng, n: usize) -> Vec<usize> {
    let mut ordem: Vec<usize> = (0..n).collect();
    for i in (1..n).rev() {
        let j = rng.uniforme(i as u64 + 1) as usize;
        ordem.swap(i, j);
    }
    ordem
}

/// Mundo sintetico das rodadas: a marginal de X1 muda de ambiente para ambiente (escala 1 e 3) e
/// X2 carrega o rotulo. O alvo depende so de X1, com o mesmo ruido nos dois ambientes — e por isso
/// que {X1} e invariante e o conjunto vazio nao e.
pub fn mundo_sintetico(semente: u64, por_ambiente: usize) -> Vec<Ambiente> {
    let mut rng = stochology_core::Rng::novo(semente);
    let mut ambientes = Vec::new();
    for (indice, nome) in ["A", "B"].iter().enumerate() {
        let e = indice as f64;
        let escala = if indice == 0 { 1.0 } else { 3.0 };
        let mut x1 = Vec::new();
        let mut x2 = Vec::new();
        let mut y = Vec::new();
        for _ in 0..por_ambiente {
            let a = escala * normal(&mut rng);
            x1.push(a);
            x2.push(0.9 * e + normal(&mut rng));
            y.push(2.0 * a + 0.6 * normal(&mut rng));
        }
        ambientes.push(Ambiente::novo(nome, vec![y, x1, x2]).expect("ambiente sintetico"));
    }
    ambientes
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn a_rodada_guarda_a_area_e_a_contagem_de_conjuntos() {
        let ambientes = mundo_sintetico(2026, 140);
        let r = rodada("tunel-de-luz", &ambientes, 0, &[1, 2], 0.05).expect("rodada");
        assert_eq!(r.conjuntos, 4);
        assert_eq!(
            r.area,
            vec![1],
            "a area que sobrevive e o invariante: {r:?}"
        );
        assert!(
            r.aceitos < r.conjuntos as usize,
            "nem todo conjunto passa: {r:?}"
        );
    }

    #[test]
    fn dez_candidatos_dao_mil_e_vinte_e_quatro_conjuntos() {
        assert_eq!(contar_conjuntos(10), 1024);
        let ambientes = mundo_sintetico(3, 40);
        let r = rodada("contagem", &ambientes, 0, &[1, 2], 0.05).expect("rodada");
        assert_eq!(r.conjuntos, 4);
    }

    #[test]
    fn o_grao_da_unidade_muda_a_familia_aceita() {
        let fina = mundo_sintetico(11, 240);
        let grossa: Vec<Ambiente> = fina
            .iter()
            .map(|a| a.media_por_grupo(20).expect("media por grupo"))
            .collect();
        let r_fina = rodada("fina", &fina, 0, &[1, 2], 0.05).expect("rodada");
        let r_grossa = rodada("grossa", &grossa, 0, &[1, 2], 0.05).expect("rodada");
        // O poder do teste depende do grao: com 12 pontos por ambiente o limiar de KS e frouxo, o
        // conjunto vazio e o confundido passam, e a area se dissolve. Quem conta o n em linha
        // pseudorreplicada fica com um teste que promete mais do que pode entregar (Phi2).
        assert_eq!(grossa[0].linhas(), 12);
        assert!(
            stochology_icp::limiar_ks(12, 12, 0.05) > stochology_icp::limiar_ks(240, 240, 0.05)
        );
        assert!(
            r_grossa.aceitos >= r_fina.aceitos,
            "grao grosso aceita mais: {} contra {}",
            r_grossa.aceitos,
            r_fina.aceitos
        );
        assert_eq!(
            r_fina.area,
            vec![1],
            "no grao fino o invariante sobrevive: {r_fina:?}"
        );
        assert_eq!(
            r_grossa.aceitos, 4,
            "no grao grosso tudo passa, inclusive o conjunto vazio: {r_grossa:?}"
        );
        assert!(
            r_grossa.area.is_empty(),
            "e a area se dissolve: {r_grossa:?}"
        );
    }

    #[test]
    fn bonferroni_derruba_uma_area_e_conta_o_k() {
        let mut p = vec![0.0001, 0.0005, 0.001, 0.03];
        p.extend((0..34).map(|i| 0.2 + 0.02 * i as f64));
        let m = multiplicidade(&p, 0.05);
        assert_eq!(m.k, 38);
        assert_eq!(m.areas_antes, 4);
        assert_eq!(m.areas_depois, 3);
        assert!((m.alfa_corrigido - 0.05 / 38.0).abs() < 1e-15);
    }

    #[test]
    fn o_contador_k_junta_especificacoes_repetidas() {
        let mut tentativas: Vec<String> = (0..38).map(|i| format!("Rodada {}", i)).collect();
        tentativas.push("rodada 0".into());
        tentativas.push(" RODADA   1 ".into());
        assert_eq!(tentativas.len(), 40);
        assert_eq!(contador_k(&tentativas), 38);
    }

    #[test]
    fn o_nulo_embaralhado_mostra_o_poder_do_teste() {
        let ambientes = mundo_sintetico(5, 150);
        let confundido =
            nulo_por_embaralhamento(&ambientes, 0, &[2], 0.05, 2026, 20).expect("nulo");
        let real = stochology_icp::conjunto_aceito(&ambientes, 0, &[2], 0.05).expect("testa");
        assert!(!real, "o confundido reprova com o rotulo real");
        assert!(
            confundido > 0.5,
            "sem rotulo, o mesmo conjunto passa: {confundido}"
        );
    }

    #[test]
    fn sem_area_a_rodada_devolve_area_vazia() {
        // A dispersao do alvo muda de ambiente para ambiente: nem o vazio, nem {X1} sobrevivem.
        let mut rng = stochology_core::Rng::novo(9);
        let mut ambientes = Vec::new();
        for (indice, nome) in ["A", "B"].iter().enumerate() {
            let ruido = if indice == 0 { 1.0 } else { 4.0 };
            let mut x1 = Vec::new();
            let mut y = Vec::new();
            for _ in 0..80 {
                let a = normal(&mut rng);
                x1.push(a);
                y.push(2.0 * a + ruido * normal(&mut rng));
            }
            ambientes.push(Ambiente::novo(nome, vec![y, x1]).expect("ambiente"));
        }
        let r = rodada("disperso", &ambientes, 0, &[1], 0.05).expect("rodada");
        assert_eq!(r.aceitos, 0, "nada e invariante: {r:?}");
        assert!(r.area.is_empty());
    }
}
