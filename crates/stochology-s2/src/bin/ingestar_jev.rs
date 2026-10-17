//! O ingestor do item 13: prepara o artefato dos pesos e o conjunto rotulado do Brier.
//!
//! Fora da biblioteca de producao de proposito: e ferramenta de operador, nao peca do aparato. Recebe os
//! caminhos explicitamente, nao embute numero nem nome de arquivo de dado, e valida antes de persistir: cada
//! fonte passa por checagem de campo, o resultado e escrito num diretorio de ensaio e lido de volta pelos mesmos
//! portoes que a ponte usa (hash declarado, formato, id sequencial, intervalo aberto da probabilidade) e so
//! entao vai para o destino. Se qualquer coisa falhar, o destino nao e tocado.
//!
//! Uso:
//!   ingestar-jev --conferir <fonte_inferencias.tsv> <fonte_desfechos.tsv>
//!   ingestar-jev <dir_pesos> <fonte_inferencias.tsv> <dir_calibracao> <fonte_desfechos.tsv>
//!   ingestar-jev --registrar <medida.txt> <rodadas.tsv> --data AAAA-MM-DD
//!
//! O modo --conferir e a fase 1 do fechamento: valida as duas fontes, roda o ensaio e nao escreve em destino
//! nenhum. Serve para o operador saber, antes de qualquer persistencia, se o conjunto tem volume, intervalo e
//! procedencia para o criterio declarado.

use std::env;
use std::path::{Path, PathBuf};
use std::process;

use stochology_s2::ponte::validar_saida;
use stochology_s2::{calibracao, runner};

fn uso_e_abortar(programa: &str) -> ! {
    eprintln!("Uso:");
    eprintln!("  {programa} --conferir <fonte_inferencias.tsv> <fonte_desfechos.tsv>");
    eprintln!("  {programa} --registrar <medida.txt> <rodadas.tsv> --data AAAA-MM-DD");
    eprintln!("  {programa} <dir_pesos> <fonte_inferencias.tsv> <dir_calibracao> <fonte_desfechos.tsv>");
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
        // A resposta pode ter mais de uma linha (o documento canonico da ficha tem). Na fonte, que e
        // linha a linha, a quebra vem escapada como a sequencia de dois caracteres barra-n, e o ingestor
        // desfaz o escape aqui. Sem escape nao ha duas linhas ambiguas.
        // As duas colunas vem escapadas da fonte. O pedido tambem: se so a resposta for desescapada, a chave
        // do artefato (sha do texto do pedido) deixa de casar com o texto guardado, e a ponte abstem em 100%
        // dos casos sem dizer por que. Foi assim que a primeira rodada com o modelo real saiu 32 de 32 validas
        // na conferencia e 100% de abstencao no instrumento.
        pares.push((
            campos[0].replace("\\n", "\n"),
            campos[1].replace("\\n", "\n"),
        ));
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

/// A fase 1 do fechamento: confere as fontes e ensaia, sem tocar em destino nenhum.
fn conferir(fonte_inferencias: &Path, fonte_desfechos: &Path) {
    println!("== Conferencia das fontes (fase 1), sem escrita em destino ==");
    let inferencias = ler_inferencias(fonte_inferencias);
    println!("  inferencias: {} pares pedido/resposta, com os dois campos", inferencias.len());
    let desfechos = ler_desfechos(fonte_desfechos);
    println!("  desfechos: {} pares, com p em (0, 1), desfecho 0 ou 1 e fonte declarada", desfechos.len());
    let ensaio_pesos = dir_de_ensaio("conf-pesos");
    let ensaio_cal = dir_de_ensaio("conf-cal");
    runner::escrever_artefato(&ensaio_pesos, &inferencias)
        .unwrap_or_else(|e| morrer(format!("Falha ao escrever o artefato de ensaio: {e}")));
    let artefato = runner::RunnerLocal::carregar(&ensaio_pesos.to_string_lossy())
        .unwrap_or_else(|e| morrer(format!("O artefato de ensaio nao passa no portao: {e:?}")));
    calibracao::escrever_desfechos(&ensaio_cal, &desfechos)
        .unwrap_or_else(|e| morrer(format!("Falha ao escrever o conjunto de ensaio: {e}")));
    let cal = calibracao::carregar(&ensaio_cal.to_string_lossy(), 0, calibracao::AMOSTRA_MINIMA)
        .unwrap_or_else(|e| morrer(format!("O conjunto de ensaio nao passa no portao: {e:?}")));
    std::fs::remove_dir_all(&ensaio_pesos).ok();
    std::fs::remove_dir_all(&ensaio_cal).ok();
    println!("  artefato de ensaio: {} respostas, hash {}", artefato.respostas(), artefato.sha256());
    // O esquema e cobrado aqui, pelo parser de verdade: o que nao passa vira abstencao na ponte, e e melhor
    // descobrir na fase 1 do que na leitura do instrumento.
    let mut fora = 0usize;
    let mut primeiro_erro = String::new();
    for (i, (_, resposta)) in inferencias.iter().enumerate() {
        if let Err(recusa) = validar_saida(resposta) {
            fora += 1;
            if primeiro_erro.is_empty() {
                primeiro_erro = format!("caso {}: {recusa:?}", i + 1);
            }
        }
    }
    println!(
        "  esquema da ficha: {} de {} respostas validas",
        inferencias.len() - fora,
        inferencias.len()
    );
    if fora > 0 {
        eprintln!("  {fora} resposta(s) fora do esquema; a primeira: {primeiro_erro}");
    }
    println!("  conjunto de ensaio: {}", cal.leitura());
    let volume = desfechos.len() >= calibracao::AMOSTRA_MINIMA;
    println!(
        "  criterio de volume (N >= {}): {}",
        calibracao::AMOSTRA_MINIMA,
        if volume { "cumprido" } else { "NAO cumprido, o escore nao sera publicado" }
    );
    println!("  nenhum destino foi tocado.");
    if !volume || fora > 0 {
        process::exit(1);
    }
}

/// A fase 4 mecanica: le a ficha da medida e grava a linha da rodada, ou recusa com o motivo.
fn registrar(ficha: &Path, rodadas: &Path, data: &str) {
    let texto = std::fs::read_to_string(ficha)
        .unwrap_or_else(|e| morrer(format!("Erro ao abrir a medida {}: {e}", ficha.display())));
    let medida = stochology_s2::ponte::de_ficha(&texto)
        .unwrap_or_else(|e| morrer(format!("Medida ilegivel: {e}")));
    let linha = match stochology_s2::ponte::linha_de_registro(&medida, data) {
        Ok(l) => l,
        Err(motivo) => morrer(format!("Registro recusado: {motivo}")),
    };
    let cabecalho = "data\thash_pesos\tpedidos\tabstencoes\tn_pares\ttokens_por_chamada\tms_por_chamada\tbrier\tece\n";
    let existente = std::fs::read_to_string(rodadas).unwrap_or_default();
    let mut destino = existente.clone();
    if destino.is_empty() {
        destino.push_str(cabecalho);
    }
    if destino.contains(&linha) {
        morrer(String::from("esta rodada ja esta no registro: a linha existe"));
    }
    destino.push_str(&linha);
    destino.push('\n');
    std::fs::write(rodadas, destino.as_bytes())
        .unwrap_or_else(|e| morrer(format!("Falha ao gravar {}: {e}", rodadas.display())));
    println!("registro: {linha}");
    println!("  gravado em {}", rodadas.display());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 6 && args[1] == "--registrar" {
        registrar(Path::new(&args[2]), Path::new(&args[3]), &args[5]);
        return;
    }
    if args.len() == 4 && args[1] == "--conferir" {
        conferir(Path::new(&args[2]), Path::new(&args[3]));
        return;
    }
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
