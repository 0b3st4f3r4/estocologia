//! CLI dos portões determinísticos.
//!
//! Uso:
//!   cargo run --release --bin portoes -- --horizonte todos --reproducao
//!   cargo run --release --bin portoes -- --horizonte h3 --defeito h3.voos23

use stochology_audit::{LivroRazao, sha256_hex};
use stochology_core::{
    AgenteA, Camada, Ficha, OrquestradorM, Rng, SEVERIDADE_MINIMA, TipoIncerteza, avaliar,
};
use stochology_drift::{Opcoes as OpcoesDeriva, cusum, nulo_estacionario};
use stochology_fairness::{Grupo as GrupoJustica, paridade_demografica};
use stochology_h1::hill;
use stochology_hypergraph::lentes;

struct Config {
    horizonte: String,
    reproducao: bool,
    defeito: Option<String>,
}

fn config() -> Config {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut cfg = Config {
        horizonte: "todos".to_string(),
        reproducao: false,
        defeito: None,
    };
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--horizonte" => {
                if i + 1 < args.len() {
                    cfg.horizonte = args[i + 1].clone();
                    i += 1;
                }
            }
            "--reproducao" => cfg.reproducao = true,
            "--defeito" => {
                if i + 1 < args.len() {
                    cfg.defeito = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            outra => {
                eprintln!("flag desconhecida: {outra}");
                std::process::exit(1);
            }
        }
        i += 1;
    }
    cfg
}

fn ficha(
    id: &str,
    camada: Camada,
    incerteza: TipoIncerteza,
    definicao: &str,
    pergunta: &str,
    acao: &str,
) -> Ficha {
    let mut f = Ficha::nova(id, camada, incerteza);
    f.definicao = definicao.to_string();
    f.artefato_simulavel = Some(format!("crates/stochology-core#{id}"));
    f.semente = Some(2026);
    f.unidade_independente = Some("dia".to_string());
    f.margem = Some(0.51);
    f.modelo_nulo = Some("media-historica".to_string());
    f.escopo_ecologico = true;
    f.quem_paga_pelo_erro = Some("decisor-declarado".to_string());
    f.clausula_de_saida = Some("parar-se-ECE>0.10".to_string());
    f.pergunta_do_decisor = pergunta.to_string();
    f.acao_do_pacote = acao.to_string();
    f.teste_de_propriedade = AgenteA::A1Codigo.ancora_estrutural(&f);
    f.trata_ausentes = AgenteA::A2Dados.ancora_estrutural(&f);
    f.deriva_declarada = AgenteA::A3CicloVida.ancora_estrutural(&f);
    f.patologias_cobertas = AgenteA::A4Inferencia.ancora_estrutural(&f);
    f.trilha_integra = AgenteA::A5Governanca.ancora_estrutural(&f);
    f.subgrupos_auditados = AgenteA::A6Justica.ancora_estrutural(&f);
    f.priori_elicitada = AgenteA::A7Dominio.ancora_estrutural(&f);
    f
}

/// As três fichas intermediárias que a banca reprovou ao longo dos horizontes H1, H2 e H3,
/// cada uma com cuidados e âncoras estruturais efetivamente ausentes.
fn fichas_reprovadas_no_historico() -> Vec<Ficha> {
    let mut h1_incendios_alavancar = ficha(
        "h1.c-incendios.alavancar",
        Camada::Agente,
        TipoIncerteza::Knightiana,
        "resseguradora triplica aposta apos 36 meses limpos com Brier skill -0,015",
        "devemos triplicar a aposta da seguradora?",
        "triplicar a aposta",
    );
    h1_incendios_alavancar.patologias_cobertas = false;
    h1_incendios_alavancar.clausula_de_saida = None;

    let mut h2_levedura_1linha = ficha(
        "h2.icp-levedura.1linha",
        Camada::Modelo,
        TipoIncerteza::Epistemica,
        "validacao por intervencao com uma unica linha por mutante (4 de 40 aprovados)",
        "devemos aceitar a genetica com uma linha?",
        "aceitar com uma linha",
    );
    h2_levedura_1linha.priori_elicitada = false;
    h2_levedura_1linha.modelo_nulo = None;

    let mut h3_prisoes_toronto = ficha(
        "h3.prisoes.toronto",
        Camada::Agente,
        TipoIncerteza::Knightiana,
        "recalibracao apenas de intercepto em Toronto 2020-2021 com ECE 0,067",
        "devemos operar o modelo em Toronto?",
        "operar em Toronto",
    );
    h3_prisoes_toronto.deriva_declarada = false;
    h3_prisoes_toronto.subgrupos_auditados = false;

    vec![
        h1_incendios_alavancar,
        h2_levedura_1linha,
        h3_prisoes_toronto,
    ]
}

fn fichas(horizonte: &str) -> Vec<Ficha> {
    let h0 = vec![
        ficha(
            "h0.posto-saude",
            Camada::Mundo,
            TipoIncerteza::Aleatoria,
            "fila com chegada nao linear e espera convexa",
            "devemos abrir uma segunda sala nos dias cheios?",
            "abrir uma segunda sala",
        ),
        ficha(
            "h0.banco-2003",
            Camada::Modelo,
            TipoIncerteza::Epistemica,
            "2,65 milhoes de chamadas que valem 81 dias de unidade independente",
            "devemos reforcar o atendimento nos picos?",
            "reforcar o atendimento",
        ),
    ];
    let h1 = vec![
        ficha(
            "h1.a-multiplicativo",
            Camada::Modelo,
            TipoIncerteza::Aleatoria,
            "fundo alavancado 3x sofre arrasto da volatilidade",
            "devemos reduzir a alavancagem de 3x para 1x?",
            "reduzir a alavancagem",
        ),
        ficha(
            "h1.b-lalonde",
            Camada::Modelo,
            TipoIncerteza::Epistemica,
            "comparacao ingenua carrega vies de selecao de -10292 dolares",
            "devemos manter o programa de treinamento?",
            "manter o programa",
        ),
        ficha(
            "h1.c-incendios",
            Camada::Mundo,
            TipoIncerteza::Knightiana,
            "incendios acima de 50 milhoes tem cauda com alfa 1,81",
            "devemos triplicar a aposta da seguradora?",
            "triplicar a aposta",
        ),
    ];
    let h2 = vec![
        ficha(
            "h2.icp-tunel-de-luz",
            Camada::Modelo,
            TipoIncerteza::Epistemica,
            "1024 sensores, causas invariantes entre regimes",
            "devemos declarar os tres LEDs como causa da corrente?",
            "declarar os tres LEDs",
        ),
        ficha(
            "h2.icp-fda-herg",
            Camada::Modelo,
            TipoIncerteza::Epistemica,
            "16 medidas por voluntario e tres canais ionicos",
            "devemos prever o verapamil pelo teste de invariancia?",
            "prever o verapamil",
        ),
        ficha(
            "h2.icp-estrela-levedura",
            Camada::Modelo,
            TipoIncerteza::Epistemica,
            "40 genes de levedura com p igual a 0,001",
            "devemos manter a genetica como terceiro pilar?",
            "manter a genetica",
        ),
    ];
    let h3 = vec![
        ficha(
            "h3.voos23",
            Camada::Agente,
            TipoIncerteza::Epistemica,
            "malha sob relogio com ECE 0,0048 e habilidade 0,019",
            "devemos manter o modelo no fluxo de voos?",
            "manter o modelo",
        ),
        ficha(
            "h3.bolsa",
            Camada::Agente,
            TipoIncerteza::Knightiana,
            "Goodhart medido por instrumento quatro vezes menor",
            "devemos corrigir a meta do fluxo de bolsa?",
            "corrigir a meta",
        ),
        ficha(
            "h3.energia",
            Camada::Agente,
            TipoIncerteza::Epistemica,
            "carga com canario de vazamento armado",
            "devemos soltar o canario no fluxo de energia?",
            "soltar o canario",
        ),
        ficha(
            "h3.prisoes",
            Camada::Agente,
            TipoIncerteza::Knightiana,
            "tres cofres ate acertar a calibracao policial",
            "devemos trocar o cofre de Toronto por Rhode Island?",
            "trocar o cofre",
        ),
    ];
    match horizonte {
        "h0" => h0,
        "h1" => h1,
        "h2" => h2,
        "h3" => h3,
        _ => [h0, h1, h2, h3].concat(),
    }
}

struct Execucao {
    relatorio: Vec<String>,
    portoes: Vec<(String, bool, String)>,
    raiz: String,
    falhas: usize,
}

/// Lista de fichas do horizonte, com o defeito injetado quando pedido.
fn lista_com_defeito(cfg: &Config) -> Vec<Ficha> {
    let mut l = fichas(&cfg.horizonte);
    if let Some(alvo) = &cfg.defeito {
        for f in l.iter_mut() {
            if &f.id == alvo {
                f.trilha_integra = false;
                f.subgrupos_auditados = false;
            }
        }
    }
    l
}

fn executar(cfg: &Config) -> Execucao {
    let mut relatorio = Vec::new();
    let mut portoes: Vec<(String, bool, String)> = Vec::new();
    let lista = lista_com_defeito(cfg);

    relatorio.push(format!(
        "horizonte={} fichas={} defeito={}",
        cfg.horizonte,
        lista.len(),
        cfg.defeito.clone().unwrap_or_else(|| "nenhum".to_string())
    ));

    let id_sessao = format!("sessao-{}", cfg.horizonte);
    let orq = OrquestradorM::novo(&id_sessao, "");
    let mut livro = LivroRazao::novo(&id_sessao);
    let mut banca_ok = true;
    let mut pior = 10u8;

    for f in &lista {
        let veredito = orq.banca(f);
        let mut menor = 10u8;
        for a in AgenteA::TODOS {
            let s = veredito.severidade(a).0;
            if s < menor {
                menor = s;
            }
        }
        if menor < pior {
            pior = menor;
        }
        if !veredito.aprovado {
            banca_ok = false;
        }
        relatorio.push(format!(
            "  ficha {} severidade_min={} aprovada={}",
            f.id, menor, veredito.aprovado
        ));
        livro.registrar(&f.id, &format!("{}|{}", f.definicao, veredito.aprovado));
    }

    // Amostra das lentes, tudo com semente fixa.
    let mut rng = Rng::novo(1909);
    let serie: Vec<f64> = (0..20_000)
        .map(|_| {
            let u = (rng.proximo() as f64) / (u64::MAX as f64);
            (1.0 - u).powf(-1.0 / 1.81)
        })
        .collect();
    let alfa = lentes::indice_hill(&serie, 500);
    let (mediana, arruinadas) = lentes::simular_multiplicativo(2026, 200, 2_000, 0.50, 0.40);
    let (rel, resol, unc) = lentes::decomposicao_murphy(
        &[0.1, 0.1, 0.2, 0.2, 0.8, 0.8, 0.9, 0.9],
        &[0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0],
    );
    relatorio.push(format!(
        "  lentes: hill_alfa={:.4} cauda_selvagem={} mediana_log_riqueza={:.4} arruinadas={:.3} murphy_rel={:.4} res={:.4} unc={:.4}",
        alfa,
        lentes::cauda_selvagem(alfa),
        mediana,
        arruinadas,
        rel,
        resol,
        unc
    ));

    // G1 determinismo: mesma entrada, mesma raiz.
    let espelho = executar_sem_portoes(cfg);
    let determinista = espelho.raiz == livro.raiz_merkle();

    // G2 semente fixa.
    let semente_ok = lista.iter().all(|f| f.semente.is_some());

    // G3 escopo: artefato simulável declarado para toda ficha.
    let escopo_ok = lista.iter().all(|f| f.artefato_simulavel.is_some());

    // G4 invariantes Phi1-Phi4 e trava de tipo III.
    let mut falhas_invariante = 0;
    for f in &lista {
        falhas_invariante += avaliar(f).len();
    }
    let invariantes_ok = falhas_invariante == 0;

    // G5 trilha: corrente íntegra e adulteração detectada.
    let mut adulterado = livro.clone();
    if !adulterado.eventos.is_empty() {
        adulterado.eventos[0].conteudo.push('!');
    }
    let trilha_ok = livro.verificar() && !adulterado.verificar();

    // G6 banca adversarial A1-A8, com vigilancia de deriva (A3), paridade de subgrupos (A6), Hill de H1 (A4)
    // e reprovação obrigatória das três fichas intermediárias com cuidados e âncoras ausentes.
    let alfa_h1 = hill(&serie, 500);
    let vigilancia_nula = cusum(
        &nulo_estacionario(2026, 500, 0.0, 1.0),
        &OpcoesDeriva::default(),
    );
    let paridade_nula = paridade_demografica(&[
        GrupoJustica::novo("coorte-a", 100, 30, 24, 6),
        GrupoJustica::novo("coorte-b", 100, 30, 24, 6),
    ]);
    let historico_reprovado_ok = fichas_reprovadas_no_historico()
        .iter()
        .all(|f| !orq.banca(f).aprovado);
    let modulos_banca_ok = (alfa - alfa_h1).abs() < 1e-12
        && vigilancia_nula.alarme.is_none()
        && paridade_nula.abs() < 1e-12
        && historico_reprovado_ok;
    let banca_portao_ok = banca_ok && pior >= SEVERIDADE_MINIMA.0 && modulos_banca_ok;

    // G7 cláusula de saída declarada e A8 acima do piso.
    let saida_ok = lista.iter().all(|f| f.clausula_de_saida.is_some())
        && lista.iter().all(|f| {
            orq.conduzir_ataque(AgenteA::A8Decisao, f)
                .map(|s| s >= SEVERIDADE_MINIMA)
                .unwrap_or(false)
        });

    let raiz = livro.raiz_merkle();
    portoes.push((
        "G1".to_string(),
        determinista,
        format!("determinismo da raiz {}", &raiz[..16]),
    ));
    portoes.push((
        "G2".to_string(),
        semente_ok,
        format!("semente fixa em {} fichas", lista.len()),
    ));
    portoes.push((
        "G3".to_string(),
        escopo_ok,
        "escopo declarado: so artefato simulado".to_string(),
    ));
    portoes.push((
        "G4".to_string(),
        invariantes_ok,
        format!("Phi1-Phi5 e tipo III (falhas={})", falhas_invariante),
    ));
    portoes.push((
        "G5".to_string(),
        trilha_ok,
        "corrente SHA-256 integra e adulteracao detectada".to_string(),
    ));
    portoes.push((
        "G6".to_string(),
        banca_portao_ok,
        format!(
            "banca A1-A8 com piso {} e pior {}",
            SEVERIDADE_MINIMA.0, pior
        ),
    ));
    portoes.push((
        "G7".to_string(),
        saida_ok,
        "clausula de saida declarada e A8 acima do piso".to_string(),
    ));

    let falhas = portoes.iter().filter(|(_, ok, _)| !ok).count();
    // A raiz entra no relatório para que duas execuções possam ser comparadas byte a byte.
    relatorio.push(format!("raiz_merkle={}", raiz));

    Execucao {
        relatorio,
        portoes,
        raiz,
        falhas,
    }
}

/// Mesma passada, sem portões: serve de espelho para o G1.
fn executar_sem_portoes(cfg: &Config) -> Execucao {
    let lista = lista_com_defeito(cfg);
    let id_sessao = format!("sessao-{}", cfg.horizonte);
    let mut livro = LivroRazao::novo(&id_sessao);
    let orq = OrquestradorM::novo(&id_sessao, "");
    for f in &lista {
        let veredito = orq.banca(f);
        livro.registrar(&f.id, &format!("{}|{}", f.definicao, veredito.aprovado));
    }
    let raiz = livro.raiz_merkle();
    Execucao {
        relatorio: Vec::new(),
        portoes: Vec::new(),
        raiz,
        falhas: 0,
    }
}

fn main() {
    let cfg = config();
    let exec = executar(&cfg);
    println!("== Estocologia :: portoes deterministicos ==");
    for linha in &exec.relatorio {
        println!("{}", linha);
    }

    if cfg.reproducao {
        let segunda = executar(&cfg);
        let identico = segunda.relatorio == exec.relatorio && segunda.raiz == exec.raiz;
        println!(
            "reproducao: segunda execucao identica = {} (raiz {})",
            identico,
            &segunda.raiz[..16]
        );
    }

    for (nome, ok, detalhe) in &exec.portoes {
        println!(
            "{} {} {}",
            nome,
            if *ok { "ok      " } else { "reprovado" },
            detalhe
        );
    }

    let hash_relatorio = sha256_hex(exec.relatorio.join("|").as_bytes());
    println!("hash_do_relatorio={}", hash_relatorio);

    if exec.falhas == 0 {
        println!("veredito: 7 portoes ok");
        std::process::exit(0);
    }
    println!("veredito: {} portoes reprovados", exec.falhas);
    std::process::exit(1);
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn sete_portoes_aprovam_horizonte_todos_com_reproducao_identica() {
        let cfg = Config {
            horizonte: "todos".to_string(),
            reproducao: true,
            defeito: None,
        };
        let a = executar(&cfg);
        let b = executar(&cfg);
        assert_eq!(a.falhas, 0);
        assert_eq!(a.portoes.len(), 7);
        assert_eq!(a.relatorio, b.relatorio);
        assert_eq!(a.raiz, b.raiz);
        assert!(a.raiz.starts_with("9716e28a87bec9cf"));
    }

    #[test]
    fn defeito_em_h3_voos23_reprova_portao_g6() {
        let cfg = Config {
            horizonte: "h3".to_string(),
            reproducao: false,
            defeito: Some("h3.voos23".to_string()),
        };
        let exec = executar(&cfg);
        assert_eq!(exec.falhas, 1);
        let g6 = exec.portoes.iter().find(|(n, _, _)| n == "G6").expect("G6");
        assert!(
            !g6.1,
            "G6 deve reprovar quando h3.voos23 perde trilha e subgrupos"
        );
    }

    #[test]
    fn cada_horizonte_individual_fecha_os_sete_portoes() {
        for h in ["h0", "h1", "h2", "h3"] {
            let cfg = Config {
                horizonte: h.to_string(),
                reproducao: false,
                defeito: None,
            };
            let exec = executar(&cfg);
            assert_eq!(exec.falhas, 0, "horizonte {h} falhou");
        }
        let orq = OrquestradorM::novo("sessao-todos", "");
        let reprovadas = fichas_reprovadas_no_historico();
        assert_eq!(reprovadas.len(), 3);
        for f in &reprovadas {
            let v = orq.banca(f);
            assert!(
                !v.aprovado,
                "ficha historica {} deveria reprovar na banca",
                f.id
            );
        }
    }
}
