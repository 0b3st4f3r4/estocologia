//! O ponto 13 da pesquisa: a escuta de quem paga o erro e o lacre da caixa.
//!
//! O programa tem sete decisões que mexem com pessoas reais e o registro diz **nenhum** ouvido nas sete
//! (`governanca/GRUPOS.md`). O protocolo da frente está em `fundo/p13-escuta/protocolo.md`, o registro em
//! máquina em `fundo/p13-escuta/registro.tsv`, e este módulo é o que lê o registro e **sabe reprovar**: uma
//! decisão que se diz emitida com a coluna "ouvidos" em nenhum é divergência, e enquanto a cobertura de escuta não
//! chega ao mínimo declarado a caixa fica lacrada.
//!
//! Rodar: ~labs --escuta~

use std::path::Path;

/// O caminho do registro, relativo à raiz do repositório — a mesma convenção dos instrumentos que leem `dados-reais/`.
pub const REGISTRO: &str = "fundo/p13-escuta/registro.tsv";

/// Uma decisão do programa, com quem paga o erro dela.
#[derive(Debug, Clone, PartialEq)]
pub struct Linha {
    pub decisao: String,
    pub grupos: String,
    /// Quantos titulares a decisão alcança, quando o número existe ("-" quando não).
    pub titulares: String,
    /// Quem foi ouvido; "nenhum" enquanto ninguém falou.
    pub ouvidos: String,
    /// Onde está o registro assinado da escuta; "-" enquanto não houver.
    pub registro: String,
    /// Se a decisão pode ser emitida como diretriz: "sim" ou "nao".
    pub pode_emitir: String,
}

impl Linha {
    /// A escuta existe quando há alguém ouvido **e** o registro assinado está apontado.
    pub fn tem_escuta(&self) -> bool {
        self.ouvidos != "nenhum" && self.ouvidos != "-" && self.registro != "-" && !self.registro.is_empty()
    }

    /// A divergência que o portão pega: a decisão se diz emitida sem ninguém ouvido.
    pub fn divergente(&self) -> bool {
        self.pode_emitir == "sim" && !self.tem_escuta()
    }
}

/// O registro inteiro, com o carimbo, o prazo da janela e a cobertura mínima declarada.
#[derive(Debug, Clone, PartialEq)]
pub struct Registro {
    pub carimbo: String,
    pub prazo: String,
    pub grupos_minimos: usize,
    pub linhas: Vec<Linha>,
}

/// O que o registro decide: quantos grupos já foram ouvidos, se a caixa segue lacrada e quem está divergente.
#[derive(Debug, Clone, PartialEq)]
pub struct Veredito {
    pub cobertura: usize,
    pub grupos_minimos: usize,
    pub lacrado: bool,
    pub divergencias: Vec<String>,
}

/// Lê o registro: linhas de comentário com `# chave<TAB>valor` e a tabela separada por tabulação.
pub fn ler(texto: &str) -> Result<Registro, String> {
    let mut carimbo = String::new();
    let mut prazo = String::new();
    let mut grupos_minimos: Option<usize> = None;
    let mut linhas = Vec::new();
    for linha in texto.lines() {
        let linha = linha.trim_end();
        if linha.trim().is_empty() {
            continue;
        }
        if let Some(resto) = linha.strip_prefix('#') {
            let campos: Vec<&str> = resto.trim().split('\t').collect();
            if campos.len() >= 2 {
                match campos[0] {
                    "registros" => carimbo = campos[1].trim().to_string(),
                    "prazo" => prazo = campos[1].trim().to_string(),
                    "grupos_minimos" => {
                        grupos_minimos = Some(campos[1].trim().parse().map_err(|_| "grupos_minimos nao e numero")?)
                    }
                    _ => {}
                }
            }
            continue;
        }
        let campos: Vec<&str> = linha.split('\t').collect();
        if campos.len() < 6 || campos[0] == "decisao" {
            continue;
        }
        linhas.push(Linha {
            decisao: campos[0].to_string(),
            grupos: campos[1].to_string(),
            titulares: campos[2].to_string(),
            ouvidos: campos[3].to_string(),
            registro: campos[4].to_string(),
            pode_emitir: campos[5].to_string(),
        });
    }
    if prazo.is_empty() || grupos_minimos.is_none() {
        return Err("registro sem prazo ou sem grupos_minimos".to_string());
    }
    Ok(Registro { carimbo, prazo, grupos_minimos: grupos_minimos.unwrap(), linhas })
}

/// Lê o registro de um arquivo. Devolve o erro em vez de entrar em pânico, para o portão poder reprovar.
pub fn ler_arquivo(caminho: &str) -> Result<Registro, String> {
    if !Path::new(caminho).exists() {
        return Err(format!("registro ausente: {caminho}"));
    }
    let texto = std::fs::read_to_string(caminho).map_err(|e| e.to_string())?;
    ler(&texto)
}

impl Registro {
    /// A cobertura é o número de decisões com escuta registrada; o lacre cai quando ela chega ao mínimo.
    pub fn veredito(&self) -> Veredito {
        let cobertura = self.linhas.iter().filter(|l| l.tem_escuta()).count();
        Veredito {
            cobertura,
            grupos_minimos: self.grupos_minimos,
            lacrado: cobertura < self.grupos_minimos,
            divergencias: self.linhas.iter().filter(|l| l.divergente()).map(|l| l.decisao.clone()).collect(),
        }
    }

    pub fn sem_escuta(&self) -> Vec<&Linha> {
        self.linhas.iter().filter(|l| !l.tem_escuta()).collect()
    }
}

/// Relatório do portão: a tabela do registro, a cobertura, o prazo e o veredito.
pub fn relatar(registro: &Registro) -> String {
    let veredito = registro.veredito();
    let mut saida = String::new();
    saida.push_str(&format!(
        "decisoes: {} | carimbo do registro: {} | prazo da janela: {} | cobertura minima: {} de {}\n",
        registro.linhas.len(),
        registro.carimbo,
        registro.prazo,
        registro.grupos_minimos,
        registro.linhas.len()
    ));
    saida.push_str(&format!(
        "{:<36} {:>9} {:<12} {:<10} {}\n",
        "decisao", "titulares", "ouvidos", "registro", "pode emitir"
    ));
    for linha in &registro.linhas {
        saida.push_str(&format!(
            "{:<36} {:>9} {:<12} {:<10} {}\n",
            linha.decisao, linha.titulares, linha.ouvidos, linha.registro, linha.pode_emitir
        ));
    }
    saida.push_str(&format!(
        "\ncobertura de escuta: {} de {} grupos ouvidos\n",
        veredito.cobertura,
        registro.linhas.len()
    ));
    if veredito.divergencias.is_empty() {
        saida.push_str("divergencias: nenhuma — nenhuma decisao se diz emitida sem ninguem ouvido\n");
    } else {
        saida.push_str(&format!("divergencias: REPROVADO em {:?}\n", veredito.divergencias));
    }
    if veredito.lacrado {
        saida.push_str(&format!(
            "veredito: CAIXA LACRADA — a cobertura de {} fica abaixo do minimo de {}; as decisoes ficam como analise de sensibilidade (n = 1) e nao viram diretriz de politica publica nem decisao de portao F8 ate {} ou ate a cobertura minima\n",
            veredito.cobertura, veredito.grupos_minimos, registro.prazo
        ));
    } else {
        saida.push_str(&format!(
            "veredito: CAIXA ABERTA para as decisoes cobertas — a cobertura de {} alcancou o minimo de {}\n",
            veredito.cobertura, veredito.grupos_minimos
        ));
    }
    saida
}

/// O laboratorio: o registro do programa, a prova negativa e a prova positiva.
pub fn rodar(caminho: &str) {
    println!("== Ponto 13 :: a escuta de quem paga o erro ==");
    match ler_arquivo(caminho) {
        Ok(registro) => {
            print!("{}", relatar(&registro));
            let sem = registro.sem_escuta();
            println!("\nsem escuta, uma por uma:");
            for linha in sem {
                println!("  {:<34} | {}", linha.decisao, linha.grupos);
            }
        }
        Err(erro) => println!("registro nao lido: {erro}"),
    }

    println!("\n-- prova negativa: a decisao que se diz emitida sem ninguem ouvido --");
    let mentiroso = "# prazo\t2026-12-15\n# grupos_minimos\t2\n# registros\t2026-10-16\ndecisao\tgrupos\ttitulares\touvidos\tregistro\tpode_emitir\nh3/prisoes\tdetidos\t5226\tnenhum\t-\tsim\nh3/prisoes20\tprisoes\t65276\tnenhum\t-\tnao\nh3/prisoes_ri\tparadas\t86113\tnenhum\t-\tnao\n";
    match ler(mentiroso) {
        Ok(r) => println!("{}", relatar(&r)),
        Err(e) => println!("nao li o caso sintetico: {e}"),
    }

    println!("-- prova positiva: dois grupos ouvidos, com registro assinado --");
    let honesto = "# prazo\t2026-12-15\n# grupos_minimos\t2\n# registros\t2026-10-16\ndecisao\tgrupos\ttitulares\touvidos\tregistro\tpode_emitir\nh3/prisoes\tdetidos\t5226\tassociacao de defesa publica de TORONTO\tregistro-01.md\tsim\nh3/prisoes20\tprisoes\t65276\tcoletivo de ex-detentos\tregistro-02.md\tsim\n";
    match ler(honesto) {
        Ok(r) => println!("{}", relatar(&r)),
        Err(e) => println!("nao li o caso sintetico: {e}"),
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    fn caminho_do_registro() -> String {
        let raiz = Path::new(env!("CARGO_MANIFEST_DIR")).parent().and_then(Path::parent).expect("raiz").to_path_buf();
        raiz.join(REGISTRO).to_string_lossy().to_string()
    }

    #[test]
    fn o_registro_do_programa_tem_sete_decisoes_e_nenhum_ouvido() {
        let registro = ler_arquivo(&caminho_do_registro()).expect("registro do ponto 13");
        assert_eq!(registro.linhas.len(), 7, "as sete decisoes de governanca/GRUPOS.md");
        assert_eq!(registro.sem_escuta().len(), 7);
        assert_eq!(registro.veredito().cobertura, 0);
        assert_eq!(registro.veredito().grupos_minimos, 2);
        assert_eq!(registro.veredito().divergencias.len(), 0, "o registro nao mente");
        assert!(registro.veredito().lacrado);
    }

    #[test]
    fn a_decisao_emitida_sem_ninguem_ouvido_reprova() {
        let texto = "# prazo\t2026-12-15\n# grupos_minimos\t2\n# registros\t2026-10-16\ndecisao\tgrupos\ttitulares\touvidos\tregistro\tpode_emitir\nh3/prisoes\tdetidos\t5226\tnenhum\t-\tsim\n";
        let registro = ler(texto).expect("caso sintetico");
        let veredito = registro.veredito();
        assert_eq!(veredito.divergencias, vec!["h3/prisoes".to_string()]);
        assert!(veredito.lacrado, "com um grupo ouvido a caixa continua lacrada");
    }

    #[test]
    fn a_cobertura_minima_abre_a_caixa() {
        let texto = "# prazo\t2026-12-15\n# grupos_minimos\t2\n# registros\t2026-10-16\ndecisao\tgrupos\ttitulares\touvidos\tregistro\tpode_emitir\nh3/prisoes\tdetidos\t5226\tdefensoria\tregistro-01.md\tsim\nh3/prisoes20\tprisoes\t65276\tcoletivo\tregistro-02.md\tsim\n";
        let veredito = ler(texto).expect("caso sintetico").veredito();
        assert_eq!(veredito.cobertura, 2);
        assert!(!veredito.lacrado);
        assert!(veredito.divergencias.is_empty());
    }

    #[test]
    fn a_escuta_sem_registro_assinado_nao_conta() {
        let texto = "# prazo\t2026-12-15\n# grupos_minimos\t2\n# registros\t2026-10-16\ndecisao\tgrupos\ttitulares\touvidos\tregistro\tpode_emitir\nh3/prisoes\tdetidos\t5226\tconversa informal\t-\tnao\n";
        let registro = ler(texto).expect("caso sintetico");
        assert_eq!(registro.veredito().cobertura, 0, "conversa sem registro nao e escuta");
    }

    #[test]
    fn o_prazo_e_a_cobertura_minima_saem_do_registro() {
        let registro = ler_arquivo(&caminho_do_registro()).expect("registro do ponto 13");
        assert_eq!(registro.prazo, "2026-12-15");
        assert_eq!(registro.carimbo, "2026-10-16");
    }
}
