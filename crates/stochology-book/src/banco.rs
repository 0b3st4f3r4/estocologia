//! H0 real em Rust: a central de chamadas do banco, sem Python.
//!
//! Le a base de 5 em 5 minutos, monta o perfil medio do treino, escala pelo Erlang C como a
//! pratica do setor e mede o que acontece quando a mesma escala encontra o dia real. Mede tambem
//! a unidade independente (o dia contra a chamada) e o nulo de metades.

use std::collections::BTreeMap;

/// Meias horas por dia util: 07:00 as 21:00.
pub const FATIAS: usize = 28;
/// Tempo medio de atendimento e alvo de espera, em segundos.
pub const AHT_S: f64 = 240.0;
pub const ALVO_S: f64 = 20.0;
pub const META: f64 = 0.80;

#[derive(Debug, Clone)]
pub struct Dia {
    pub data: String,
    pub fatias: [f64; FATIAS],
    pub total: f64,
}

/// Probabilidade de esperar (Erlang C) para carga oferecida a e c atendentes (delega ao nucleo H0).
pub fn erlang_c(c: f64, a: f64) -> f64 {
    if c <= a {
        return 1.0;
    }
    stochology_h0::erlang_c(c as u32, a)
}

/// Nivel de servico: fracao atendida em ate alvo segundos.
pub fn nivel_de_servico(c: f64, a: f64, aht_s: f64, alvo_s: f64) -> f64 {
    if c <= a {
        return 0.0;
    }
    let espera = erlang_c(c, a);
    let dentro = 1.0 - espera * (-((c - a) * alvo_s / aht_s)).exp();
    dentro.clamp(0.0, 1.0)
}

/// Menor numero de atendentes que atinge a meta para uma taxa de chegada (chamadas por hora).
pub fn agentes_para_meta(lam_hora: f64, aht_s: f64, alvo_s: f64, meta: f64) -> u32 {
    let a = lam_hora * aht_s / 3600.0;
    let mut c = a.floor().max(1.0);
    while c < 5_000.0 {
        if nivel_de_servico(c, a, aht_s, alvo_s) >= meta {
            return c as u32;
        }
        c += 1.0;
    }
    c as u32
}

/// Le a base: rownames,DateTime,Calls com carimbos de 5 minutos.
pub fn carregar(caminho: &str) -> Vec<Dia> {
    let texto = std::fs::read_to_string(caminho).expect("nao li o CSV do banco");
    let mut por_dia: BTreeMap<String, [f64; FATIAS]> = BTreeMap::new();
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
        if !(7..=20).contains(&hora) {
            continue;
        }
        let fatia = (hora - 7) * 2 + if minuto >= 30 { 1 } else { 0 };
        let chamadas: f64 = campos[2].parse().unwrap_or(0.0);
        let entrada = por_dia.entry(data.to_string()).or_insert([0.0; FATIAS]);
        entrada[fatia] += chamadas;
    }
    por_dia
        .into_iter()
        .map(|(data, fatias)| Dia {
            data,
            fatias,
            total: fatias.iter().sum(),
        })
        .collect()
}

fn mes(data: &str) -> u32 {
    data[5..7].parse().unwrap_or(0)
}

#[derive(Debug, Clone)]
pub struct MedidaBanco {
    pub dias_treino: usize,
    pub dias_cofre: usize,
    pub chamadas_cofre: f64,
    pub nivel_no_perfil_medio: f64,
    pub nivel_medio_dos_dias: f64,
    pub fracao_dias_abaixo_da_meta: f64,
    pub var_sobre_media: f64,
    pub var_sobre_media_treino: f64,
    pub var_sobre_media_todos: f64,
    pub sd_entre_dias: f64,
    pub poisson_sd: f64,
    pub agentes_no_pico: u32,
    pub agentes_por_dia: f64,
    pub nulo_metades_dia: f64,
    pub nulo_metades_chamada: f64,
}

/// Escala pelo Erlang C sobre a taxa media do treino, fatia a fatia.
fn escala_do_perfil_medio(treino: &[Dia]) -> [u32; FATIAS] {
    let mut media = [0.0_f64; FATIAS];
    for d in treino {
        for i in 0..FATIAS {
            media[i] += d.fatias[i];
        }
    }
    let n = treino.len() as f64;
    let mut escala = [1_u32; FATIAS];
    for i in 0..FATIAS {
        // cada fatia vale meia hora: a media esta em chamadas por meia hora
        escala[i] = agentes_para_meta((media[i] / n) * 2.0, AHT_S, ALVO_S, META);
    }
    escala
}

fn nivel_do_dia(escala: &[u32; FATIAS], dia: &Dia) -> f64 {
    let mut peso = 0.0;
    let mut acc = 0.0;
    for i in 0..FATIAS {
        let lam = dia.fatias[i] * 2.0; // chamadas por hora na fatia
        let a = lam * AHT_S / 3600.0;
        let c = escala[i] as f64;
        let sl = nivel_de_servico(c, a, AHT_S, ALVO_S);
        acc += sl * dia.fatias[i];
        peso += dia.fatias[i];
    }
    if peso == 0.0 { 0.0 } else { acc / peso }
}

/// Mede tudo o que o capitulo cita da base real.
pub fn medir(caminho: &str, semente: u64, replicas: usize) -> MedidaBanco {
    let dias = carregar(caminho);
    let treino: Vec<Dia> = dias.iter().filter(|d| (3..=6).contains(&mes(&d.data))).cloned().collect();
    let cofre: Vec<Dia> = dias.iter().filter(|d| (7..=10).contains(&mes(&d.data))).cloned().collect();
    let escala = escala_do_perfil_medio(&treino);

    let media = {
        let mut m = [0.0_f64; FATIAS];
        for d in &treino {
            for i in 0..FATIAS {
                m[i] += d.fatias[i];
            }
        }
        m.map(|x| x / treino.len() as f64)
    };
    let mut nivel_perfil = 0.0;
    let mut peso = 0.0;
    for i in 0..FATIAS {
        nivel_perfil += nivel_de_servico(escala[i] as f64, media[i] * 2.0 * AHT_S / 3600.0, AHT_S, ALVO_S) * media[i];
        peso += media[i];
    }
    let nivel_perfil = nivel_perfil / peso;

    let niveis: Vec<f64> = cofre.iter().map(|d| nivel_do_dia(&escala, d)).collect();
    let nivel_medio = niveis.iter().sum::<f64>() / niveis.len() as f64;
    let abaixo = niveis.iter().filter(|x| **x < META).count() as f64 / niveis.len() as f64;

    let totais: Vec<f64> = cofre.iter().map(|d| d.total).collect();
    let media_dia = totais.iter().sum::<f64>() / totais.len() as f64;
    let var = totais.iter().map(|t| (t - media_dia) * (t - media_dia)).sum::<f64>() / (totais.len() as f64 - 1.0);

    // nulo de metades: 400 reparticões aleatórias dos dias do cofre
    let mut rng = crate::banco::Rng::novo(semente);
    let mut meias = Vec::with_capacity(replicas);
    let mut niveis_embaralhados = niveis.clone();
    for _ in 0..replicas {
        for i in (1..niveis_embaralhados.len()).rev() {
            let j = rng.uniforme(i as u64 + 1) as usize;
            niveis_embaralhados.swap(i, j);
        }
        let meio = niveis_embaralhados.len() / 2;
        let a: f64 = niveis_embaralhados[..meio].iter().sum::<f64>() / meio as f64;
        let b: f64 = niveis_embaralhados[meio..].iter().sum::<f64>() / (niveis_embaralhados.len() - meio) as f64;
        meias.push((a - b).abs());
    }
    let media_meias = meias.iter().sum::<f64>() / meias.len() as f64;
    let nulo_dia = (meias.iter().map(|x| (x - media_meias) * (x - media_meias)).sum::<f64>()
        / (meias.len() as f64 - 1.0))
        .sqrt();
    let n_chamadas: f64 = totais.iter().sum();
    // nulo de metades na unidade da chamada: duas metades iguais de chamadas independentes
    let nulo_chamada = (2.0 * nivel_medio * (1.0 - nivel_medio) / (n_chamadas / 2.0)).sqrt();

    let dispersao = |ds: &[Dia]| -> f64 {
        let t: Vec<f64> = ds.iter().map(|d| d.total).collect();
        let m = t.iter().sum::<f64>() / t.len() as f64;
        let v = t.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (t.len() as f64 - 1.0);
        v / m
    };
    let var_treino = dispersao(&treino);
    let var_todos = dispersao(&dias);

    MedidaBanco {
        dias_treino: treino.len(),
        dias_cofre: cofre.len(),
        chamadas_cofre: n_chamadas,
        nivel_no_perfil_medio: nivel_perfil,
        nivel_medio_dos_dias: nivel_medio,
        fracao_dias_abaixo_da_meta: abaixo,
        var_sobre_media: var / media_dia,
        var_sobre_media_treino: var_treino,
        var_sobre_media_todos: var_todos,
        sd_entre_dias: var.sqrt(),
        poisson_sd: media_dia.sqrt(),
        agentes_no_pico: escala.iter().copied().max().unwrap_or(0),
        agentes_por_dia: escala.iter().map(|c| *c as f64).sum::<f64>() / 2.0,
        nulo_metades_dia: nulo_dia,
        nulo_metades_chamada: nulo_chamada,
    }
}

/// Gerador determinista re-exportado do núcleo, sem duplicação.
pub use stochology_core::Rng;

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn erlang_c_cresce_com_a_carga() {
        let leve = erlang_c(20.0, 10.0);
        let pesada = erlang_c(20.0, 18.0);
        assert!(leve < pesada, "{leve} {pesada}");
        assert!((0.0..=1.0).contains(&pesada));
    }

    #[test]
    fn agente_a_mais_melhora_o_nivel() {
        let a = 10.0;
        let pior = nivel_de_servico(12.0, a, AHT_S, ALVO_S);
        let melhor = nivel_de_servico(14.0, a, AHT_S, ALVO_S);
        assert!(melhor > pior);
        assert!(melhor > META);
    }

    #[test]
    fn escalar_pela_meta_devolve_meta() {
        let c = agentes_para_meta(300.0, AHT_S, ALVO_S, META);
        let sl = nivel_de_servico(c as f64, 300.0 * AHT_S / 3600.0, AHT_S, ALVO_S);
        assert!(sl >= META, "nivel {sl}");
        let sl_menos_um = nivel_de_servico(c as f64 - 1.0, 300.0 * AHT_S / 3600.0, AHT_S, ALVO_S);
        assert!(sl_menos_um < META, "nivel com um a menos: {sl_menos_um}");
    }
}
