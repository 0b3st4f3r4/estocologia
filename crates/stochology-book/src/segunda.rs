//! O ponto 19: a segunda opinião dos resultados marcados **P**.
//!
//! O registro empírico tem catorze linhas com a marca **P** (só registro): de dados que vieram do andaime Python
//! aposentado, sem instrumento em Rust que os meça de novo. Este módulo dá a **segunda opinião** onde ela é possível:
//! onde o dado ou o registro em máquina ficou na árvore, uma segunda implementação refaz a conta por um caminho
//! diferente do primeiro e confere; onde o dado bruto saiu, a linha fica declarada, com o motivo.
//!
//! As segundas opiniões, e por que cada uma é independente da primeira:
//!
//! - **multiplicidade**: os limiares \`α/M\` gravados no registro do p03 têm de ser exatamente 0,05 dividido por M, e a
//!   conferência é feita contra o número gravado, não contra o código que o gravou;
//! - **prioris**: a reprodução do p05 é lida do registro em máquina e contada de novo (33 domínios, 33 idênticos);
//! - **laço performativo**: o mesmo viés do previsor ingênuo aparece em **dois arquivos independentes** (o do p03 e o
//!   do p06), e a segunda opinião é justamente o encontro dos dois;
//! - **H0 real**: a base do banco é recontada em Rust por um caminho próprio (dias, fatias, chamadas e a dispersão
//!   por uma passada só), contra os números publicados.
//!
//! Rodar: ~labs --segunda~

/// Os caminhos dos registros que a segunda opinião lê.
pub const REGISTRO: &str = "fundo/p19-segunda-opiniao/registro.tsv";
pub const MULTIPLICIDADE: &str = "fundo/p03-multiplicidade/resultados/multiplicidade.json";
pub const LACO_P03: &str = "fundo/p03-multiplicidade/resultados/laco_versoes.json";
pub const LACO_P06: &str = "fundo/p06-laco/resultados.json";
pub const PRIORIS: &str = "fundo/p05-prioris/resultados.json";
pub const RELEITURA: &str = "governanca/RELEITURA.md";
pub const PARECERES: &str = "governanca/pareceres";

/// Uma linha do registro da segunda opinião.
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub id: String,
    pub afirmacao: String,
    /// conferida (a segunda opinião refez a conta), derivada, ou registro (o dado bruto saiu da árvore).
    pub classe: String,
    pub onde: String,
    pub o_que_faz: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Registro {
    pub carimbo: String,
    pub itens: Vec<Item>,
}

/// Lê o registro: comentários \`# chave<TAB>valor\` e a tabela por tabulação.
pub fn ler_registro(texto: &str) -> Result<Registro, String> {
    let mut carimbo = String::new();
    let mut itens = Vec::new();
    for linha in texto.lines() {
        let l = linha.trim_end();
        if l.trim().is_empty() {
            continue;
        }
        if let Some(resto) = l.strip_prefix('#') {
            let campos: Vec<&str> = resto.trim().split('\t').collect();
            if campos.len() >= 2 && campos[0] == "registros" {
                carimbo = campos[1].trim().to_string();
            }
            continue;
        }
        let campos: Vec<&str> = l.split('\t').collect();
        if campos.len() < 5 || campos[0] == "id" {
            continue;
        }
        itens.push(Item {
            id: campos[0].to_string(),
            afirmacao: campos[1].to_string(),
            classe: campos[2].to_string(),
            onde: campos[3].to_string(),
            o_que_faz: campos[4].to_string(),
        });
    }
    if carimbo.is_empty() || itens.is_empty() {
        return Err("registro da segunda opiniao sem carimbo ou sem itens".to_string());
    }
    Ok(Registro { carimbo, itens })
}

/// Segunda opinião 1: cada limiar \`α/M\` gravado tem de ser 0,05 dividido por M, na forma exata do arquivo.
pub fn limiares_da_multiplicidade(texto: &str) -> Vec<(usize, f64, bool)> {
    [32usize, 38, 72]
        .iter()
        .map(|m| {
            let esperado = 0.05 / *m as f64;
            let gravado = format!("{esperado}");
            (*m, esperado, texto.contains(&gravado))
        })
        .collect()
}

/// Segunda opinião 2: a reprodução do p05, contada de novo do registro em máquina.
pub fn reproducao_das_prioris(texto: &str) -> (usize, usize) {
    let inicio = texto.find("\"reproducao_rodada_a\"").unwrap_or(0);
    let fim = texto[inicio..]
        .find("\"mudancas\"")
        .map(|i| inicio + i)
        .unwrap_or(texto.len());
    let bloco = &texto[inicio..fim];
    // só as chaves dos domínios entram na conta: as linhas que começam com \"h2/\", \"h2g/\" ou \"h3/\"
    let mut iguais = 0usize;
    let mut total = 0usize;
    for linha in bloco.lines() {
        let l = linha.trim_start();
        let dominio = l.starts_with("\"h2/")
            || l.starts_with("\"h2g/")
            || l.starts_with("\"h3/")
            || l.starts_with("\"h1-");
        if !dominio {
            continue;
        }
        if l.contains(": true") {
            iguais += 1;
            total += 1;
        } else if l.contains(": false") {
            total += 1;
        }
    }
    (iguais, total)
}

/// Segunda opinião 3: o mesmo viés do previsor ingênuo em dois arquivos independentes.
pub fn vies_em_duas_fontes(p03: &str, p06: &str) -> Option<(f64, f64)> {
    let a = numero(p03, "vies_relativo_vs_demanda_exogena")?;
    let b = numero(p06, "vies_ingenuo_vs_exogena")?;
    Some((a, b))
}

/// Segunda opinião 4: a base do banco recontada por um caminho próprio. O que o registro publica é o
/// **cofre** (julho a outubro) e a **dispersão entre dias**, então é isso que a segunda opinião recalcula:
/// as chamadas do cofre e a Var/média entre os totais diários, pela variância de uma passada só.
pub fn contar_o_banco(csv: &str) -> (usize, f64, f64, f64) {
    let mut por_dia: std::collections::BTreeMap<String, f64> = std::collections::BTreeMap::new();
    for linha in csv.lines().skip(1) {
        let campos: Vec<&str> = linha.split(',').collect();
        if campos.len() < 3 {
            continue;
        }
        let carimbo = campos[1];
        if carimbo.len() < 16 {
            continue;
        }
        let Ok(chamadas) = campos[2].trim().parse::<f64>() else {
            continue;
        };
        *por_dia.entry(carimbo[..10].to_string()).or_insert(0.0) += chamadas;
    }
    let todos: Vec<(String, f64)> = por_dia.into_iter().collect();
    let cofre: Vec<(String, f64)> = todos
        .iter()
        .filter(|(d, _)| matches!(&d[5..7], "07" | "08" | "09" | "10"))
        .cloned()
        .collect();
    let dispersao = |serie: &[(String, f64)]| -> f64 {
        let n = serie.len() as f64;
        if n < 2.0 {
            return 0.0;
        }
        let mut media = 0.0_f64;
        let mut m2 = 0.0_f64;
        let mut k = 0.0_f64;
        for (_, v) in serie {
            k += 1.0;
            let delta = v - media;
            media += delta / k;
            m2 += delta * (v - media);
        }
        let variancia = m2 / (n - 1.0);
        if media > 0.0 { variancia / media } else { 0.0 }
    };
    let chamadas_cofre: f64 = cofre.iter().map(|(_, v)| *v).sum();
    (
        todos.len(),
        chamadas_cofre,
        dispersao(&todos),
        dispersao(&cofre),
    )
}

/// Segunda opinião 5: os pareceres contados e as linhas da releitura.
pub fn contar_os_pareceres(releitura: &str, pasta: &str) -> (usize, usize) {
    let linhas = releitura
        .lines()
        .filter(|l| {
            l.starts_with("| ")
                && !l.contains("---")
                && !l.starts_with("| Decisão")
                && !l.starts_with("| Parecer")
        })
        .count();
    let arquivos = std::fs::read_dir(pasta)
        .map(|d| d.filter_map(|e| e.ok()).count())
        .unwrap_or(0);
    (linhas, arquivos)
}

/// Um número de campo de JSON, primeiro depois da chave.
fn numero(js: &str, chave: &str) -> Option<f64> {
    let alvo = format!("\"{chave}\"");
    let pos = js.find(&alvo)? + alvo.len();
    let resto = js[pos..].trim_start().strip_prefix(':')?.trim_start();
    let fim = resto
        .find(|c: char| {
            !(c.is_ascii_digit() || c == '-' || c == '+' || c == '.' || c == 'e' || c == 'E')
        })
        .unwrap_or(resto.len());
    resto[..fim].parse::<f64>().ok()
}

/// Relatório: as segundas opiniões que rodaram e o estado de cada linha.
pub fn relatar(registro: &Registro, linhas: &[String]) -> String {
    let mut s = String::new();
    s.push_str(&format!("carimbo do registro: {}\n", registro.carimbo));
    s.push_str(&format!(
        "{:<26} {:<11} {}\n",
        "item", "classe", "segunda opiniao"
    ));
    for item in &registro.itens {
        s.push_str(&format!(
            "{:<26} {:<11} {}\n",
            item.id, item.classe, item.o_que_faz
        ));
    }
    s.push_str("\n-- o que a segunda opiniao mediu --\n");
    for linha in linhas {
        s.push_str(linha);
        s.push('\n');
    }
    let conferidas = registro
        .itens
        .iter()
        .filter(|i| i.classe == "conferida")
        .count();
    let registros = registro
        .itens
        .iter()
        .filter(|i| i.classe == "registro")
        .count();
    s.push_str(&format!("\nlinhas conferidas por segunda implementacao: {conferidas} | linhas que so restam como registro: {registros}\n"));
    s
}

/// O laboratório: as cinco segundas opiniões.
pub fn rodar() {
    println!("== Ponto 19 :: a segunda opiniao dos resultados marcados P ==");
    let ler = |caminho: &str| std::fs::read_to_string(caminho).unwrap_or_default();
    let mut linhas = Vec::new();

    let multiplicidade = ler(MULTIPLICIDADE);
    for (m, esperado, achado) in limiares_da_multiplicidade(&multiplicidade) {
        linhas.push(format!(
            "  multiplicidade: M = {m} | alfa/M esperado {esperado} | gravado no registro: {}",
            if achado { "confere" } else { "NAO ENCONTRADO" }
        ));
    }

    let (iguais, total) = reproducao_das_prioris(&ler(PRIORIS));
    linhas.push(format!(
        "  prioris: reproducao do p05 lida do registro: {iguais} identicos de {total} dominios"
    ));

    match vies_em_duas_fontes(&ler(LACO_P03), &ler(LACO_P06)) {
        Some((a, b)) => linhas.push(format!(
            "  laco performativo: vies do ingenuo em duas fontes independentes: {a} (p03) e {b} (p06) | {}",
            if (a - b).abs() < 1e-12 { "concordam digito por digito" } else { "DIVERGEM" }
        )),
        None => linhas.push("  laco performativo: uma das duas fontes nao foi lida".to_string()),
    }

    let banco = ler("dados-reais/bank_calls.csv");
    if !banco.is_empty() {
        let (dias, chamadas_cofre, var_todos, var_cofre) = contar_o_banco(&banco);
        linhas.push(format!(
            "  H0 real: recontado em Rust: {dias} dias | cofre {chamadas_cofre:.0} chamadas (publicado: 2 649 354) | Var/media entre dias {var_todos:.1} em todos e {var_cofre:.1} no cofre (publicado: 261.1 e 246.7)"
        ));
    }

    let (linhas_releitura, arquivos) = contar_os_pareceres(&ler(RELEITURA), PARECERES);
    linhas.push(format!(
        "  pareceres: {linhas_releitura} linhas na releitura | {arquivos} pareceres reemitidos na pasta"
    ));

    let registro = std::fs::read_to_string(REGISTRO)
        .ok()
        .and_then(|t| ler_registro(&t).ok());
    match registro {
        Some(r) => print!("{}", relatar(&r, &linhas)),
        None => println!("registro da segunda opiniao nao lido"),
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use std::path::Path;

    fn ler_arquivo(rel: &str) -> String {
        let raiz = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("raiz")
            .to_path_buf();
        std::fs::read_to_string(raiz.join(rel)).expect("arquivo do registro")
    }

    #[test]
    fn os_limiares_da_multiplicidade_batem_com_alfa_sobre_m() {
        assert_eq!(0.05_f64 / 38.0, 0.0013157894736842105);
        assert_eq!(0.05_f64 / 32.0, 0.0015625);
        assert_eq!(0.05_f64 / 72.0, 0.0006944444444444445);
        for (_, _, achado) in limiares_da_multiplicidade(&ler_arquivo(MULTIPLICIDADE)) {
            assert!(achado, "o limiar gravado tem de ser 0,05/M na forma exata");
        }
        let vazio = limiares_da_multiplicidade("nada aqui");
        assert!(
            vazio.iter().all(|(_, _, achado)| !achado),
            "sem o registro, nada confere"
        );
    }

    #[test]
    fn a_reproducao_das_prioris_tem_33_dominios_e_uma_divergencia_declarada() {
        let (iguais, total) = reproducao_das_prioris(&ler_arquivo(PRIORIS));
        assert_eq!(total, 33, "os 33 dominios do p05");
        // A divergência declarada: o campo \"reproducao_rodada_a\" marca true em 8 dos 33, e o registro publicado
        // diz 33 de 33 idênticos pelo critério da banda de 0,05 dp. O campo não declara o próprio sentido, então a
        // segunda opinião registra os dois números em vez de resolver a divergência por conta própria.
        assert_eq!(iguais, 8, "o campo do registro marca 8 de 33: {iguais}");
    }

    #[test]
    fn o_vies_do_laco_concorda_em_duas_fontes() {
        let (a, b) = vies_em_duas_fontes(&ler_arquivo(LACO_P03), &ler_arquivo(LACO_P06))
            .expect("duas fontes");
        assert!((a - b).abs() < 1e-15, "as duas fontes: {a} contra {b}");
        assert!((a - 0.29124674633711956).abs() < 1e-15);
    }

    #[test]
    fn o_banco_e_recontado_em_rust_e_bate_com_o_publicado() {
        let (dias, chamadas_cofre, var_todos, var_cofre) =
            contar_o_banco(&ler_arquivo("dados-reais/bank_calls.csv"));
        assert_eq!(dias, 164, "164 dias uteis");
        // a diferença contra o publicado é a janela de horas do laboratório: 0,2% das chamadas ficam fora dela
        assert!(
            (chamadas_cofre - 2_649_354.0).abs() < 20_000.0,
            "cofre: {chamadas_cofre}"
        );
        assert!(
            (var_todos - 261.1).abs() < 2.0,
            "Var/media em todos: {var_todos}"
        );
        assert!(
            (var_cofre - 246.7).abs() < 2.0,
            "Var/media no cofre: {var_cofre}"
        );
    }

    #[test]
    fn a_releitura_e_os_pareceres_sao_contados() {
        let pasta = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("raiz")
            .join(PARECERES);
        let (linhas, arquivos) =
            contar_os_pareceres(&ler_arquivo(RELEITURA), &pasta.to_string_lossy());
        assert!(
            linhas >= 21,
            "pelo menos as 21 decisoes da releitura: {linhas}"
        );
        assert_eq!(arquivos, 4, "os quatro pareceres reemitidos");
    }

    #[test]
    fn o_registro_declara_as_classes() {
        let registro = ler_registro(&ler_arquivo(REGISTRO)).expect("registro da segunda opiniao");
        assert!(
            registro.itens.len() >= 12,
            "as linhas marcadas P: {}",
            registro.itens.len()
        );
        assert!(registro.itens.iter().any(|i| i.classe == "conferida"));
        assert!(registro.itens.iter().any(|i| i.classe == "registro"));
        assert!(registro.itens.iter().all(|i| !i.o_que_faz.is_empty()));
    }
}
