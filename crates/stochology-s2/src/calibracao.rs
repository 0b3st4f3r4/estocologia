//! O conjunto rotulado que alimenta o Brier: leitura estrita, amostra minima e abstencao contada.
//!
//! O runner congela respostas; a calibracao precisa de desfecho. Este modulo le o arquivo de desfechos do
//! operador e devolve o escore com as regras declaradas antes de olhar o numero:
//!
//! - **formato estrito** — `fundo/p20-jev/calibracao/desfechos.tsv`, TSV em UTF-8, cabecalho
//!   `id caso_id probabilidade_declarada desfecho_observado fonte_desfecho`, `id` sequencial de 1 a N;
//! - **probabilidade no intervalo aberto** — 0.0 e 1.0 estritos acionam `ExtremoDogmatico`, porque a regra de
//!   ouro proibe os extremos dogmaticos;
//! - **amostra minima** — abaixo de `AMOSTRA_MINIMA` o instrumento devolve `AmostraInsuficiente` e nao publica
//!   escore, em vez de publicar um numero fragil;
//! - **abstencao nao vira 0.5** — o caso em que o modelo absteu sai do denominador do Brier condicional e entra na
//!   taxa de abstencao, que e reportada ao lado do escore.
//!
//! O hash de `desfechos.tsv` e declarado em `calibracao/SHA256`, no mesmo formato e com a mesma regra de tempo
//! do artefato dos pesos: **antes** da primeira execucao.

use crate::ponte::Recusa;
use std::path::Path;

/// O cabecalho exato do arquivo de desfechos.
pub const CABECALHO: &str = "id	caso_id	probabilidade_declarada	desfecho_observado	fonte_desfecho";
/// O nome do arquivo de desfechos.
pub const ARQUIVO: &str = "desfechos.tsv";
/// O diretorio canonico do conjunto rotulado.
pub const DIR_PADRAO: &str = "fundo/p20-jev/calibracao";
/// Abaixo desta contagem o escore nao e publicado. Escolha de especificacao, declarada uma vez.
pub const AMOSTRA_MINIMA: usize = 30;
/// Baldes do ECE. A regra e a do stochology_h3::ece, reusada e nao reescrita, para nao haver
/// duas definicoes de calibracao no mesmo repositorio.
pub const BALDES_ECE: usize = 10;

/// Uma linha do conjunto rotulado.
#[derive(Debug, Clone, PartialEq)]
pub struct Desfecho {
    pub id: usize,
    pub caso_id: String,
    pub probabilidade: f64,
    pub desfecho: bool,
    pub fonte: String,
}

/// O resultado da calibracao, ou o motivo de nao haver escore.
#[derive(Debug, Clone, PartialEq)]
pub enum Calibracao {
    /// Nenhuma linha de dado: nao ha calibracao sem teste empirico.
    Vazia,
    /// Ha dado, e nao ha amostra para publicar escore.
    AmostraInsuficiente { n: usize, minimo: usize },
    /// O escore condicional, com a abstencao ao lado.
    Escore { n: usize, brier: f64, ece: f64, abstencoes: usize, taxa_de_abstencao: f64 },
}

impl Calibracao {
    /// O escore, quando existe.
    pub fn escore(&self) -> Option<f64> {
        match self {
            Calibracao::Escore { brier, .. } => Some(*brier),
            _ => None,
        }
    }

    /// A leitura em uma linha, para o instrumento imprimir.
    pub fn leitura(&self) -> String {
        match self {
            Calibracao::Vazia => {
                String::from("sem conjunto rotulado: nenhum par (probabilidade, desfecho), Brier nao publicado")
            }
            Calibracao::AmostraInsuficiente { n, minimo } => format!(
                "AmostraInsuficiente: {n} pares, minimo declarado de {minimo}; o escore nao e publicado"
            ),
            Calibracao::Escore { n, brier, ece, abstencoes, taxa_de_abstencao } => format!(
                "Brier {brier:.6} e ECE {ece:.6} em {BALDES_ECE} baldes sobre {n} pares | abstencoes {abstencoes} ({:.1}%) fora do denominador",
                taxa_de_abstencao * 100.0
            ),
        }
    }
}

/// Le o TSV com as validacoes de campo, na ordem em que o arquivo se le.
pub fn ler_desfechos(caminho: &str) -> Result<Vec<Desfecho>, Recusa> {
    let texto =
        std::fs::read_to_string(caminho).map_err(|_| Recusa::PesosAusentes(caminho.to_string()))?;
    let mut linhas = texto.lines();
    let cabecalho = linhas.next().unwrap_or("").trim_end();
    if cabecalho != CABECALHO {
        return Err(Recusa::ArtefatoIlegivel(format!(
            "cabecalho {cabecalho:?}, esperado {CABECALHO:?}"
        )));
    }
    let mut saida = Vec::new();
    for (i, linha) in linhas.enumerate() {
        if linha.trim().is_empty() {
            continue;
        }
        let campos: Vec<&str> = linha.split('\t').collect();
        if campos.len() != 5 {
            return Err(Recusa::ArtefatoIlegivel(format!(
                "linha {}: {} campos, esperado 5",
                i + 2,
                campos.len()
            )));
        }
        let id: usize = campos[0]
            .trim()
            .parse()
            .map_err(|_| Recusa::ArtefatoIlegivel(format!("id invalido na linha {}", i + 2)))?;
        let esperado = saida.len() + 1;
        if id != esperado {
            return Err(Recusa::ArtefatoIlegivel(format!(
                "id fora de ordem: {id} na linha {}, esperado {esperado}",
                i + 2
            )));
        }
        let caso_id = campos[1].trim().to_string();
        if caso_id.is_empty() {
            return Err(Recusa::ArtefatoIlegivel(format!("caso_id vazio na linha {}", i + 2)));
        }
        let probabilidade: f64 = campos[2]
            .trim()
            .parse()
            .map_err(|_| Recusa::ArtefatoIlegivel(format!("probabilidade invalida na linha {}", i + 2)))?;
        if !(0.0 < probabilidade && probabilidade < 1.0) {
            return Err(Recusa::ExtremoDogmatico(format!(
                "probabilidade {probabilidade} na linha {}",
                i + 2
            )));
        }
        let desfecho = match campos[3].trim() {
            "0" => false,
            "1" => true,
            outro => {
                return Err(Recusa::ArtefatoIlegivel(format!(
                    "desfecho {outro:?} na linha {}",
                    i + 2
                )))
            }
        };
        let fonte = campos[4].trim().to_string();
        if fonte.is_empty() {
            return Err(Recusa::ArtefatoIlegivel(format!("fonte vazia na linha {}", i + 2)));
        }
        saida.push(Desfecho { id, caso_id, probabilidade, desfecho, fonte });
    }
    Ok(saida)
}

/// Escreve o conjunto rotulado e o seu `SHA256`, no mesmo padrao do artefato dos pesos.
pub fn escrever_desfechos(dir: &Path, linhas: &[(String, f64, bool, String)]) -> std::io::Result<String> {
    std::fs::create_dir_all(dir)?;
    let mut texto = String::from(CABECALHO);
    texto.push('\n');
    for (i, (caso_id, p, o, fonte)) in linhas.iter().enumerate() {
        texto.push_str(&format!(
            "{}	{}	{:.4}	{}	{}
",
            i + 1,
            caso_id,
            p,
            if *o { 1 } else { 0 },
            fonte
        ));
    }
    let caminho = dir.join(ARQUIVO);
    std::fs::write(&caminho, texto.as_bytes())?;
    let hash = stochology_audit::sha256_hex(texto.as_bytes());
    std::fs::write(dir.join("SHA256"), format!("{hash}  {ARQUIVO}
"))?;
    Ok(hash)
}

/// O escore condicional: abstencao sai do denominador, e a taxa vai ao lado.
pub fn calibrar(desfechos: &[Desfecho], abstencoes: usize, minimo: usize) -> Calibracao {
    if desfechos.is_empty() {
        return Calibracao::Vazia;
    }
    let n = desfechos.len();
    if n < minimo {
        return Calibracao::AmostraInsuficiente { n, minimo };
    }
    let pares: Vec<(f64, bool)> = desfechos.iter().map(|d| (d.probabilidade, d.desfecho)).collect();
    let brier = crate::runner::brier(&pares);
    let confianca: Vec<f64> = pares.iter().map(|(p, _)| *p).collect();
    let acerto: Vec<bool> = pares.iter().map(|(_, y)| *y).collect();
    let ece = stochology_h3::ece(&confianca, &acerto, BALDES_ECE);
    let total = n + abstencoes;
    let taxa_de_abstencao = abstencoes as f64 / total.max(1) as f64;
    Calibracao::Escore { n, brier, ece, abstencoes, taxa_de_abstencao }
}

/// Carrega o conjunto rotulado com o hash declarado conferido antes, como no artefato dos pesos.
pub fn carregar(dir: &str, abstencoes: usize, minimo: usize) -> Result<Calibracao, Recusa> {
    let esperado = crate::runner::hash_declarado(dir)?;
    let caminho = format!("{dir}/{ARQUIVO}");
    crate::ponte::verificar_pesos(&caminho, &esperado)?;
    let desfechos = ler_desfechos(&caminho)?;
    Ok(calibrar(&desfechos, abstencoes, minimo))
}

#[cfg(test)]
mod testes {
    use super::*;

    fn temp(nome: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("p20-jev-cal-{nome}-{}", std::process::id()));
        std::fs::create_dir_all(&d).ok();
        d
    }

    fn tres() -> Vec<(String, f64, bool, String)> {
        vec![
            ("C-001".into(), 0.75, true, "registro-hospitalar".into()),
            ("C-002".into(), 0.20, false, "registro-hospitalar".into()),
            ("C-003".into(), 0.60, true, "boletim-auditado".into()),
        ]
    }

    #[test]
    fn o_conjunto_rotulado_le_e_o_brier_confere() {
        let d = temp("le");
        let hash = escrever_desfechos(&d, &tres()).expect("escreve");
        let desfechos = ler_desfechos(&d.join(ARQUIVO).to_string_lossy()).expect("le");
        assert_eq!(desfechos.len(), 3, "tres linhas");
        assert_eq!(desfechos[0].id, 1, "id sequencial");
        let esperado =
            ((0.75f64 - 1.0).powi(2) + (0.20f64 - 0.0).powi(2) + (0.60f64 - 1.0).powi(2)) / 3.0;
        match calibrar(&desfechos, 0, 1) {
            Calibracao::Escore { brier, n, .. } => {
                assert_eq!(n, 3);
                assert!((brier - esperado).abs() < 1e-12, "{brier} contra {esperado}");
            }
            outro => panic!("esperava escore: {outro:?}"),
        }
        assert_eq!(hash.len(), 64, "o manifesto tem 64 hex");
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn probabilidade_no_extremo_recusa() {
        let d = temp("extremo");
        escrever_desfechos(&d, &[("C-1".into(), 1.0, true, "fonte".into())]).expect("escreve");
        match ler_desfechos(&d.join(ARQUIVO).to_string_lossy()) {
            Err(Recusa::ExtremoDogmatico(_)) => {}
            outro => panic!("esperava extremo dogmatico: {outro:?}"),
        }
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn desfecho_que_nao_e_0_nem_1_e_artefato_ilegivel() {
        let d = temp("desfecho");
        std::fs::create_dir_all(&d).ok();
        let texto = format!("{CABECALHO}\n1\tC-1\t0.4000\t2\tfonte\n");
        std::fs::write(d.join(ARQUIVO), texto.as_bytes()).expect("escreve");
        match ler_desfechos(&d.join(ARQUIVO).to_string_lossy()) {
            Err(Recusa::ArtefatoIlegivel(m)) => assert!(m.contains("desfecho"), "{m}"),
            outro => panic!("esperava artefato ilegivel: {outro:?}"),
        }
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn id_fora_de_ordem_recusa() {
        let d = temp("ordem");
        std::fs::create_dir_all(&d).ok();
        let texto = format!("{CABECALHO}\n2\tC-1\t0.4000\t1\tfonte\n");
        std::fs::write(d.join(ARQUIVO), texto.as_bytes()).expect("escreve");
        match ler_desfechos(&d.join(ARQUIVO).to_string_lossy()) {
            Err(Recusa::ArtefatoIlegivel(m)) => assert!(m.contains("fora de ordem"), "{m}"),
            outro => panic!("esperava id fora de ordem: {outro:?}"),
        }
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn sem_dado_nao_ha_brier() {
        let cal = calibrar(&[], 0, AMOSTRA_MINIMA);
        assert_eq!(cal, Calibracao::Vazia);
        assert!(cal.escore().is_none(), "sem par, nenhum escore");
        assert!(cal.leitura().contains("Brier nao publicado"), "{}", cal.leitura());
    }

    #[test]
    fn amostra_abaixo_do_minimo_nao_publica_escore() {
        let d = temp("minimo");
        escrever_desfechos(&d, &tres()).expect("escreve");
        let desfechos = ler_desfechos(&d.join(ARQUIVO).to_string_lossy()).expect("le");
        let cal = calibrar(&desfechos, 0, AMOSTRA_MINIMA);
        match &cal {
            Calibracao::AmostraInsuficiente { n, minimo } => assert_eq!((*n, *minimo), (3, AMOSTRA_MINIMA)),
            outro => panic!("esperava amostra insuficiente: {outro:?}"),
        }
        assert!(cal.leitura().contains("AmostraInsuficiente"), "{}", cal.leitura());
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn a_abstencao_sai_do_denominador_e_entra_na_taxa() {
        let d = temp("abstencao");
        escrever_desfechos(&d, &tres()).expect("escreve");
        let desfechos = ler_desfechos(&d.join(ARQUIVO).to_string_lossy()).expect("le");
        let sem = calibrar(&desfechos, 0, 1);
        let com = calibrar(&desfechos, 1, 1);
        assert_eq!(sem.escore(), com.escore(), "a abstencao nao muda o escore condicional");
        match com {
            Calibracao::Escore { abstencoes, taxa_de_abstencao, n, .. } => {
                assert_eq!((n, abstencoes), (3, 1));
                assert!((taxa_de_abstencao - 0.25).abs() < 1e-12, "{taxa_de_abstencao}");
            }
            outro => panic!("esperava escore: {outro:?}"),
        }
        std::fs::remove_dir_all(&d).ok();
    }


    #[test]
    fn o_ece_usa_a_regra_do_h3_e_os_baldes_declarados() {
        let um = Desfecho {
            id: 1,
            caso_id: "C-1".into(),
            probabilidade: 0.3,
            desfecho: false,
            fonte: "f".into(),
        };
        match calibrar(&[um], 0, 1) {
            Calibracao::Escore { ece, .. } => assert!((ece - 0.3).abs() < 1e-12, "ece {ece}"),
            outro => panic!("esperava escore: {outro:?}"),
        }
        let a = Desfecho {
            id: 1,
            caso_id: "C-A".into(),
            probabilidade: 0.55,
            desfecho: true,
            fonte: "f".into(),
        };
        let b = Desfecho {
            id: 2,
            caso_id: "C-B".into(),
            probabilidade: 0.55,
            desfecho: false,
            fonte: "f".into(),
        };
        match calibrar(&[a, b], 0, 1) {
            Calibracao::Escore { ece, .. } => assert!((ece - 0.05).abs() < 1e-12, "ece {ece}"),
            outro => panic!("esperava escore: {outro:?}"),
        }
        assert_eq!(BALDES_ECE, 10, "baldes declarados");
        assert!(calibrar(&[], 0, 1).leitura().contains("Brier nao publicado"));
    }

    #[test]
    fn o_portao_do_hash_vale_para_o_conjunto_rotulado() {
        let d = temp("hash");
        escrever_desfechos(&d, &tres()).expect("escreve");
        assert!(carregar(&d.to_string_lossy(), 0, 1).is_ok(), "com hash declarado, carrega");
        let caminho = d.join(ARQUIVO);
        let mut texto = std::fs::read_to_string(&caminho).expect("le");
        texto.push_str("4\tC-004\t0.3000\t0\tfonte\n");
        std::fs::write(&caminho, texto.as_bytes()).expect("escreve");
        match carregar(&d.to_string_lossy(), 0, 1) {
            Err(Recusa::PesosComHashDiferente { .. }) => {}
            outro => panic!("esperava recusa por hash: {outro:?}"),
        }
        std::fs::remove_dir_all(&d).ok();
    }
}
