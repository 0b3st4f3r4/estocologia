//! A cadeia do H3, re-executada até onde a árvore deixa.
//!
//! O registro empírico traz quatro linhas do H3, três com a marca **P** (só registro) e uma com **D**
//! (divergência declarada): os seis fluxos sob relógio estrito com canário, o cofre de Rhode Island, o laço
//! performativo medido (Goodhart) e o hipergrafo do H3. Este módulo percorre as quatro e, para cada uma, diz
//! **o que dá para refazer** e **o que só resta declarar** — porque o dado bruto de três delas saiu da árvore
//! com o andaime Python (`DECLARACAO_REMOVIDO_NAO_SIGNIFICANTE.md`).
//!
//! Três classes, declaradas em `fundo/p14-h3/DECLARACAO.md`:
//!
//! - **medida**: o dado bruto está na árvore e o instrumento mede de novo;
//! - **derivada**: o número está num registro versionado e o instrumento re-deriva a aritmética e o veredito;
//! - **registro**: o dado bruto não está na árvore; fica a declaração do que se mediu e de por que não se refaz.
//!
//! A única linha derivável hoje é o laço: `fundo/p06-laco/resultados.json` está na árvore, com a estimativa
//! 2SLS, o intervalo por bootstrap de dias e o F do primeiro estágio. Contra ela o instrumento refaz a conta do
//! critério declarado no pré-registro da frente — sinal, ordem de grandeza, compatibilidade estrita e força do
//! instrumento — e **sabe reprovar**: um registro em que a razão caia dentro da banda declarada muda o veredito,
//! e um registro em que o campo da razão não bata com a conta é reprovado.
//!
//! Rodar: ~labs --h3~

/// O registro em máquina das quatro linhas da cadeia.
pub const REGISTRO: &str = "fundo/p14-h3/registro.tsv";
/// O registro versionado do laço performativo medido (ponto de fundo 6).
pub const RESULTADOS_DO_LACO: &str = "fundo/p06-laco/resultados.json";
/// A correção grosseira feita depois de ver o resultado.
pub const POSTHOC_DO_LACO: &str = "fundo/p06-laco/posthoc.json";

/// O que se pode fazer com uma linha da cadeia.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Classe {
    /// O dado bruto está na árvore: o instrumento mede de novo.
    Medida,
    /// O número está num registro versionado: o instrumento re-deriva a conta e o veredito.
    Derivada,
    /// O dado bruto não está na árvore: fica a declaração.
    Registro,
}

impl Classe {
    pub fn da_linha(texto: &str) -> Option<Classe> {
        match texto {
            "medida" => Some(Classe::Medida),
            "derivada" => Some(Classe::Derivada),
            "registro" => Some(Classe::Registro),
            _ => None,
        }
    }
    pub fn nome(&self) -> &'static str {
        match self {
            Classe::Medida => "medida",
            Classe::Derivada => "derivada",
            Classe::Registro => "registro",
        }
    }
}

/// Uma linha da cadeia, com o que ela afirma e o que dá para refazer.
#[derive(Debug, Clone, PartialEq)]
pub struct Linha {
    pub linha: String,
    pub afirmacao: String,
    pub classe: Classe,
    /// A marca do registro empírico: R, D, P ou V (o veredito re-derivado).
    pub marca: String,
    /// O arquivo versionado que sustenta a re-derivação ("-" quando não há).
    pub registro: String,
    pub numeros: String,
    pub comando: String,
}

/// O registro da cadeia, com o carimbo.
#[derive(Debug, Clone, PartialEq)]
pub struct Cadeia {
    pub carimbo: String,
    pub linhas: Vec<Linha>,
}

/// O critério declarado antes de rodar, lido do registro em máquina (a fonte está no pré-registro da frente).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CriterioDoLaco {
    pub r_declarado: f64,
    pub razao_minima: f64,
    pub razao_maxima: f64,
    pub f_minimo: f64,
}

/// O que a re-derivação do laço devolve.
#[derive(Debug, Clone, PartialEq)]
pub struct VereditoDoLaco {
    pub r_iv: f64,
    pub ic90: (f64, f64),
    pub f_primeiro_estagio: f64,
    /// A razão recalculada aqui: r declarado / r medido.
    pub razao: f64,
    /// A razão que o próprio registro traz, para conferência.
    pub razao_no_registro: f64,
    pub sinal_confere: bool,
    pub ordem_confere: bool,
    pub estrito: bool,
    /// O campo de contenção que o registro traz, para conferência.
    pub contencao_no_registro: bool,
    pub vies_ingenuo: f64,
    pub veredito: String,
    pub divergencias: Vec<String>,
}

/// Lê o registro da cadeia: comentários `# chave<TAB>valor` e a tabela por tabulação.
pub fn ler_cadeia(texto: &str) -> Result<Cadeia, String> {
    let mut carimbo = String::new();
    let mut linhas = Vec::new();
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
        if campos.len() < 7 || campos[0] == "linha" {
            continue;
        }
        let classe = Classe::da_linha(campos[2]).ok_or_else(|| format!("classe desconhecida: {}", campos[2]))?;
        linhas.push(Linha {
            linha: campos[0].to_string(),
            afirmacao: campos[1].to_string(),
            classe,
            marca: campos[3].to_string(),
            registro: campos[4].to_string(),
            numeros: campos[5].to_string(),
            comando: campos[6].to_string(),
        });
    }
    if carimbo.is_empty() || linhas.is_empty() {
        return Err("registro da cadeia sem carimbo ou sem linhas".to_string());
    }
    Ok(Cadeia { carimbo, linhas })
}

/// Lê o critério do laço, que vive no registro em máquina com a fonte declarada.
pub fn ler_criterio(texto: &str) -> Result<CriterioDoLaco, String> {
    let mut r_declarado = None;
    let mut razao_minima = None;
    let mut razao_maxima = None;
    let mut f_minimo = None;
    for linha in texto.lines() {
        let l = linha.trim_end();
        let Some(resto) = l.strip_prefix('#') else { continue };
        let campos: Vec<&str> = resto.trim().split('\t').collect();
        if campos.len() < 2 {
            continue;
        }
        let numero = |v: &str| v.trim().parse::<f64>().map_err(|_| format!("{v} nao e numero"));
        match campos[0] {
            "r_declarado" => r_declarado = Some(numero(campos[1])?),
            "razao_minima" => razao_minima = Some(numero(campos[1])?),
            "razao_maxima" => razao_maxima = Some(numero(campos[1])?),
            "f_minimo" => f_minimo = Some(numero(campos[1])?),
            _ => {}
        }
    }
    match (r_declarado, razao_minima, razao_maxima, f_minimo) {
        (Some(r), Some(lo), Some(hi), Some(f)) => Ok(CriterioDoLaco { r_declarado: r, razao_minima: lo, razao_maxima: hi, f_minimo: f }),
        _ => Err("criterio do laco incompleto no registro".to_string()),
    }
}

/// Um leitor de campo de JSON, mínimo e declarado: acha a chave e lê o que vem depois. Não é um parser geral;
/// é o suficiente para o registro do laço, e o teste contra o arquivo real prende o resultado.
fn apos_chave<'a>(js: &'a str, chave: &str) -> Option<&'a str> {
    let alvo = format!("\"{chave}\"");
    let pos = js.find(&alvo)? + alvo.len();
    let resto = js[pos..].trim_start();
    let resto = resto.strip_prefix(':')?.trim_start();
    Some(resto)
}

/// O primeiro número depois da chave.
pub fn numero(js: &str, chave: &str) -> Option<f64> {
    let resto = apos_chave(js, chave)?;
    let fim = resto.find(|c: char| !(c.is_ascii_digit() || c == '-' || c == '+' || c == '.' || c == 'e' || c == 'E')).unwrap_or(resto.len());
    resto[..fim].parse::<f64>().ok()
}

/// A primeira lista de números depois da chave.
pub fn lista(js: &str, chave: &str) -> Option<Vec<f64>> {
    let resto = apos_chave(js, chave)?;
    let inicio = resto.find('[')? + 1;
    let fim = resto.find(']')?;
    resto[inicio..fim]
        .split(',')
        .map(|p| p.trim().parse::<f64>().ok())
        .collect::<Option<Vec<f64>>>()
}

/// O primeiro booleano depois da chave.
pub fn booleano(js: &str, chave: &str) -> Option<bool> {
    let resto = apos_chave(js, chave)?;
    if resto.starts_with("true") {
        Some(true)
    } else if resto.starts_with("false") {
        Some(false)
    } else {
        None
    }
}

/// Refaz a conta do critério declarado no pré-registro do ponto 6, contra o registro versionado.
pub fn rederivar_o_laco(resultados: &str, posthoc: &str, criterio: CriterioDoLaco) -> Result<VereditoDoLaco, String> {
    let r_iv = numero(resultados, "r_IV").ok_or("registro sem r_IV")?;
    let ic = lista(resultados, "ic90").ok_or("registro sem ic90")?;
    if ic.len() != 2 {
        return Err("ic90 sem dois extremos".to_string());
    }
    let f = numero(resultados, "F_primeiro_estagio").ok_or("registro sem F do primeiro estagio")?;
    let razao_no_registro = numero(resultados, "razao_0,6_sobre_r_IV").ok_or("registro sem a razao")?;
    let contencao_no_registro = booleano(resultados, "0,6_no_ic90").ok_or("registro sem o campo de contenção")?;
    let vies = numero(resultados, "vies_ingenuo_vs_exogena").ok_or("registro sem o vies do ingenuo")?;

    let razao = criterio.r_declarado / r_iv;
    let sinal_confere = ic[0] > 0.0;
    let ordem_confere = razao >= criterio.razao_minima && razao <= criterio.razao_maxima;
    let estrito = criterio.r_declarado >= ic[0] && criterio.r_declarado <= ic[1];

    let mut divergencias = Vec::new();
    if (razao - razao_no_registro).abs() > 1e-9 {
        divergencias.push(format!("a razao do registro ({razao_no_registro:.6}) nao bate com a conta ({razao:.6})"));
    }
    if contencao_no_registro != estrito {
        divergencias.push(format!("a contencao do registro ({contencao_no_registro}) nao bate com a conta ({estrito})"));
    }
    let veredito = if f < criterio.f_minimo {
        "indecidido: o instrumento e fraco (F abaixo do minimo declarado)".to_string()
    } else if !sinal_confere {
        "nao confere: o IC90 inclui 0 ou fica abaixo de 0".to_string()
    } else if ordem_confere {
        format!("o laco do H3 tem o sinal e a ordem do medido (razao {razao:.4} dentro da banda)")
    } else {
        format!(
            "o laco do H3 superestima o medido: o sinal confere e a ordem nao (razao {razao:.4}, banda de {:.4} a {:.4})",
            criterio.razao_minima, criterio.razao_maxima
        )
    };
    // o post-hoc só entra no relatório se existir, e não decide nada
    let _ = numero(posthoc, "r_IV / P(fila depois da desistência) (correção grosseira, supõe novas chamadas como pedidos comuns)");
    Ok(VereditoDoLaco {
        r_iv,
        ic90: (ic[0], ic[1]),
        f_primeiro_estagio: f,
        razao,
        razao_no_registro,
        sinal_confere,
        ordem_confere,
        estrito,
        contencao_no_registro,
        vies_ingenuo: vies,
        veredito,
        divergencias,
    })
}

/// Relatório da cadeia: uma linha por linha do H3, com o que foi refeito e o que só resta declarar.
pub fn relatar(cadeia: &Cadeia, veredito: Option<&VereditoDoLaco>) -> String {
    let mut s = String::new();
    s.push_str(&format!("carimbo do registro: {}\n", cadeia.carimbo));
    s.push_str(&format!("{:<16} {:<9} {:<4} {:<34} {}\n", "linha", "classe", "marca", "registro versionado", "o que o instrumento faz"));
    for l in &cadeia.linhas {
        let o_que = match l.classe {
            Classe::Derivada if veredito.is_some() => "re-deriva a conta e o veredito",
            Classe::Derivada => "NAO re-deriva: o registro versionado nao foi lido",
            Classe::Medida => "mede de novo a partir do dado na arvore",
            Classe::Registro => "declara: o dado bruto nao esta na arvore",
        };
        s.push_str(&format!("{:<16} {:<9} {:<4} {:<34} {}\n", l.linha, l.classe.nome(), l.marca, l.registro, o_que));
    }
    match veredito {
        Some(v) => {
            s.push_str("\n-- o laco performativo, re-derivado do registro em arvore --\n");
            s.push_str(&format!(
                "estimativa 2SLS: r = {:.10} | IC90 de {:.10} a {:.10} | F do primeiro estagio = {:.2}\n",
                v.r_iv, v.ic90.0, v.ic90.1, v.f_primeiro_estagio
            ));
            s.push_str(&format!(
                "razao r declarado / r medido: {:.10} (o registro traz {:.10}) | banda declarada: {:.4} a {:.4}\n",
                v.razao, v.razao_no_registro, 1.0 / 3.0, 3.0
            ));
            s.push_str(&format!(
                "sinal confere: {} | ordem de grandeza confere: {} | compativel no sentido estrito: {} (o registro diz {})\n",
                v.sinal_confere, v.ordem_confere, v.estrito, v.contencao_no_registro
            ));
            s.push_str(&format!("vies do previsor ingenuo contra a demanda exogena: {:.4}\n", v.vies_ingenuo));
            if v.divergencias.is_empty() {
                s.push_str("divergencias entre a conta e o registro: nenhuma\n");
            } else {
                s.push_str(&format!("divergencias entre a conta e o registro: REPROVADO em {:?}\n", v.divergencias));
            }
            s.push_str(&format!("veredito: {}\n", v.veredito));
        }
        None => s.push_str("\no laco nao foi re-derivado: o registro versionado nao foi lido\n"),
    }
    s
}

/// O laboratório: a cadeia do H3, a prova negativa e a prova positiva.
pub fn rodar(caminho: &str, caminho_resultados: &str, caminho_posthoc: &str) {
    println!("== Ponto 14 :: a cadeia do H3, re-executada ate onde a arvore deixa ==");
    let texto = match std::fs::read_to_string(caminho) {
        Ok(t) => t,
        Err(e) => {
            println!("registro da cadeia nao lido ({caminho}): {e}");
            return;
        }
    };
    let cadeia = match ler_cadeia(&texto) {
        Ok(c) => c,
        Err(e) => {
            println!("registro da cadeia invalido: {e}");
            return;
        }
    };
    let criterio = ler_criterio(&texto);
    let veredito = match (criterio, std::fs::read_to_string(caminho_resultados), std::fs::read_to_string(caminho_posthoc)) {
        (Ok(c), Ok(r), Ok(p)) => rederivar_o_laco(&r, &p, c).ok(),
        _ => None,
    };
    print!("{}", relatar(&cadeia, veredito.as_ref()));

    println!("\n-- prova negativa: um registro em que a razao cai dentro da banda declarada --");
    let dentro = format!(
        "{{\"principal_15min\": {{\"r_IV\": {}, \"ic90\": [{}, {}], \"F_primeiro_estagio\": 5523.46, \"razao_0,6_sobre_r_IV\": {}, \"0,6_no_ic90\": false, \"vies_ingenuo_vs_exogena\": 0.29}}}}",
        0.4, 0.35, 0.45, 0.6 / 0.4
    );
    if let Ok(c) = ler_criterio(&texto) {
        match rederivar_o_laco(&dentro, "{}", c) {
            Ok(v) => println!("  com r medido 0,4: ordem confere = {} | veredito: {}", v.ordem_confere, v.veredito),
            Err(e) => println!("  caso sintetico nao lido: {e}"),
        }
    }

    println!("-- prova negativa 2: um registro em que o campo da razao nao bate com a conta --");
    let mentiroso = "{\"principal_15min\": {\"r_IV\": 0.1446378859, \"ic90\": [0.1319811884, 0.1568385251], \"F_primeiro_estagio\": 5523.46, \"razao_0,6_sobre_r_IV\": 2.0, \"0,6_no_ic90\": true, \"vies_ingenuo_vs_exogena\": 0.29}}";
    if let Ok(c) = ler_criterio(&texto) {
        match rederivar_o_laco(mentiroso, "{}", c) {
            Ok(v) => println!("  divergencias: {:?}", v.divergencias),
            Err(e) => println!("  caso sintetico nao lido: {e}"),
        }
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use std::path::Path;

    fn raiz() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).parent().and_then(Path::parent).expect("raiz").to_path_buf()
    }
    fn ler_arquivo(rel: &str) -> String {
        std::fs::read_to_string(raiz().join(rel)).expect("arquivo do registro")
    }

    #[test]
    fn a_cadeia_tem_quatro_linhas_com_as_marcas_do_registro() {
        let cadeia = ler_cadeia(&ler_arquivo(REGISTRO)).expect("registro da cadeia");
        assert_eq!(cadeia.linhas.len(), 4, "as quatro linhas do H3");
        let marcas: Vec<&str> = cadeia.linhas.iter().map(|l| l.marca.as_str()).collect();
        assert_eq!(marcas, vec!["P", "D", "V", "P"]);
        let classes: Vec<&str> = cadeia.linhas.iter().map(|l| l.classe.nome()).collect();
        assert_eq!(classes, vec!["registro", "registro", "derivada", "registro"]);
    }

    #[test]
    fn o_laco_e_re_derivado_do_registro_versionado() {
        let criterio = ler_criterio(&ler_arquivo(REGISTRO)).expect("criterio");
        assert_eq!(criterio.r_declarado, 0.6);
        let v = rederivar_o_laco(&ler_arquivo(RESULTADOS_DO_LACO), &ler_arquivo(POSTHOC_DO_LACO), criterio).expect("re-derivacao");
        assert!((v.r_iv - 0.1446378859145094).abs() < 1e-12, "r medido: {}", v.r_iv);
        assert!((v.ic90.0 - 0.131981188365084).abs() < 1e-12);
        assert!((v.razao - 4.148290720694298).abs() < 1e-9, "razao: {}", v.razao);
        assert!((v.razao - v.razao_no_registro).abs() < 1e-9);
        assert!(v.sinal_confere);
        assert!(!v.ordem_confere, "a razao 4,148 fica fora da banda");
        assert!(!v.estrito, "0,6 nao esta no IC90");
        assert!(v.divergencias.is_empty(), "a conta bate com o registro: {:?}", v.divergencias);
        assert!(v.veredito.starts_with("o laco do H3 superestima"), "veredito: {}", v.veredito);
    }

    #[test]
    fn o_veredito_vira_quando_a_razao_entra_na_banda() {
        let criterio = ler_criterio(&ler_arquivo(REGISTRO)).expect("criterio");
        let dentro = "{\"principal_15min\": {\"r_IV\": 0.4, \"ic90\": [0.35, 0.45], \"F_primeiro_estagio\": 5523.46, \"razao_0,6_sobre_r_IV\": 1.5, \"0,6_no_ic90\": false, \"vies_ingenuo_vs_exogena\": 0.29}}";
        let v = rederivar_o_laco(dentro, "{}", criterio).expect("re-derivacao");
        assert!(v.ordem_confere);
        assert!(v.veredito.starts_with("o laco do H3 tem o sinal e a ordem"), "veredito: {}", v.veredito);
    }

    #[test]
    fn o_registro_que_mente_na_razao_e_reprovado() {
        let criterio = ler_criterio(&ler_arquivo(REGISTRO)).expect("criterio");
        let mentiroso = "{\"principal_15min\": {\"r_IV\": 0.1446378859, \"ic90\": [0.1319811884, 0.1568385251], \"F_primeiro_estagio\": 5523.46, \"razao_0,6_sobre_r_IV\": 2.0, \"0,6_no_ic90\": true, \"vies_ingenuo_vs_exogena\": 0.29}}";
        let v = rederivar_o_laco(mentiroso, "{}", criterio).expect("re-derivacao");
        assert_eq!(v.divergencias.len(), 2, "a razao e a contencao divergem: {:?}", v.divergencias);
    }

    #[test]
    fn o_instrumento_fraco_indecide() {
        let criterio = ler_criterio(&ler_arquivo(REGISTRO)).expect("criterio");
        let fraco = "{\"principal_15min\": {\"r_IV\": 0.4, \"ic90\": [0.35, 0.45], \"F_primeiro_estagio\": 4.0, \"razao_0,6_sobre_r_IV\": 1.5, \"0,6_no_ic90\": false, \"vies_ingenuo_vs_exogena\": 0.29}}";
        let v = rederivar_o_laco(fraco, "{}", criterio).expect("re-derivacao");
        assert!(v.veredito.starts_with("indecidido"), "veredito: {}", v.veredito);
    }

    #[test]
    fn uma_linha_derivada_sem_registro_versionado_reprova() {
        let criterio = ler_criterio(&ler_arquivo(REGISTRO)).expect("criterio");
        let sem = "{\"principal_15min\": {\"ic90\": [0.1, 0.2]}}";
        assert!(rederivar_o_laco(sem, "{}", criterio).is_err(), "sem r_IV nao ha re-derivacao");
    }

    #[test]
    fn o_leitor_de_json_le_o_registro_real() {
        let js = ler_arquivo(RESULTADOS_DO_LACO);
        assert!((numero(&js, "chamadas_indice").expect("campo") - 198980.0).abs() < 1e-9);
        let ic = lista(&js, "ic90").expect("lista");
        assert_eq!(ic.len(), 2);
        assert_eq!(booleano(&js, "0,6_no_ic90"), Some(false));
    }
}
