//! Gera uma rodada sintetica de 30 pares, para atestar o caminho inteiro do item 13.
//!
//! Este exemplo **nao** e o modelo: e um gerador de fixture. Ele escreve as duas fontes que o ingestor le
//! (pedido/resposta e caso/probabilidade/desfecho/fonte) com regras deterministicas, sem sorteio e sem numero
//! escondido no codigo:
//!
//! - pedido de cada caso: o mesmo texto que a ponte manda, via ponte::pedido;
//! - resposta: o documento canonico da ficha do modelo de ensaio, que passa no esquema;
//! - probabilidade: 0.05 + 0.90 * (((n * 7) % 19) / 18), sempre no intervalo aberto (0, 1);
//! - desfecho: 1 quando (n * 13) % 5 < 3, e 0 nos outros dois quintos;
//! - fonte: a etiqueta fixa rodada-sintetica, para o registro nao confundir com desfecho observado.
//!
//! Uso: cargo run --release --example rodada_sintetica -- <diretorio_de_saida>

use std::path::PathBuf;

use stochology_s2::ponte::{ModeloCongelado, ModeloDeEnsaio, pedido};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Uso: {} <diretorio_de_saida>", args[0]);
        std::process::exit(1);
    }
    let dir = PathBuf::from(&args[1]);
    std::fs::create_dir_all(&dir).expect("cria o diretorio de saida");

    let mut ensaio = ModeloDeEnsaio::novo();
    let mut inferencias = String::new();
    let mut desfechos = String::from("caso_id\tprobabilidade\tdesfecho\tfonte\n");
    let mut positivos = 0usize;

    for n in 1..=30usize {
        let pergunta = format!("caso {n}");
        let texto_do_pedido = pedido(&pergunta);
        let resposta = ensaio.decodificar(&texto_do_pedido);
        inferencias.push_str(&format!(
            "{texto_do_pedido}\t{}\n",
            resposta.replace('\n', "\\n")
        ));

        let p = 0.05 + 0.90 * (((n * 7) % 19) as f64 / 18.0);
        let desfecho = usize::from((n * 13) % 5 < 3);
        positivos += desfecho;
        desfechos.push_str(&format!("C-{n:03}\t{p:.4}\t{desfecho}\trodada-sintetica\n"));
    }

    let arq_inf = dir.join("inferencias_congeladas.tsv");
    let arq_desf = dir.join("desfechos_auditados.tsv");
    std::fs::write(&arq_inf, inferencias.as_bytes()).expect("escreve as inferencias");
    std::fs::write(&arq_desf, desfechos.as_bytes()).expect("escreve os desfechos");

    println!("rodada sintetica escrita em {}", dir.display());
    println!("  30 inferencias: {arq_inf:?}");
    println!(
        "  30 desfechos: {arq_desf:?} ({positivos} com desfecho 1, {} com 0)",
        30 - positivos
    );
    println!(
        "  regras: p = 0.05 + 0.90 * (((n * 7) % 19) / 18); desfecho = 1 quando (n * 13) % 5 < 3"
    );
}
