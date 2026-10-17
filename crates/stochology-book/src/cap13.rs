//! Laboratorio do Capitulo 13 — a banca adversarial A1-A8.
//!
//! A condicao de aprovacao e o E logico das oito severidades contra o piso 6:
//!   Aprovado(X) = AND_{i=1..8} ( A_i(X) >= Severidade_min ).
//! Semente fixa. Sem dependencias externas. SplitMix64 embutido.

fn splitmix64(estado: &mut u64) -> u64 {
    stochology_core::rng::passo_splitmix64(estado)
}

fn uniforme(estado: &mut u64, limite: u64) -> u64 {
    splitmix64(estado) % limite
}

/// Os oito cuidados que a banca cobra, um por faixa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cuidado {
    TesteDePropriedade,
    TrataAusentes,
    DerivaDeclarada,
    PatologiasCobertas,
    TrilhaIntegra,
    SubgruposAuditados,
    PrioriElicitada,
    ClausulaDeSaida,
}

const TODOS: [Cuidado; 8] = [
    Cuidado::TesteDePropriedade,
    Cuidado::TrataAusentes,
    Cuidado::DerivaDeclarada,
    Cuidado::PatologiasCobertas,
    Cuidado::TrilhaIntegra,
    Cuidado::SubgruposAuditados,
    Cuidado::PrioriElicitada,
    Cuidado::ClausulaDeSaida,
];

impl Cuidado {
    fn nome(self) -> &'static str {
        match self {
            Cuidado::TesteDePropriedade => "A1-codigo",
            Cuidado::TrataAusentes => "A2-dados",
            Cuidado::DerivaDeclarada => "A3-ciclo-de-vida",
            Cuidado::PatologiasCobertas => "A4-inferencia",
            Cuidado::TrilhaIntegra => "A5-governanca",
            Cuidado::SubgruposAuditados => "A6-justica",
            Cuidado::PrioriElicitada => "A7-dominio",
            Cuidado::ClausulaDeSaida => "A8-decisao",
        }
    }

    /// Ordinal que entra na mistura da semente, como no modulo do repositorio.
    fn ordinal(self) -> u64 {
        match self {
            Cuidado::TesteDePropriedade => 1,
            Cuidado::TrataAusentes => 2,
            Cuidado::DerivaDeclarada => 3,
            Cuidado::PatologiasCobertas => 4,
            Cuidado::TrilhaIntegra => 5,
            Cuidado::SubgruposAuditados => 6,
            Cuidado::PrioriElicitada => 7,
            Cuidado::ClausulaDeSaida => 8,
        }
    }

    fn semente(self, base: u64) -> u64 {
        base ^ self.ordinal().wrapping_mul(0x9E37_79B9_7F4A_7C15)
    }
}

/// Escala 0 a 10. Piso de aprovacao: 6.
const PISO: u8 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Cuidados {
    teste_de_propriedade: bool,
    trata_ausentes: bool,
    deriva_declarada: bool,
    patologias_cobertas: bool,
    trilha_integra: bool,
    subgrupos_auditados: bool,
    priori_elicitada: bool,
    clausula_de_saida: bool,
}

impl Cuidados {
    fn todos() -> Self {
        Cuidados {
            teste_de_propriedade: true,
            trata_ausentes: true,
            deriva_declarada: true,
            patologias_cobertas: true,
            trilha_integra: true,
            subgrupos_auditados: true,
            priori_elicitada: true,
            clausula_de_saida: true,
        }
    }

    fn atende(&self, c: Cuidado) -> bool {
        match c {
            Cuidado::TesteDePropriedade => self.teste_de_propriedade,
            Cuidado::TrataAusentes => self.trata_ausentes,
            Cuidado::DerivaDeclarada => self.deriva_declarada,
            Cuidado::PatologiasCobertas => self.patologias_cobertas,
            Cuidado::TrilhaIntegra => self.trilha_integra,
            Cuidado::SubgruposAuditados => self.subgrupos_auditados,
            Cuidado::PrioriElicitada => self.priori_elicitada,
            Cuidado::ClausulaDeSaida => self.clausula_de_saida,
        }
    }

    fn retirar(&mut self, c: Cuidado) {
        match c {
            Cuidado::TesteDePropriedade => self.teste_de_propriedade = false,
            Cuidado::TrataAusentes => self.trata_ausentes = false,
            Cuidado::DerivaDeclarada => self.deriva_declarada = false,
            Cuidado::PatologiasCobertas => self.patologias_cobertas = false,
            Cuidado::TrilhaIntegra => self.trilha_integra = false,
            Cuidado::SubgruposAuditados => self.subgrupos_auditados = false,
            Cuidado::PrioriElicitada => self.priori_elicitada = false,
            Cuidado::ClausulaDeSaida => self.clausula_de_saida = false,
        }
    }

    fn retirados(&self) -> Vec<Cuidado> {
        TODOS.iter().copied().filter(|c| !self.atende(*c)).collect()
    }
}

#[derive(Debug, Clone)]
struct Ficha {
    id: String,
    cuidados: Cuidados,
}

/// Severidade deterministica do ataque sobre a ficha, para a semente dada.
fn estressar(agente: Cuidado, ficha: &Ficha, semente: u64) -> u8 {
    let mut rng = agente.semente(semente);
    if ficha.cuidados.atende(agente) {
        (8 + uniforme(&mut rng, 3)) as u8
    } else {
        (2 + uniforme(&mut rng, 3)) as u8
    }
}

#[derive(Debug, Clone)]
struct Aprovacao {
    itens: Vec<(Cuidado, u8)>,
    aprovado: bool,
}

fn banca(ficha: &Ficha, semente: u64) -> Aprovacao {
    let itens: Vec<(Cuidado, u8)> = TODOS
        .iter()
        .map(|c| (*c, estressar(*c, ficha, semente)))
        .collect();
    let aprovado = itens.iter().all(|(_, s)| *s >= PISO);
    Aprovacao { itens, aprovado }
}

impl Aprovacao {
    fn menor(&self) -> u8 {
        self.itens.iter().map(|(_, s)| *s).min().unwrap_or(0)
    }

    fn linha(&self) -> String {
        self.itens
            .iter()
            .map(|(c, s)| format!("{}={}", c.nome(), s))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

/// Sistema 1: triagem categorica fechada com probabilidade restrita a [0,01; 0,99].
#[derive(Debug, Clone)]
struct AvaliacaoS1 {
    rotulo: &'static str,
    probabilidades: Vec<f64>,
}

fn s1(ficheiro: &str) -> AvaliacaoS1 {
    let mut rng = 0u64;
    for b in ficheiro.as_bytes() {
        rng = rng.wrapping_mul(1_099_511_628_211).wrapping_add(*b as u64);
    }
    let bruto = (uniforme(&mut rng, 99) as f64 + 1.0) / 100.0;
    let restrita = bruto.clamp(0.01, 0.99);
    AvaliacaoS1 {
        rotulo: "ficha-em-ordem",
        probabilidades: vec![restrita, 1.0 - restrita],
    }
}

pub fn rodar() {
    println!("== Capitulo 13 :: a banca adversarial A1-A8 ==");

    let completa = Ficha {
        id: "h3.voos".to_string(),
        cuidados: Cuidados::todos(),
    };
    let com_defeito = Ficha {
        id: "h3.voos-com-defeito".to_string(),
        cuidados: Cuidados {
            trilha_integra: false,
            subgrupos_auditados: false,
            ..Cuidados::todos()
        },
    };

    for (rotulo, ficha) in [
        ("ficha completa", &completa),
        ("ficha com A5 e A6 removidos", &com_defeito),
    ] {
        let r = banca(ficha, 2026);
        println!(
            "{} (ficha {}): {} | menor={} piso={} aprovado={}",
            rotulo,
            ficha.id,
            r.linha(),
            r.menor(),
            PISO,
            r.aprovado
        );
    }

    // Um cuidado retirado de cada vez: quantos agentes reprovam e quem reprova.
    let mut reprovacoes_por_agente = [0usize; 8];
    let mut menor_global = 10u8;
    let mut fichas_reprovadas = 0usize;
    for c in TODOS {
        let mut cuidados = Cuidados::todos();
        cuidados.retirar(c);
        let f = Ficha {
            id: format!("ficha-sem-{}", c.nome()),
            cuidados,
        };
        let r = banca(&f, 2026);
        let reprova: Vec<&'static str> = r
            .itens
            .iter()
            .filter(|(_, s)| *s < PISO)
            .map(|(a, _)| a.nome())
            .collect();
        if !r.aprovado {
            fichas_reprovadas += 1;
        }
        if r.menor() < menor_global {
            menor_global = r.menor();
        }
        for (i, (_, s)) in r.itens.iter().enumerate() {
            if *s < PISO {
                reprovacoes_por_agente[i] += 1;
            }
        }
        let faltando = f.cuidados.retirados();
        println!(
            "sem {}: reprovam={:?} qtd_reprovando={} menor={}",
            faltando[0].nome(),
            reprova,
            reprova.len(),
            r.menor()
        );
    }
    println!(
        "varredura: fichas reprovadas={} de 8 | menor severidade global={} | severidades < piso por agente={:?}",
        fichas_reprovadas, menor_global, reprovacoes_por_agente
    );

    // Reprogramar a banca com outra semente nao muda o veredito.
    for semente in [7u64, 2026, 999_983] {
        let r = banca(&completa, semente);
        println!(
            "ficha completa com semente {}: menor={} aprovado={}",
            semente,
            r.menor(),
            r.aprovado
        );
    }

    // Sistema 1: probabilidades restritas ao intervalo epistemologico.
    let s = s1("h3.voos");
    let fora = s
        .probabilidades
        .iter()
        .filter(|p| **p < 0.01 || **p > 0.99)
        .count();
    println!(
        "sistema 1: rotulo={} produtos={:?} soma={:.4} fora_de[0.01;0.99]={}",
        s.rotulo,
        s.probabilidades
            .iter()
            .map(|p| (p * 10_000.0).round() / 10_000.0)
            .collect::<Vec<f64>>(),
        s.probabilidades.iter().sum::<f64>(),
        fora
    );
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn banca_aprova_ficha_completa_e_reprova_quando_falta_cuidado() {
        let mut f = Ficha {
            id: "h3.teste".into(),
            cuidados: Cuidados::todos(),
        };
        assert!(banca(&f, 2026).aprovado);
        f.cuidados.retirar(Cuidado::TrilhaIntegra);
        assert!(!banca(&f, 2026).aprovado);
    }
}

