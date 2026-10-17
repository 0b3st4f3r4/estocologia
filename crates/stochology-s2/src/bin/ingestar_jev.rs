//! O ingestor do item 13: prepara o artefato dos pesos e o conjunto rotulado do Brier.
//!
//! Fora da biblioteca de producao de proposito: e ferramenta de operador, nao peca do aparato. Recebe os
//! caminhos explicitamente, nao embute numero nem nome de arquivo de dado, e valida antes de persistir: cada
//! fonte passa por checagem de campo, o resultado e escrito num diretorio de ensaio e lido de volta pelos mesmos
//! portoes que a ponte usa (hash declarado, formato, id sequencial, intervalo aberto da probabilidade) e so
//! entao vai para o destino. Se qualquer coisa falhar, o destino nao e tocado.
//!
//! Uso: ingestar-jev <dir_pesos> <fonte_inferencias.tsv> <dir_calibracao> <fonte_desfechos.tsv>

use std::env;
use std::path::{Path, PathBuf};
use std::process;

use stochology_s2::{calibracao, runner};

fn uso_e_abortar(programa: &str) -> ! {
    eprintln!("Uso: {programa} <dir_pesos> <fonte_inferencias.tsv> <dir_calibracao> <fonte_desfechos.tsv>");
    eprintln!();
    eprintln!("Argumentos:");
    eprintln!("  dir_pesos                destino do artefato (ex.: fundo/p20-jev/pesos)");
    eprintln!("  fonte_inferencias.tsv    linhas pedido-TAB-resposta, uma por caso congelado");
    eprintln!("  dir_calibracao           destino do conjunto rotulado (ex.: fundo/p20-jev/calibracao)");
    eprintln!("  fonte_desfechos.tsv      linhas caso_id-TAB-prob-TAB-desfecho-TAB-fonte");
    eprintln!();
    eprintln!("Nada e escrito no destino antes de o ensaio passar nos portoes da ponte.");
    process::exit(1);
}

fn morrer(mensagem: String) -> ! {
    eprintln!("{mensagem}");
    process::exit(1);
}

/// Le as inferencias congeladas, com dois campos e nenhum vazio.
fn ler_inferencias(caminho: &Path) -> Vec<(String, String)> {
    let conteudo = std::fs::read_to_string(caminho).unwrap_or_else(|e| {
        morrer(format!("Erro ao abrir fonte de inferencias {}: {e}", caminho.display()))
    });
    let mut pares = Vec::new();
    for (i, linha) in conteudo.lines().enumerate() {
        let l = linha.trim_end();
        if l.is_empty() || l.starts_with('#') {
            continue;
        }
        let campos: Vec<&str> = l.split('\t').collect();
        if campos.len() != 2 {
            morrer(format!(
                "Erro na linha {} de {}: esperava dois campos separados por TAB, achei {}",
                i + 1,
                caminho.display(),
                campos.len()
            ));
        }
        if campos[0].trim().is_empty() || campos[1].trim().is_empty() {
            morrer(format!("Erro na linha {} de {}: campo vazio", i + 1, caminho.display()));
        }
        pares.push((campos[0].to_string(), campos[1].to_string()));
    }
    if pares.is_empty() {
        morrer(format!("{} nao tem nenhuma inferencia", caminho.display()));
    }
    pares
}

/// Le os desfechos, com as regras da frente aplicadas antes de qualquer escrita.
fn ler_desfechos(caminho: &Path) -> Vec<(String, f64, bool, String)> {
    let conteudo = std::fs::read_to_string(caminho).unwrap_or_else(|e| {
        morrer(format!("Erro ao abrir fonte de desfechos {}: {e}", caminho.display()))
    });
    let mut pares = Vec::new();
    for (i, linha) in conteudo.lines().enumerate() {
        let l = linha.trim_end();
        if l.is_empty() || l.starts_with('#') || l.starts_with("caso_id\t") {
            continue;
        }
        let campos: Vec<&str> = l.split('\t').collect();
        if campos.len() != 4 {
            morrer(format!(
                "Erro na linha {} de {}: esperava quatro campos separados por TAB, achei {}",
                i + 1,
                caminho.display(),
                campos.len()
            ));
        }
        if campos[0].trim().is_empty() || campos[3].trim().is_empty() {
            morrer(format!("Erro na linha {} de {}: caso_id ou fonte vazio", i + 1, caminho.display()));
        }
        let prob: f64 = campos[1].trim().parse().unwrap_or_else(|_| {
            morrer(format!("Linha {}: probabilidade invalida '{}'", i + 1, campos[1]))
        });
        if !(0.0 < prob && prob < 1.0) {
            morrer(format!(
                "Linha {}: probabilidade {prob} fora do intervalo aberto (0, 1); extremo dogmatico recusado",
                i + 1
            ));
        }
        let desfecho = match campos[2].trim() {
            "0" => false,
            "1" => true,
            outro => morrer(format!("Linha {}: desfecho '{}' nao e 0 nem 1", i + 1, outro)),
        };
        pares.push((campos[0].trim().to_string(), prob, desfecho, campos[3].trim().to_string()));
    }
    if pares.is_empty() {
        morrer(format!("{} nao tem nenhum desfecho", caminho.display()));
    }
    pares
}

fn dir_de_ensaio(rotulo: &str) -> PathBuf {
    let d = env::temp_dir().join(format!("ingestar-jev-{rotulo}-{}", process::id()));
    std::fs::create_dir_all(&d).ok();
    d
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        uso_e_abortar(&args[0]);
    }
    let dir_pesos = Path::new(&args[1]);
    let fonte_inferencias = Path::new(&args[2]);
    let dir_calibracao = Path::new(&args[3]);
    let fonte_desfechos = Path::new(&args[4]);

    println!("== Ingestao de artefatos Jev-LLM (item 13) ==");

    println!("\n[1/3] Lendo inferencias de {}...", fonte_inferencias.display());
    let inferencias = ler_inferencias(fonte_inferencias);
    println!("  {} pares pedido/resposta, todos com os dois campos", inferencias.len());

    println!("\n[2/3] Lendo desfechos de {}...", fonte_desfechos.display());
    let desfechos = ler_desfechos(fonte_desfechos);
    println!(
        "  {} pares rotulados, com probabilidade em (0, 1), desfecho 0 ou 1 e fonte declarada",
        desfechos.len()
    );

    println!("\n[3/3] Ensaio antes de persistir: os dois arquivos vao a um diretorio temporario e voltam pelos portoes");
    let ensaio_pesos = dir_de_ensaio("pesos");
    let ensaio_cal = dir_de_ensaio("calibracao");
    runner::escrever_artefato(&ensaio_pesos, &inferencias)
        .unwrap_or_else(|e| morrer(format!("Falha ao escrever o artefato de ensaio: {e}")));
    let artefato = runner::RunnerLocal::carregar(&ensaio_pesos.to_string_lossy())
        .unwrap_or_else(|e| morrer(format!("O artefato de ensaio nao passa no portao: {e:?}")));
    println!(
        "  artefato: {} respostas congeladas, hash verificado {}",
        artefato.respostas(),
        artefato.sha256()
    );
    calibracao::escrever_desfechos(&ensaio_cal, &desfechos)
        .unwrap_or_else(|e| morrer(format!("Falha ao escrever o conjunto de ensaio: {e}")));
    let conferido = calibracao::carregar(&ensaio_cal.to_string_lossy(), 0, calibracao::AMOSTRA_MINIMA)
        .unwrap_or_else(|e| morrer(format!("O conjunto de ensaio nao passa no portao: {e:?}")));
    println!("  conjunto rotulado: {}", conferido.leitura());
    std::fs::remove_dir_all(&ensaio_pesos).ok();
    std::fs::remove_dir_all(&ensaio_cal).ok();

    println!("\nPersistindo nos destinos declarados...");
    let hash_pesos = runner::escrever_artefato(dir_pesos, &inferencias)
        .unwrap_or_else(|e| morrer(format!("Falha em runner::escrever_artefato: {e}")));
    println!("  {} -> modelo.congelado + SHA256 ({hash_pesos})", dir_pesos.display());
    let hash_cal = calibracao::escrever_desfechos(dir_calibracao, &desfechos)
        .unwrap_or_else(|e| morrer(format!("Falha em calibracao::escrever_desfechos: {e}")));
    println!("  {} -> desfechos.tsv + SHA256 ({hash_cal})", dir_calibracao.display());

    println!("\nPronto. Conferir os lacres e rodar o instrumento:");
    println!("  (cd {} && sha256sum -c SHA256)", dir_pesos.display());
    println!("  (cd {} && sha256sum -c SHA256)", dir_calibracao.display());
    println!("  cargo run --release --bin labs -- --jev");
}
