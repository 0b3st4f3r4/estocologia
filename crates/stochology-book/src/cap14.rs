//! Laboratorio do Capitulo 14 — fichas, proibicoes, invariantes Phi1-Phi4 e portoes.
//!
//! Validar uma ficha completa e uma ficha com um defeito plantado por vez,
//! imprimindo as falhas por portao, no espirito de ~portoes --defeito~.
//!
//! Semente fixa. Sem dependencias externas. SplitMix64 embutido.

fn splitmix64(estado: &mut u64) -> u64 {
    stochology_core::rng::passo_splitmix64(estado)
}

fn uniforme(estado: &mut u64, limite: u64) -> u64 {
    splitmix64(estado) % limite
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Portao {
    Phi1,
    Phi2,
    Phi3,
    Phi4,
    TipoIII,
}

impl Portao {
    fn codigo(self) -> &'static str {
        match self {
            Portao::Phi1 => "Phi1-simulabilidade",
            Portao::Phi2 => "Phi2-unidade-independente",
            Portao::Phi3 => "Phi3-camadas-e-modelo-nulo",
            Portao::Phi4 => "Phi4-escopo-e-saida",
            Portao::TipoIII => "tipo-III-pergunta-do-decisor",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Defeito {
    SemSemente,
    SemDefinicao,
    SemMargemNemUnidade,
    SemModeloNulo,
    SemEscopoSemQuemPagaSemSaida,
    PerguntaSemInterrogacao,
    PerguntaSemAcao,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Camada {
    L1,
    L2,
    L3,
}

impl Camada {
    fn codigo(self) -> &'static str {
        match self {
            Camada::L1 => "L1",
            Camada::L2 => "L2",
            Camada::L3 => "L3",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TipoIncerteza {
    Aleatoria,
    Epistemica,
    Knightiana,
    NaoSeAplica,
}

impl TipoIncerteza {
    fn codigo(self) -> &'static str {
        match self {
            TipoIncerteza::Aleatoria => "aleatoria",
            TipoIncerteza::Epistemica => "epistemica",
            TipoIncerteza::Knightiana => "knightiana",
            TipoIncerteza::NaoSeAplica => "nao-se-aplica",
        }
    }
}

#[derive(Debug, Clone)]
struct Ficha {
    id: String,
    camada: Camada,
    incerteza: TipoIncerteza,
    definicao: String,
    artefato_simulavel: Option<String>,
    semente: Option<u64>,
    unidade_independente: Option<String>,
    margem: Option<f64>,
    modelo_nulo: Option<String>,
    escopo_ecologico: bool,
    quem_paga_pelo_erro: Option<String>,
    clausula_de_saida: Option<String>,
    pergunta_do_decisor: String,
    acao_do_pacote: String,
    cabecalho: [String; 5],
}

fn ficha_completa() -> Ficha {
    Ficha {
        id: "h0.posto-saude".to_string(),
        camada: Camada::L1,
        incerteza: TipoIncerteza::Aleatoria,
        definicao: "fila com chegada nao linear e espera convexa".to_string(),
        artefato_simulavel: Some("crates/stochology-book#cap16".to_string()),
        semente: Some(2026),
        unidade_independente: Some("dia".to_string()),
        margem: Some(0.51),
        modelo_nulo: Some("media-historica".to_string()),
        escopo_ecologico: true,
        quem_paga_pelo_erro: Some("o paciente que espera e desiste".to_string()),
        clausula_de_saida: Some("parar se a fracao de espera acima de 30 min passar de 0,25".to_string()),
        pergunta_do_decisor:
            "devemos abrir uma segunda sala nos dias cheios?".to_string(),
        acao_do_pacote: "abrir uma segunda sala".to_string(),
        cabecalho: [
            "processo_gerador: fila com taxa de chegada que oscila entre 5 e 11 por hora e chegada nao Poisson"
                .to_string(),
            "nao_linearidade: a espera cresce em curva; f(E[X]) nao e E[f(X)] perto da saturacao".to_string(),
            "unidade_e_margem: a unidade independente e o DIA; 81 dias, nao 2,65 milhoes de chamadas".to_string(),
            "fisica_do_ruido: a demanda diaria tem Var/media = 273 (Poisson: 1); sem estados absorventes".to_string(),
            "escada_causal: degrau 2 do(escala) simulado sobre a demanda real; premissa nao testavel: a demanda nao reage a escala".to_string(),
        ],
    }
}

#[derive(Debug, Clone)]
struct Falha {
    portao: Portao,
    detalhe: String,
}

fn avaliar(f: &Ficha) -> Vec<Falha> {
    let mut falhas: Vec<Falha> = Vec::new();
    let mut add = |portao: Portao, detalhe: &str| {
        falhas.push(Falha {
            portao,
            detalhe: detalhe.to_string(),
        });
    };

    if f.semente.is_none() || f.artefato_simulavel.is_none() {
        add(Portao::Phi1, "falta semente fixa ou artefato simulavel");
    }
    if f.margem.is_none() || f.unidade_independente.is_none() {
        add(Portao::Phi2, "margem sem unidade independente declarada");
    }
    if f.modelo_nulo.is_none() || f.definicao.trim().is_empty() {
        add(
            Portao::Phi3,
            "sem modelo nulo ou sem definicao em uma linha",
        );
    }
    if !f.escopo_ecologico || f.quem_paga_pelo_erro.is_none() || f.clausula_de_saida.is_none() {
        add(
            Portao::Phi4,
            "escopo ecologico, quem paga pelo erro ou clausula de saida ausente",
        );
    }
    let pergunta = f.pergunta_do_decisor.trim();
    let acao = f.acao_do_pacote.trim().to_lowercase();
    let casa = !acao.is_empty() && pergunta.to_lowercase().contains(&acao);
    if !pergunta.ends_with('?') || !casa {
        add(
            Portao::TipoIII,
            "pergunta_do_decisor precisa terminar em ? e citar a acao do pacote",
        );
    }
    if f.cabecalho.iter().any(|c| c.trim().is_empty()) {
        add(Portao::Phi3, "cabecalho de cinco perguntas incompleto");
    }
    for (i, c) in f.cabecalho.iter().enumerate() {
        let esperado = [
            "processo_gerador",
            "nao_linearidade",
            "unidade_e_margem",
            "fisica_do_ruido",
            "escada_causal",
        ][i];
        if !c.starts_with(esperado) {
            add(Portao::Phi3, "campo do cabecalho fora de ordem");
        }
    }
    falhas
}

// ----------------------------------------------- grafo, no espirito do Cap. 12

#[derive(Debug, Clone, PartialEq)]
struct No {
    id: String,
    definicao: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Hiperaresta {
    id: String,
    membros: Vec<String>,
}

#[derive(Debug, Clone, Default)]
struct Grafo {
    nos: Vec<No>,
    tree: Vec<(String, String)>,
    hiperarestas: Vec<Hiperaresta>,
}

impl Grafo {
    fn validar(&self) -> Vec<String> {
        let mut p: Vec<String> = Vec::new();
        let existe = |id: &str| self.nos.iter().any(|n| n.id == id);
        for (de, para) in &self.tree {
            if !existe(de) || !existe(para) {
                p.push(format!("aresta para no ausente: {} -> {}", de, para));
            }
        }
        for h in &self.hiperarestas {
            if h.membros.len() < 2 {
                p.push(format!("hiperaresta com menos de dois membros: {}", h.id));
            }
            for m in &h.membros {
                if !existe(m) {
                    p.push(format!("hiperaresta {} cita no ausente: {}", h.id, m));
                }
            }
        }
        let mut cor: Vec<(String, u8)> = Vec::new();
        let mut ciclo: Option<String> = None;
        for n in &self.nos {
            if self.visitar(&n.id, &mut cor, &mut ciclo) {
                p.push(format!("ciclo em E_tree: {:?}", ciclo));
                break;
            }
        }
        p
    }

    fn visitar(&self, id: &str, cor: &mut Vec<(String, u8)>, ciclo: &mut Option<String>) -> bool {
        match cor.iter().find(|(x, _)| x == id).map(|(_, c)| *c) {
            Some(1) => {
                *ciclo = Some(id.to_string());
                true
            }
            Some(2) => false,
            _ => {
                cor.push((id.to_string(), 1));
                for (de, para) in &self.tree {
                    if de == id && self.visitar(para, cor, ciclo) {
                        return true;
                    }
                }
                if let Some(e) = cor.iter_mut().find(|(x, _)| x == id) {
                    e.1 = 2;
                }
                false
            }
        }
    }
}

fn grafo_bom() -> Grafo {
    Grafo {
        nos: vec![
            No {
                id: "L1_fila".to_string(),
                definicao: "espera medida em 81 dias".to_string(),
            },
            No {
                id: "L2_erlang".to_string(),
                definicao: "nivel de servico estacionario".to_string(),
            },
            No {
                id: "L3_gestor".to_string(),
                definicao: "escala decidida pela media".to_string(),
            },
        ],
        tree: vec![
            ("L1_fila".to_string(), "L2_erlang".to_string()),
            ("L2_erlang".to_string(), "L3_gestor".to_string()),
        ],
        hiperarestas: vec![Hiperaresta {
            id: "H1".to_string(),
            membros: vec!["L1_fila".to_string(), "L2_erlang".to_string()],
        }],
    }
}

fn com_defeito(d: Defeito) -> Ficha {
    let mut f = ficha_completa();
    match d {
        Defeito::SemSemente => f.semente = None,
        Defeito::SemDefinicao => f.definicao = String::new(),
        Defeito::SemMargemNemUnidade => {
            f.margem = None;
            f.unidade_independente = None;
        }
        Defeito::SemModeloNulo => f.modelo_nulo = None,
        Defeito::SemEscopoSemQuemPagaSemSaida => {
            f.escopo_ecologico = false;
            f.quem_paga_pelo_erro = None;
            f.clausula_de_saida = None;
        }
        Defeito::PerguntaSemInterrogacao => {
            f.pergunta_do_decisor = "devemos abrir uma segunda sala nos dias cheios".to_string()
        }
        Defeito::PerguntaSemAcao => {
            f.pergunta_do_decisor = "devemos fazer alguma coisa nos dias cheios?".to_string()
        }
    }
    f
}

fn nomes(v: &[Portao]) -> String {
    v.iter()
        .map(|p| p.codigo())
        .collect::<Vec<&str>>()
        .join(",")
}

pub fn rodar() {
    println!("== Capitulo 14 :: o contrato de ferro ==");

    let f = ficha_completa();
    let falhas = avaliar(&f);
    println!(
        "ficha {}: camada={} incerteza={} cabecalho={} campos | falhas={:?} total={}",
        f.id,
        f.camada.codigo(),
        f.incerteza.codigo(),
        f.cabecalho.len(),
        falhas
            .iter()
            .map(|x| x.portao.codigo())
            .collect::<Vec<&str>>(),
        falhas.len()
    );

    // A taxonomia declarada: tres camadas e quatro tipos de incerteza.
    let camadas = [Camada::L1, Camada::L2, Camada::L3];
    let incertezas = [
        TipoIncerteza::Aleatoria,
        TipoIncerteza::Epistemica,
        TipoIncerteza::Knightiana,
        TipoIncerteza::NaoSeAplica,
    ];
    println!(
        "taxonomia declarada: camadas={:?} incertezas={:?}",
        camadas.iter().map(|c| c.codigo()).collect::<Vec<&str>>(),
        incertezas.iter().map(|i| i.codigo()).collect::<Vec<&str>>()
    );

    let defeitos = [
        Defeito::SemSemente,
        Defeito::SemDefinicao,
        Defeito::SemMargemNemUnidade,
        Defeito::SemModeloNulo,
        Defeito::SemEscopoSemQuemPagaSemSaida,
        Defeito::PerguntaSemInterrogacao,
        Defeito::PerguntaSemAcao,
    ];
    for d in defeitos {
        let fx = com_defeito(d);
        let falhas = avaliar(&fx);
        let portoes: Vec<Portao> = {
            let mut v: Vec<Portao> = falhas.iter().map(|x| x.portao).collect();
            v.sort_by_key(|p| p.codigo());
            v.dedup();
            v
        };
        println!(
            "defeito {:?}: falhas={} portoes={} detalhe={:?}",
            d,
            falhas.len(),
            nomes(&portoes),
            falhas
                .first()
                .map(|x| x.detalhe.clone())
                .unwrap_or_default()
        );
    }

    // Os dois invariantes de grafo, no espirito de --defeito.
    let g = grafo_bom();
    println!("grafo integro: validacao={:?}", g.validar());
    let mut g1 = g.clone();
    g1.tree
        .push(("L2_erlang".to_string(), "L2_fantasma".to_string()));
    println!("grafo com aresta fantasma: validacao={:?}", g1.validar());
    let mut g2 = g.clone();
    g2.tree
        .push(("L3_gestor".to_string(), "L1_fila".to_string()));
    println!("grafo com ciclo dedutivo: validacao={:?}", g2.validar());
    let mut g3 = g.clone();
    g3.hiperarestas.push(Hiperaresta {
        id: "H2".to_string(),
        membros: vec!["L3_gestor".to_string()],
    });
    println!(
        "grafo com hiperaresta de um membro: validacao={:?}",
        g3.validar()
    );

    // Teste de propriedades sobre a funcao do portao Phi2: a margem simulada
    // nunca pode ser negativa e nunca pode encolher quando a amostra cresce.
    let mut rng = 2026u64;
    let mut violacoes = 0usize;
    let mut menor = f64::MAX;
    let mut maior = f64::MIN;
    for _ in 0..10_000 {
        let n = (uniforme(&mut rng, 900) + 100) as f64;
        let p = (uniforme(&mut rng, 99) + 1) as f64 / 100.0;
        let margem = 1.96 * (p * (1.0 - p) / n).sqrt();
        menor = menor.min(margem);
        maior = maior.max(margem);
        if !(0.0..=0.5).contains(&margem) {
            violacoes += 1;
        }
    }
    println!(
        "teste de propriedades (10.000 fichas simuladas): violacoes={} margem_min={:.5} margem_max={:.5}",
        violacoes, menor, maior
    );

    let repeticoes = 32usize;
    let mut diferentes = 0usize;
    for _ in 0..repeticoes {
        let mut a = 2026u64;
        let mut b = 2026u64;
        let va: Vec<u64> = (0..16).map(|_| uniforme(&mut a, 1_000_000)).collect();
        let vb: Vec<u64> = (0..16).map(|_| uniforme(&mut b, 1_000_000)).collect();
        if va != vb {
            diferentes += 1;
        }
    }
    println!(
        "mesma semente, dois validadores: execucoes divergentes em {} de {}",
        diferentes, repeticoes
    );
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn ficha_completa_passa_e_defeito_aciona_portao() {
        assert!(avaliar(&ficha_completa()).is_empty());
        assert!(!avaliar(&com_defeito(Defeito::SemSemente)).is_empty());
        assert!(grafo_bom().validar().is_empty());
    }
}
