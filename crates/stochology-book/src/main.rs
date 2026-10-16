//! Instrumento dos laboratórios do livro.
//!
//! Uso: ~cargo run --release --bin labs -- --cap 4~
//!      ~cargo run --release --bin labs -- --todos~

use stochology_book::{banco, fila};
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
    let mut fila_flag = false;
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
            "--fila" => fila_flag = true,
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
    if fila_flag {
        fila::rodar();
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
