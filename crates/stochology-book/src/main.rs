//! Instrumento dos laboratórios do livro.
//!
//! Uso: ~cargo run --release --bin labs -- --cap 4~
//!      ~cargo run --release --bin labs -- --todos~

use stochology_book::{banco, cadeia, escuta, fila, genetica, jev, ponte, segunda};
use stochology_core::s1::{Categoria, S1};
use stochology_core::resolucao::{Resolucao, medir as medir_perda};
use stochology_book::{
    cap01, cap02, cap03, cap04, cap05, cap06, cap07, cap08, cap09, cap10, cap11, cap12, cap13, cap14,
    cap15, cap16, cap17, cap18, cap19, cap20, cap21,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut cap = 0usize;
    let mut todos = false;
    let mut resolucao = false;
    let mut banco = false;
    let mut sistema1 = false;
    let mut sistema2 = false;
    let mut pre_teste_flag = false;
    let mut respostas: Option<String> = None;
    let mut elicitacao: Option<String> = None;
    let mut limiar: Option<f64> = None;
    let mut escala: Option<String> = None;
    let mut gabarito: Option<String> = None;
    let mut fila_flag = false;
    let mut genetica_flag = false;
    let mut escuta_flag = false;
    let mut cadeia_flag = false;
    let mut ponte_flag = false;
    let mut schema_flag = false;
    let mut segunda_flag = false;
    let mut jev_flag = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--cap" => {
                if i + 1 < args.len() {
                    cap = args[i + 1].parse().unwrap_or(0);
                    i += 1;
                }
            }
            "--todos" => todos = true,
            "--resolucao" => resolucao = true,
            "--banco" => banco = true,
            "--s1" => sistema1 = true,
            "--s2" => sistema2 = true,
            "--preteste" => pre_teste_flag = true,
            "--respostas" => {
                if i + 1 < args.len() {
                    respostas = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--escala" => {
                if i + 1 < args.len() {
                    escala = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--limiar" => {
                if i + 1 < args.len() {
                    limiar = args[i + 1].replace(',', ".").parse::<f64>().ok();
                    i += 1;
                }
            }
            "--elicitacao" => {
                if i + 1 < args.len() {
                    elicitacao = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--gabarito" => {
                if i + 1 < args.len() {
                    gabarito = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--fila" => fila_flag = true,
            "--genetica" => genetica_flag = true,
            "--escuta" => escuta_flag = true,
            "--h3" => cadeia_flag = true,
            "--ponte" => ponte_flag = true,
            "--schema" => schema_flag = true,
            "--segunda" => segunda_flag = true,
            "--jev" => jev_flag = true,
            _ => {}
        }
        i += 1;
    }
    if resolucao {
        horizonte_r();
        return;
    }
    if banco {
        h0_real();
        return;
    }
    if sistema1 {
        sistema_1();
        return;
    }
    if sistema2 {
        sistema_2();
        return;
    }
    if pre_teste_flag {
        pre_teste_do_painel(respostas.as_deref(), gabarito.as_deref());
        return;
    }
    if let Some(entrada) = elicitacao.as_deref() {
        elicitar(entrada, limiar, escala.as_deref());
        return;
    }
    if fila_flag {
        fila::rodar();
        return;
    }
    if genetica_flag {
        genetica::rodar();
        return;
    }
    if escuta_flag {
        escuta::rodar(escuta::REGISTRO);
        return;
    }
    if cadeia_flag {
        cadeia::rodar(cadeia::REGISTRO, cadeia::RESULTADOS_DO_LACO, cadeia::POSTHOC_DO_LACO);
        return;
    }
    if ponte_flag {
        ponte::rodar(ponte::REGISTRO, ponte::NOTAS);
        return;
    }
    if schema_flag {
        contrato();
        return;
    }
    if segunda_flag {
        segunda::rodar();
        return;
    }
    if jev_flag {
        jev::rodar();
        return;
    }
    if todos {
        for n in 1usize..=21 {
            rodar(n);
        }
        return;
    }
    rodar(cap);
}

/// Sistema 1: triagem determinística de fichas, com o vocabulário fechado.
fn sistema_1() {
    println!("== Sistema 1 :: triagem determinística de fichas ==");
    let s1 = S1::novo();
    println!(
        "arvore de sufixos: {} sufixos indexados em {} temas declarados",
        s1.sufixos_indexados(),
        Categoria::TODAS.len()
    );
    let amostras = [
        "tempo de espera na fila do posto de saude",
        "efeito causal do tratamento sobre a renda",
        "cauda pesada de evento raro e extremo",
        "calibracao do escore por platt com brier",
        "justica entre subgrupos e quem paga o erro",
        "xyzzy plugh abracadabra",
    ];
    for texto in amostras {
        let t = s1.triar(texto);
        println!(
            "{} | probabilidade {:.4} | soma {:.4} | fora de [0.01;0.99] {}",
            t.linha(),
            t.probabilidade,
            t.soma(),
            t.fora_do_intervalo()
        );
    }
}


/// Elicitação SHELF: o ajuste da priori pelos quartis e os controles do protocolo.
///
/// Uso: `labs --elicitacao q05,q25,q50,q75,q95`
fn elicitar(entrada: &str, limiar: Option<f64>, escala: Option<&str>) {
    use stochology_h3::shelf::{
        ANCORA_ESPERADA, ANCORA_TOLERANCIA, RAZAO_LARGURA_MINIMA, ajustar_normal,
        excesso_de_confianca_ok, largura_90_elicitada, largura_90_implicada, p_acima_de,
        razao_largura_90,
    };

    let valores: Vec<f64> = entrada
        .split(',')
        .filter_map(|t| t.trim().replace(',', ".").parse::<f64>().ok())
        .collect();
    if valores.len() != 5 {
        println!("preciso de cinco quantis na ordem q05,q25,q50,q75,q95; vieram {}", valores.len());
        return;
    }
    let [q05, q25, q50, q75, q95] = [valores[0], valores[1], valores[2], valores[3], valores[4]];
    println!("== Elicitação SHELF :: ajuste pelos quartis e controles ==");
    if escala == Some("beta") {
        // domínio de frequência em (0, 1): o ajuste é Beta, e o protocolo declara isso
        use stochology_h3::beta::{ajustar_beta, beta_quantil};
        println!(
            "escala declarada: Beta (frequência em (0, 1)) | quantis elicitados: q05 {q05} | q25 {q25} | q50 {q50} | q75 {q75} | q95 {q95}"
        );
        let p_beta = ajustar_beta(q25, q50, q75);
        println!(
            "Beta ajustada aos três quartis centrais: a = {:.6} | b = {:.6} | erro máximo nos quartis {:.3e}",
            p_beta.a, p_beta.b, p_beta.erro_max_quartis
        );
        println!(
            "quantis que ela implica: q05 {:.4} | q25 {:.4} | q50 {:.4} | q75 {:.4} | q95 {:.4}",
            beta_quantil(0.05, p_beta.a, p_beta.b),
            beta_quantil(0.25, p_beta.a, p_beta.b),
            beta_quantil(0.50, p_beta.a, p_beta.b),
            beta_quantil(0.75, p_beta.a, p_beta.b),
            beta_quantil(0.95, p_beta.a, p_beta.b)
        );
        let elicitada = q95 - q05;
        let implicada =
            beta_quantil(0.95, p_beta.a, p_beta.b) - beta_quantil(0.05, p_beta.a, p_beta.b);
        let razao = if implicada > 0.0 { elicitada / implicada } else { f64::NAN };
        println!(
            "largura 90%: elicitada {:.4} | implicada pela própria Beta {:.4} | razão {:.4} (mínimo declarado {:.2}) -> {}",
            elicitada,
            implicada,
            razao,
            RAZAO_LARGURA_MINIMA,
            if excesso_de_confianca_ok(razao) { "ok" } else { "EXCESSO DE CONFIANÇA" }
        );
        let na_mediana = 1.0 - stochology_h3::beta::beta_incompleta(p_beta.a, p_beta.b, q50);
        println!(
            "âncora: P(theta > mediana) pela própria Beta = {:.4} | esperado {:.2} +- {:.2}",
            na_mediana, ANCORA_ESPERADA, ANCORA_TOLERANCIA
        );
        if let Some(s) = limiar {
            println!(
                "limiar da decisão em {s}: P(theta > {s}) pela Beta = {:.4}",
                1.0 - stochology_h3::beta::beta_incompleta(p_beta.a, p_beta.b, s)
            );
        }
        return;
    }
    if escala == Some("log") {
        // domínio binomial em razão de chances: os quantis viram log antes do ajuste
        let logs: Vec<f64> = valores.iter().map(|v| v.ln()).collect();
        println!(
            "escala declarada: log natural (razão de chances) | quantis elicitados: q05 {q05} | q25 {q25} | q50 {q50} | q75 {q75} | q95 {q95}"
        );
        println!(
            "no log: q05 {:.6} | q25 {:.6} | q50 {:.6} | q75 {:.6} | q95 {:.6}",
            logs[0], logs[1], logs[2], logs[3], logs[4]
        );
        let p_log = ajustar_normal(logs[1], logs[2], logs[3]);
        println!(
            "Normal ajustada no log: média {:.6} | dp {:.6} | erro máximo nos quartis {:.3e}",
            p_log.media, p_log.dp, p_log.erro_max_quartis
        );
        let elicitada = largura_90_elicitada(logs[0], logs[4]);
        let implicada = largura_90_implicada(p_log.dp);
        let razao = razao_largura_90(elicitada, p_log.dp);
        println!(
            "largura 90% no log: elicitada {:.4} | implicada pela própria Normal {:.4} | razão {:.4} (mínimo declarado {:.2}) -> {}",
            elicitada,
            implicada,
            razao,
            RAZAO_LARGURA_MINIMA,
            if excesso_de_confianca_ok(razao) { "ok" } else { "EXCESSO DE CONFIANÇA" }
        );
        println!(
            "no log 0 é razão de chances 1 (tratamento igual): P(RC > 1) na própria priori = {:.4}",
            p_acima_de(0.0, p_log.media, p_log.dp)
        );
        if let Some(s) = limiar {
            println!(
                "limiar da decisão em {s} (razão de chances): P(RC > {s}) na própria priori = {:.4}",
                p_acima_de(s.ln(), p_log.media, p_log.dp)
            );
        }
        let mediana = p_acima_de(logs[2], p_log.media, p_log.dp);
        println!(
            "âncora com S igual à mediana do próprio especialista: P(RC > {q50}) = {:.4} (esperado {:.2} +- {:.2})",
            mediana, ANCORA_ESPERADA, ANCORA_TOLERANCIA
        );
        return;
    }
    println!(
        "quantis elicitados: q05 {q05} | q25 {q25} | q50 {q50} | q75 {q75} | q95 {q95}"
    );
    let priori = ajustar_normal(q25, q50, q75);
    println!(
        "Normal ajustada por mínimos quadrados em q25/q50/q75: média {:.6} | dp {:.6} | erro máximo nos quartis {:.3e}",
        priori.media, priori.dp, priori.erro_max_quartis
    );
    let elicitada = largura_90_elicitada(q05, q95);
    let implicada = largura_90_implicada(priori.dp);
    let razao = razao_largura_90(elicitada, priori.dp);
    println!(
        "largura 90%: elicitada {:.4} | implicada pela própria Normal {:.4} | razão {:.4} (mínimo declarado {:.2}) -> {}",
        elicitada,
        implicada,
        razao,
        RAZAO_LARGURA_MINIMA,
        if excesso_de_confianca_ok(razao) { "ok" } else { "EXCESSO DE CONFIANÇA" }
    );
    println!(
        "âncora com S igual à mediana do próprio especialista: esperado {:.2} +- {:.2} em P(theta > S)",
        ANCORA_ESPERADA, ANCORA_TOLERANCIA
    );
    if let Some(s) = limiar {
        println!(
            "limiar da decisão em {s}: P(theta > {s}) na própria priori = {:.4} | P(theta > S) com S na mediana = {:.4}",
            p_acima_de(s, priori.media, priori.dp),
            p_acima_de(priori.media, priori.media, priori.dp)
        );
    }
}

/// Pré-teste de calibração do painel SHELF: o nulo declarado do desenho do protocolo.
fn pre_teste_do_painel(respostas: Option<&str>, gabarito: Option<&str>) {
    use stochology_h3::preteste::{
        BALDES_AJUSTADOS, BALDES_DO_PROTOCOLO, ECE_MAXIMO_AJUSTADO, ECE_MAXIMO_DO_PROTOCOLO,
        avaliar_ajustado, avaliar_com, nulo_com_gate, probabilidades_invalidas,
    };

    if let (Some(respostas), Some(gabarito)) = (respostas, gabarito) {
        let confianca: Vec<f64> = respostas
            .split(',')
            .filter_map(|t| t.trim().parse::<f64>().ok())
            .collect();
        let acerto: Vec<bool> = gabarito
            .split(',')
            .map(|t| t.trim().eq_ignore_ascii_case("sim"))
            .collect();
        if confianca.len() != acerto.len() || confianca.is_empty() {
            println!("respostas com {} valores e gabarito com {}: não pontuo", confianca.len(), acerto.len());
            return;
        }
        let invalidas = probabilidades_invalidas(&confianca);
        if !invalidas.is_empty() {
            println!("p fora de (0, 1) nos itens {:?}: o protocolo recusa 0 e 1 exatos", invalidas);
        }
        println!("== Pré-teste SHELF :: pontuação de um especialista ==");
        println!("itens: {} | p distintos: {}", confianca.len(), {
            let mut v = confianca.clone();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            v.dedup();
            v.len()
        });
        // declaração 3 do ponto 12: o critério 3 vale pelo teste binomial, e o portão em vigor é o ajustado
        let ajustado = avaliar_ajustado(&confianca, &acerto);
        println!(
            "Brier {:.6} | climatologia do pool {:.6} | habilidade {:.6} | REL {:.6} RES {:.6} UNC {:.6}",
            ajustado.brier,
            ajustado.brier_clima,
            ajustado.habilidade,
            ajustado.murphy.rel,
            ajustado.murphy.res,
            ajustado.murphy.unc
        );
        for caixa in &ajustado.caixas {
            println!(
                "  caixa {} | n {} | media prevista {:.4} | taxa observada {:.4} | desvio {:+.4}",
                caixa.indice, caixa.n, caixa.media_prevista, caixa.taxa_observada, caixa.desvio()
            );
        }
        let caixas_testadas = stochology_h3::preteste::caixas_por_teste_binomial(&ajustado.caixas);
        for t in &caixas_testadas {
            println!(
                "  caixa {} pelo teste binomial: {} acertos de {} com previsao {:.4} | p bilateral {:.4} | p com Bonferroni {:.4} -> {}",
                t.indice,
                t.acertos,
                t.n,
                t.media_prevista,
                t.p_bilateral,
                t.p_com_bonferroni,
                if t.sistematica(0.05) { "SISTEMATICA" } else { "ok" }
            );
        }
        let escrito_limiar = avaliar_com(&confianca, &acerto, BALDES_AJUSTADOS, ECE_MAXIMO_AJUSTADO);
        let por_limiar_fixo = escrito_limiar
            .reprovacoes
            .iter()
            .filter(|x| matches!(x, stochology_h3::preteste::Reprovacao::CaixaSistemáticaPorLimiarFixo { .. }))
            .count();
        println!(
            "criterio 3, as duas reguas: pelo teste binomial (em vigor, declaracao 3) {} | pelo limiar fixo de 0,25 (como escrito) {}",
            if caixas_testadas.iter().any(|t| t.sistematica(0.05)) { "reprovado" } else { "ok" },
            if por_limiar_fixo > 0 { "reprovado" } else { "ok" }
        );
        println!(
            "curva observada (diagnostico que morde): {}",
            if stochology_h3::preteste::curva_observada_e_fracamente_monotona(&ajustado.caixas) { "fracamente monotona" } else { "NAO monotonona" }
        );
        for caixa in &ajustado.caixas {
            println!("  item a item da caixa {}: ver o formulario do especialista", caixa.indice);
        }
        println!(
            "portao ajustado, em vigor ({} caixas, ECE <= {}, criterio 3 pelo teste binomial): ECE {:.4} -> {}",
            BALDES_AJUSTADOS,
            ECE_MAXIMO_AJUSTADO,
            ajustado.ece,
            if ajustado.aprovado { "APROVADO" } else { "REPROVADO" }
        );
        for r in &ajustado.reprovacoes {
            println!("  reprovacao: {r:?}");
        }
        let escrito = avaliar_com(&confianca, &acerto, BALDES_DO_PROTOCOLO, ECE_MAXIMO_DO_PROTOCOLO);
        println!(
            "portao do protocolo como escrito ({} caixas, ECE <= {}): ECE {:.4} -> {}",
            BALDES_DO_PROTOCOLO,
            ECE_MAXIMO_DO_PROTOCOLO,
            escrito.ece,
            if escrito.aprovado { "aprovado" } else { "reprovado" }
        );
        return;
    }

    println!("== Pré-teste SHELF :: calibração do painel ==");
    println!(
        "desenho do protocolo: 20 itens, 10 caixas de massa igual; limiar do ECE {}",
        ECE_MAXIMO_DO_PROTOCOLO
    );
    println!(
        "limites medidos do desenho: o critério 3 (caixa com n >= 4) fica vacuoso, porque 20 itens em 10 caixas dão 2 por caixa; o critério 4, na letra do protocolo, é inerte: as caixas são cortadas por p ordenado e a média prevista da de cima supera a da de baixo por construção"
    );
    for (itens, baldes, limiar) in [
        (20usize, 5usize, ECE_MAXIMO_AJUSTADO),
        (20, BALDES_DO_PROTOCOLO, ECE_MAXIMO_DO_PROTOCOLO),
        (200, BALDES_DO_PROTOCOLO, ECE_MAXIMO_DO_PROTOCOLO),
    ] {
        let nulo = nulo_com_gate(2026, 20_000, itens, baldes, limiar);
        println!(
            "nulo calibrado, {itens} itens em {baldes} caixas, limiar de ECE {limiar}: ECE p50 {:.4} | p90 {:.4} | p95 {:.4} | maximo {:.4} | habilidade p50 {:.4} | p10 {:.4} | Brier p50 {:.4}",
            nulo.ece_p50,
            nulo.ece_p90,
            nulo.ece_p95,
            nulo.ece_maximo,
            nulo.habilidade_p50,
            nulo.habilidade_p10,
            nulo.brier_p50
        );
        println!(
            "    veredito do portao no mundo calibrado: aprovados {:.4} | reprovados pelo criterio 3 {:.4} | pelo criterio 3 corrigido (binomial) {:.4} | pelo criterio 2 {:.4} | por algum criterio {:.4}",
            nulo.quantos_passam,
            nulo.reprovados_criterio_3,
            nulo.reprovados_criterio_3_binomial,
            nulo.reprovados_criterio_2,
            nulo.reprovados_por_algum
        );
    }
}

/// Sistema 2: deliberação sobre o hipergrafo, crença da banca, inferência e eixo R.
fn sistema_2() {
    use stochology_core::medir_perda_de_resolucao;
    use stochology_core::{Camada, Ficha, TipoIncerteza};
    use stochology_s2::{
        CrencaDaBanca, Lats, Metropolis, MundoHipergrafo, ORDEM_DECLARADA, aprender,
        avaliar_resolucoes, decidir, destino_da_acao, estressar_deliberativo, estressar_em_ordem,
        ganancioso, grafo_de_amostra, log_beta,
    };

    println!("== Sistema 2 :: deliberação sobre o hipergrafo ==");
    let grafo = grafo_de_amostra();
    let problemas = grafo.validar();
    println!(
        "grafo declarado: {} nos, {} arestas de arvore, {} de grafo, {} hiperarestas | validacao {}",
        grafo.nos.len(),
        grafo.arestas_tree.len(),
        grafo.arestas_graph.len(),
        grafo.hiperarestas.len(),
        if problemas.is_empty() { "ok".to_string() } else { format!("{problemas:?}") }
    );
    let mundo = MundoHipergrafo::novo(&grafo);
    println!("nos que nenhuma lente cobre: {:?}", mundo.nos_descobertos());
    let lats = Lats { profundidade: 2, orcamento: 40, exploracao: 1.0, semente: 2026 };
    let guloso = ganancioso(&mundo, "raiz.estocologia").expect("o grafo tem saidas");
    let busca = mundo.busca("raiz.estocologia", &lats, 3).expect("a busca chega");
    println!(
        "ganancioso: para em {} (valor {:.3}) | LATS: {} (valor {:.3}, no sem lente {})",
        destino_da_acao(&guloso.acao),
        guloso.valor,
        busca.caminho.join(" -> "),
        busca.valor,
        busca.sem_lente
    );

    let ficha_completa = |id: &str| {
        Ficha::nova(id, Camada::Agente, TipoIncerteza::Epistemica).com_cuidados().completa()
    };
    let mut banca = CrencaDaBanca::nova();
    let treino: Vec<Ficha> = (0..4)
        .map(|i| {
            let mut f = ficha_completa(&format!("s2.treino.{i}"));
            f.trilha_integra = false;
            f
        })
        .collect();
    aprender(&mut banca, &treino, 2026, 3.0);
    let topo = banca.ordem()[0];
    println!(
        "crenca da banca: faixa mais provavel {} com probabilidade {:.4} | entropia {:.4}",
        topo.nome(),
        banca.probabilidade(topo),
        banca.entropia()
    );
    let mut alvo = ficha_completa("s2.alvo");
    alvo.trilha_integra = false;
    let deliberado = estressar_deliberativo(&alvo, 2026, &banca);
    let declarado = estressar_em_ordem(&alvo, 2026, &ORDEM_DECLARADA);
    println!(
        "estresse: deliberado parou em {} com {} ataque(s) | ordem declarada parou em {} com {} ataques | mesma reprovacao {}",
        deliberado.agente.nome(),
        deliberado.ataques,
        declarado.agente.nome(),
        declarado.ataques,
        deliberado.reprovou == declarado.reprovou
    );

    let cadeia = Metropolis { passo: 0.15, semente: 7 }.amostrar(log_beta(4.0, 8.0), 0.5, 200_000);
    println!(
        "inferencia: Beta(4, 8) media da cadeia {:.4} contra a forma fechada {:.4} | aceitacao {:.3}",
        cadeia.media(20_000),
        1.0 / 3.0,
        cadeia.aceitacao()
    );

    let perda = medir_perda_de_resolucao(1909, 200, 10, 5.0, 11.0);
    let decisao = decidir(&perda, 12.0);
    let fora: Vec<String> = avaliar_resolucoes(&perda, 12.0)
        .iter()
        .filter(|a| a.fora_do_alcance.is_some())
        .map(|a| a.resolucao.codigo().to_string())
        .collect();
    println!(
        "eixo R: perda do agregado {:.2} min contra margem de 12,00 min -> {} ({}) | fora do alcance medido: {}",
        perda.perda,
        decisao.escolhida.codigo(),
        decisao.motivo,
        fora.join(", ")
    );
    let sem_oscilacao = medir_perda_de_resolucao(1909, 200, 10, 8.0, 8.0);
    println!(
        "eixo R sem oscilacao: perda {:.2} min -> {}",
        sem_oscilacao.perda,
        decidir(&sem_oscilacao, 12.0).escolhida.codigo()
    );
}

/// H0 real em Rust: a central de chamadas do banco.
fn h0_real() {
    println!("== H0 real :: a central de chamadas do banco, em Rust ==");
    let caminho = match std::env::var("ESTOCOLOGIA_DADOS") {
        Ok(base) => format!("{}/dados-reais/bank_calls.csv", base.trim_end_matches('/')),
        Err(_) => "dados-reais/bank_calls.csv".to_string(),
    };
    let m = banco::medir(&caminho, 1909, 400);
    println!(
        "treino {} dias (mar-jun) | cofre {} dias (jul-out) | {:.0} chamadas no cofre",
        m.dias_treino, m.dias_cofre, m.chamadas_cofre
    );
    println!(
        "escala do perfil medio: {} agentes no pico, {:.0} agente-horas por dia",
        m.agentes_no_pico, m.agentes_por_dia
    );
    println!(
        "nivel de servico no perfil medio = {:.4} | media dos dias do cofre = {:.4} | dias abaixo da meta de {:.0}% = {:.4}",
        m.nivel_no_perfil_medio, m.nivel_medio_dos_dias, banco::META * 100.0, m.fracao_dias_abaixo_da_meta
    );
    println!(
        "dispersao entre dias: Var/media = {:.1} no cofre, {:.1} no treino, {:.1} em todos | desvio entre dias = {:.0} | Poisson preveria {:.0}",
        m.var_sobre_media, m.var_sobre_media_treino, m.var_sobre_media_todos, m.sd_entre_dias, m.poisson_sd
    );
    println!(
        "nulo de metades: por dia = {:.4} | por chamada = {:.4}",
        m.nulo_metades_dia, m.nulo_metades_chamada
    );
}

/// Horizonte R: quanto custa perguntar grosso, com o mesmo recurso total.
fn horizonte_r() {
    println!("== Horizonte R :: resolucao da pergunta ==");
    println!("bracos pareados: mesmas chegadas e mesmas bases de servico; muda so a taxa de atendimento");
    for (rotulo, baixa, alta) in [
        ("demanda 5 ou 11 (o posto do Capitulo 16)", 5.0_f64, 11.0_f64),
        ("demanda 7 ou 9 (oscilacao leve)", 7.0, 9.0),
        ("demanda 8 ou 8 (sem oscilacao)", 8.0, 8.0),
    ] {
        let p = medir_perda(1909, 200, 10, baixa, alta);
        println!(
            "{}: capacidade media {:.1} | espera agregada {:.2} min, espera fina {:.2} min, perda {:.2} min",
            rotulo, p.capacidade_agregada, p.espera_agregada, p.espera_fina, p.perda
        );
        println!(
            "   acima de 30 min: agregada {:.4}, fina {:.4}",
            p.acima30_agregada, p.acima30_fina
        );
    }
    let finuras: Vec<&str> = Resolucao::TODAS.iter().map(|r| r.codigo()).collect();
    println!("escada de resolucao: {}", finuras.join(" -> "));

    // ---- previsao 3: o limiar de oscilacao
    println!("\n-- previsao 3: o limiar de oscilacao (dia falho: mais de 10% acima de 30 min; limiar: mais de 1/4 dos dias) --");
    let amplitudes = [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];
    let calibrada = stochology_core::resolucao::escala_calibrada(1909, 200, 10, 8.0, 0.10);
    let erlang = stochology_core::resolucao::limiar_de_oscilacao(1909, 200, 10, 8.0, &amplitudes);
    println!(
        "  {} dias por amplitude | 10 h por dia | especificacao 1: o plano de Erlang do dia medio, {} agentes",
        erlang.pontos[0].dias, erlang.pontos[0].agentes_r0
    );
    println!("  {:>10} {:>12} {:>14} {:>12} {:>10}", "amplitude", "dias falhos", "fracao", "perda (min)", "acima30 R0");
    for p in &erlang.pontos {
        println!(
            "  {:>10.1} {:>12} {:>14.3} {:>12.2} {:>10.4}",
            p.amplitude, p.dias_falhos, p.fracao_de_dias_falhos, p.perda, p.acima30_r0
        );
    }
    match erlang.limiar {
        Some(a) => println!(
            "  limiar: amplitude {:.1} | perda de resolucao no limiar {:.2} min | monotona: {}",
            a,
            erlang.perda_no_limiar.unwrap_or(f64::NAN),
            erlang.monotona
        ),
        None => println!("  limiar: NAO ENCONTRADO no grid (a previsao 3 nao se sustenta aqui) | monotona: {}", erlang.monotona),
    }

    println!("\n-- previsao 3, especificacao 2: a escala de R0 calibrada na demanda plana --");
    let l = stochology_core::resolucao::limiar_de_oscilacao_com_plano(1909, 200, 10, 8.0, &amplitudes, calibrada);
    println!("  a escala calibrada tem {} agentes (a plana falha no maximo 10% dos dias)", calibrada);
    println!("  {:>10} {:>12} {:>14} {:>12} {:>10}", "amplitude", "dias falhos", "fracao", "perda (min)", "acima30 R0");
    for p in &l.pontos {
        println!(
            "  {:>10.1} {:>12} {:>14.3} {:>12.2} {:>10.4}",
            p.amplitude, p.dias_falhos, p.fracao_de_dias_falhos, p.perda, p.acima30_r0
        );
    }
    match l.limiar {
        Some(a) => println!(
            "  limiar: amplitude {:.1} | perda de resolucao no limiar {:.2} min | monotona: {}",
            a,
            l.perda_no_limiar.unwrap_or(f64::NAN),
            l.monotona
        ),
        None => println!("  limiar: NAO ENCONTRADO no grid (a previsao 3 nao se sustenta aqui) | monotona: {}", l.monotona),
    }

    // ---- previsao 5: a transferencia
    println!("\n-- previsao 5: a transferencia (sobrevive a R1 implica sobrevive a R0; o contrario nao) --");
    let perfis = [(8.0, 0.5), (8.0, 1.0), (8.0, 1.5), (8.0, 2.0), (8.0, 2.5), (8.0, 3.0)];
    let t = stochology_core::resolucao::transferencia(1909, 200, 10, &perfis);
    println!(
        "  {} perfis x 200 dias | limiar do dia {:.2} | violacoes da implicacao: {} | contraexemplos: {} | conservacao do agregado conferida: {}",
        t.perfis, t.limiar, t.violacoes, t.contraexemplos, t.conservacao_conferida
    );
    println!("  leitura: nenhum perfil sobrevive a R1 e falha em R0; {} perfis passam em R0 e falham em algum dia", t.contraexemplos);
    let l_limiar = l.limiar;

    // ---- o eixo R, capítulo por capítulo
    println!("\n-- o eixo R, capitulo por capitulo (a tabela esta em book/eixo-r.tsv; a leitura e calculada aqui) --");
    let perfis: [(&str, f64, f64, &str); 3] = [
        ("posto", 5.0, 11.0, "demanda de 5 a 11"),
        ("posto-leve", 7.0, 9.0, "demanda de 7 a 9"),
        ("plano", 8.0, 8.0, "demanda plana"),
    ];
    let tabela = std::fs::read_to_string("book/eixo-r.tsv").unwrap_or_default();
    for linha in tabela.lines() {
        let l = linha.trim_end();
        if l.is_empty() || l.starts_with('#') {
            continue;
        }
        let campos: Vec<&str> = l.split('\t').collect();
        if campos.len() < 3 || campos[0] == "cap" {
            continue;
        }
        let (cap, caso, frase) = (campos[0], campos[1], campos[2]);
        let leitura = match perfis.iter().find(|(nome, _, _, _)| *nome == caso) {
            Some((_, baixa, alta, rotulo)) => {
                let p = medir_perda(1909, 200, 10, *baixa, *alta);
                format!("perda de resolucao {:.2} min ({})", p.perda, rotulo)
            }
            None => match caso {
                "previsoes" => match l_limiar {
                    Some(amplitude) => format!("limiar de oscilacao na amplitude {amplitude:.1} (plano calibrado)"),
                    None => "limiar de oscilacao nao encontrado".to_string(),
                },
                _ => "sem perda medida neste capitulo".to_string(),
            },
        };
        println!("capítulo {cap:>2} | {leitura} | {frase}");
    }
}


fn contrato() {
    println!("== O contrato :: a ficha em arquivo, a versao e a cadeia de migracoes ==");
    println!(
        "versao da ficha: {} | cadeia declarada em crates/stochology-schema/migracoes.tsv",
        stochology_schema::VERSAO_DA_FICHA.texto()
    );
    for (i, etapa) in stochology_schema::PADRAO.descricao().iter().enumerate() {
        println!("  etapa {}: {}", i + 1, etapa);
    }
    println!(
        "  versoes da cadeia: {}",
        stochology_schema::PADRAO
            .versoes()
            .iter()
            .map(|v| v.texto())
            .collect::<Vec<_>>()
            .join(" -> ")
    );

    println!("\n-- demonstracao: um documento da versao 1.0 migra ate a versao da ficha --");
    let mut ficha = stochology_core::Ficha::nova("lab.schema.1", stochology_core::Camada::Modelo, stochology_core::TipoIncerteza::Epistemica)
        .com_resolucao(stochology_core::Resolucao::R3Subpergunta)
        .completa()
        .com_cuidados();
    ficha.definicao = "ficha de demonstracao do contrato".into();
    ficha.pergunta_do_decisor = "cada dia, o servico da conta?".into();
    ficha.acao_do_pacote = "dimensionar a escala do dia".into();

    let atual = stochology_schema::texto(&ficha);
    let mut antigo = String::from("versao = 1.0\n");
    for linha in atual.lines().skip(1) {
        if linha.starts_with("resolucao ") || linha.starts_with("priori_elicitada ") {
            continue;
        }
        antigo.push_str(linha);
        antigo.push('\n');
    }
    println!("  o documento da 1.0 nao declara os dois campos que a cadeia acrescenta");
    match stochology_schema::de_texto(&antigo) {
        Ok(migrada) => {
            println!(
                "  migrou: resolucao = {} (o valor de acrescimo declarado) | priori_elicitada = {} | definicao intacta = {}",
                migrada.resolucao.codigo(),
                migrada.priori_elicitada,
                migrada.definicao == ficha.definicao
            );
            println!(
                "  a ficha migrada validou: id {} | camada {} | acao \"{}\"",
                migrada.id,
                migrada.camada.codigo(),
                migrada.acao_do_pacote
            );
        }
        Err(erro) => println!("  REPROVADO: {erro:?}"),
    }
    println!("  a ficha na versao atual nao passa pela cadeia: {}", stochology_schema::de_texto(&atual).is_ok());
}

fn rodar(cap: usize) {
    match cap {
        1 => cap01(),
        2 => cap02(),
        3 => cap03(),
        4 => cap04(),
        5 => cap05::rodar(),
        6 => cap06::rodar(),
        7 => cap07::rodar(),
        8 => cap08::rodar(),
        9 => cap09::rodar(),
        10 => cap10::rodar(),
        11 => cap11::rodar(),
        12 => cap12::rodar(),
        13 => cap13::rodar(),
        14 => cap14::rodar(),
        15 => cap15::rodar(),
        16 => cap16::rodar(),
        17 => cap17::rodar(),
        18 => cap18::rodar(),
        19 => cap19::rodar(),
        20 => cap20::rodar(),
        21 => cap21::rodar(),
        _ => println!("uso: labs --cap N (1..21) ou --todos"),
    }
}

fn cap01() {
    println!("== Capítulo 1 :: o alvo invisível ==");
    let n = 10_000;
    println!(
        "duas maquinas com a semente 1713 concordam em {} de {} sorteios: {}",
        n,
        n,
        cap01::duas_maquinas_concordam(1713, n)
    );
    println!(
        "sorteios coincidentes com a semente seguinte: {}",
        cap01::coincidencias_com_outra_semente(1713, n)
    );
    for semente in [1713u64, 1909, 2026] {
        println!(
            "semente {}: frequencia de caras em 100 mil jogadas = {:.4}",
            semente,
            cap01::frequencia_de_caras(semente, 100_000)
        );
    }
}

fn cap02() {
    println!("== Capítulo 2 :: a falácia mora na ponte ==");
    let falso_positivo = 1.0 / 1_000_000.0;
    for prevalencia in [0.01_f64, 1.0 / 1_000.0, 1.0 / 1_000_000.0] {
        let p = cap02::falacia_do_promotor(prevalencia, 1.0, falso_positivo);
        println!(
            "prevalencia {:.0e}: P(evidencia|inocente)={:.8}  P(inocente|evidencia)={:.4}  fator da troca={:.1}",
            prevalencia,
            p.verossimilhanca,
            p.posterior_de_inocencia,
            cap02::fator_da_troca(prevalencia, 1.0, falso_positivo)
        );
    }
}

fn cap03() {
    println!("== Capítulo 3 :: árvores, chances e decibans ==");
    let a = cap03::arvore(10_000, 0.01, 0.90, 0.09);
    println!(
        "mamografia: {} doentes, {} positivos doentes, {} saudaveis, {} falsos positivos, {} positivas",
        a.doentes, a.positivos_doentes, a.saudaveis, a.falsos_positivos, a.positivos
    );
    println!(
        "P(doente|positivo) = {:.4}  (uma em cada {:.1} positivas)",
        a.fracao_doentes_entre_positivos(),
        a.uma_em_cada()
    );
    println!(
        "segundo exame: independente = {:.1} dB, correlacionado (razao efetiva 4,2) = {:.2} dB",
        cap03::decibans(10.0),
        cap03::decibans(4.2)
    );
    println!(
        "posterior com dois positivos: independentes = {:.4}, correlacionados = {:.4}",
        cap03::posterior_por_chances(0.01, &[10.0, 10.0]),
        cap03::posterior_por_chances(0.01, &[10.0, 4.2])
    );
}

fn cap04() {
    println!("== Capítulo 4 :: a moeda cujo vício muda ==");
    let c = cap04::moeda_movel(2026, 6_400, 200, 0.99);
    println!(
        "vies sobe de {:.1} a {:.1} em {} passos, {} repeticoes, desconto {}",
        cap04::vies_em(0, c.passos),
        cap04::vies_em(c.passos, c.passos),
        c.passos,
        c.repeticoes,
        c.desconto
    );
    println!(
        "cobertura do intervalo de 95%: modelo fixo = {:.4}, modelo com esquecimento = {:.4}",
        c.cobertura_estatica, c.cobertura_descontada
    );
    println!(
        "largura do intervalo fixo no passo 100 = {:.4}, no passo {} = {:.4}",
        c.largura_estatica_100, c.passos, c.largura_estatica_final
    );
    println!(
        "largura do intervalo com esquecimento no passo {} = {:.4} (razao {:.1}x o fixo final)",
        c.passos,
        c.largura_descontada_final,
        c.largura_descontada_final / c.largura_estatica_final
    );
}
