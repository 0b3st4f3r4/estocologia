//! Os testes do ingestor: o que ele grava, o que ele recusa e o que ele nao toca quando recusa.

use std::path::PathBuf;
use std::process::Command;

fn temp(nome: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("ingestar-jev-teste-{nome}-{}", std::process::id()));
    std::fs::create_dir_all(&d).expect("cria o diretorio de teste");
    d
}

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_ingestar-jev")
}

fn conferir_lacre(dir: &PathBuf, arquivo: &str) {
    let manifesto = std::fs::read_to_string(dir.join("SHA256")).expect("le o manifesto");
    let declarado = manifesto.split_whitespace().next().expect("hash no manifesto").to_lowercase();
    let dados = std::fs::read(dir.join(arquivo)).expect("le o arquivo");
    assert_eq!(declarado, stochology_audit::sha256_hex(&dados), "o manifesto confere com o arquivo");
}

#[test]
fn o_ingestor_grava_o_artefato_e_o_conjunto_com_os_lacres() {
    let d = temp("ok");
    let inf = d.join("inferencias.tsv");
    std::fs::write(&inf, "pergunta A\tresposta A\npergunta B\tresposta B\n").expect("escreve");
    let desf = d.join("desfechos.tsv");
    std::fs::write(&desf, "C-001\t0.7000\t1\tregistro\nC-002\t0.2000\t0\tregistro\n").expect("escreve");
    let pesos = d.join("pesos");
    let cal = d.join("calibracao");
    let saida = Command::new(bin())
        .arg(&pesos)
        .arg(&inf)
        .arg(&cal)
        .arg(&desf)
        .output()
        .expect("roda o ingestor");
    assert!(
        saida.status.success(),
        "o ingestor devia passar: {}",
        String::from_utf8_lossy(&saida.stderr)
    );
    conferir_lacre(&pesos, "modelo.congelado");
    conferir_lacre(&cal, "desfechos.tsv");
    let texto = std::fs::read_to_string(cal.join("desfechos.tsv")).expect("le o tsv");
    assert!(texto.starts_with("id\tcaso_id\tprobabilidade_declarada\t"), "cabecalho canonico");
    assert_eq!(texto.lines().count(), 3, "cabecalho e dois casos");
    std::fs::remove_dir_all(&d).ok();
}

#[test]
fn probabilidade_no_extremo_aborta_e_nao_escreve_nada() {
    let d = temp("extremo");
    let inf = d.join("inferencias.tsv");
    std::fs::write(&inf, "pergunta A\tresposta A\n").expect("escreve");
    let desf = d.join("desfechos.tsv");
    std::fs::write(&desf, "C-001\t1.0000\t1\tregistro\n").expect("escreve");
    let pesos = d.join("pesos");
    let cal = d.join("calibracao");
    let saida = Command::new(bin())
        .arg(&pesos)
        .arg(&inf)
        .arg(&cal)
        .arg(&desf)
        .output()
        .expect("roda o ingestor");
    assert!(!saida.status.success(), "probabilidade em 1.0 tinha de abortar");
    let erro = String::from_utf8_lossy(&saida.stderr);
    assert!(erro.contains("extremo dogmatico"), "{erro}");
    assert!(!pesos.exists(), "nada e escrito no destino dos pesos");
    assert!(!cal.exists(), "nada e escrito no destino da calibracao");
    std::fs::remove_dir_all(&d).ok();
}

#[test]
fn sem_argumentos_o_ingestor_imprime_o_uso_e_sai_com_1() {
    let saida = Command::new(bin()).output().expect("roda o ingestor");
    assert_eq!(saida.status.code(), Some(1), "uso errado sai com 1");
    let erro = String::from_utf8_lossy(&saida.stderr);
    assert!(erro.contains("Uso:"), "{erro}");
    assert!(erro.contains("dir_pesos"), "{erro}");
}

#[test]
fn o_modo_conferir_valida_sem_tocar_em_destino() {
    let d = temp("conferir");
    let inf = d.join("inferencias.tsv");
    std::fs::write(&inf, "pergunta A\tresposta A\npergunta B\tresposta B\n").expect("escreve");
    let desf = d.join("desfechos.tsv");
    std::fs::write(&desf, "C-001\t0.4000\t1\tregistro\n").expect("escreve");
    let saida = Command::new(bin())
        .arg("--conferir")
        .arg(&inf)
        .arg(&desf)
        .output()
        .expect("roda o ingestor");
    let texto = String::from_utf8_lossy(&saida.stdout);
    assert!(texto.contains("nenhum destino foi tocado"), "{texto}");
    assert!(texto.contains("AmostraInsuficiente"), "com um par, o escore nao e publicado: {texto}");
    assert_eq!(saida.status.code(), Some(1), "volume abaixo do minimo sai com 1");
    assert!(!d.join("pesos").exists(), "o modo de conferencia nao cria destino");
    assert!(!d.join("calibracao").exists(), "o modo de conferencia nao cria destino");
    std::fs::remove_dir_all(&d).ok();
}

#[test]
fn o_modo_conferir_passa_com_trinta_pares() {
    let d = temp("conferir30");
    let inf = d.join("inferencias.tsv");
    let mut linhas = String::new();
    for n in 1..=30 {
        linhas.push_str(&format!("pergunta {n}\tresposta {n}\n"));
    }
    std::fs::write(&inf, linhas.as_bytes()).expect("escreve");
    let desf = d.join("desfechos.tsv");
    let mut dlinhas = String::new();
    for n in 1..=30 {
        let p = 0.05 + 0.9 * (n as f64 / 31.0);
        dlinhas.push_str(&format!("C-{n:03}\t{p:.4}\t{}\tregistro\n", n % 2));
    }
    std::fs::write(&desf, dlinhas.as_bytes()).expect("escreve");
    let saida = Command::new(bin())
        .arg("--conferir")
        .arg(&inf)
        .arg(&desf)
        .output()
        .expect("roda o ingestor");
    let texto = String::from_utf8_lossy(&saida.stdout);
    assert!(saida.status.success(), "{texto}{}", String::from_utf8_lossy(&saida.stderr));
    assert!(texto.contains("criterio de volume (N >= 30): cumprido"), "{texto}");
    std::fs::remove_dir_all(&d).ok();
}
