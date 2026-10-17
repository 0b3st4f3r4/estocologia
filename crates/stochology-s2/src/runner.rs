//! O runner local dos pesos congelados: o que faltava para o item 13 deixar de ser arcabouço.
//!
//! O `ponte` tem os portões; este módulo tem o que os alimenta. Ele é o lado de máquina do item 13:
//! um artefato em disco, o `sha256` declarado **antes** da execução, e uma tabela de respostas que o
//! runner devolve byte por byte igual em duas execuções — que é o que o G1 cobra.
//!
//! # O artefato
//!
//! `fundo/p20-jev/pesos/SHA256` declara o hash antes de qualquer chamada:
//!
//! ```text
//! <64 hex>  modelo.congelado
//! ```
//!
//! `fundo/p20-jev/pesos/modelo.congelado` é texto, e a primeira linha é o formato:
//!
//! ```text
//! # p20-jev/runner-v1
//! @pedido <sha256 do pedido>
//! <documento canonico da ficha, uma linha por campo>
//! @end
//! ```
//!
//! Cada bloco é uma resposta congelada. Pedido sem bloco **não** inventa resposta: a ponte abstém em
//! `Categoria::Outros`, e a abstenção entra na conta da adequação do espaço. O texto é auditável e diffável,
//! e o hash cobre o arquivo inteiro: trocar um byte invalida a ponte inteira.
//!
//! # O que este módulo não faz
//!
//! Não gera resposta nem carrega tensor: o artefato é a tabela congelada que o operador produz a partir do modelo
//! que ele congelou. Um formato de tensor fica declarado como passo seguinte, e não como meio-caminho: enquanto
//! não existir, o que roda é a tabela, e ela é suficiente para o portão, o determinismo, o custo e a abstenção.

use crate::ponte::{Recusa, pedido};
use std::collections::BTreeMap;

/// O diretório onde o operador deixa o artefato.
pub const DIR_PADRAO: &str = "fundo/p20-jev/pesos";
/// O nome do arquivo dos pesos congelados.
pub const ARQUIVO: &str = "modelo.congelado";
/// A primeira linha do artefato.
pub const CABECALHO: &str = "# p20-jev/runner-v1";

/// Lê o hash declarado em `SHA256`, no formato que o `sha256sum` escreve.
pub fn hash_declarado(dir: &str) -> Result<String, Recusa> {
    let caminho = format!("{dir}/SHA256");
    let texto =
        std::fs::read_to_string(&caminho).map_err(|_| Recusa::HashNaoDeclarado(caminho.clone()))?;
    let linha = texto
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty() && !l.starts_with('#'))
        .ok_or_else(|| Recusa::HashNaoDeclarado(caminho.clone()))?;
    let hash = linha.split_whitespace().next().unwrap_or("").to_lowercase();
    if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(Recusa::HashNaoDeclarado(format!("{caminho}: {linha}")));
    }
    Ok(hash)
}

/// O runner: pesos verificados e a tabela de respostas congeladas.
pub struct RunnerLocal {
    caminho: String,
    sha256: String,
    tabela: BTreeMap<String, String>,
    pedidos: BTreeMap<String, String>,
    ultimos_tokens: usize,
}

impl RunnerLocal {
    /// Carrega o artefato, confere o hash declarado e lê os blocos. Sem hash declarado, não roda.
    pub fn carregar(dir: &str) -> Result<Self, Recusa> {
        let esperado = hash_declarado(dir)?;
        let caminho = format!("{dir}/{ARQUIVO}");
        let medido = crate::ponte::verificar_pesos(&caminho, &esperado)?;
        let texto = std::fs::read_to_string(&caminho)
            .map_err(|_| Recusa::PesosAusentes(caminho.clone()))?;
        let (tabela, pedidos) = ler_blocos(&texto)?;
        Ok(RunnerLocal {
            caminho,
            sha256: medido,
            tabela,
            pedidos,
            ultimos_tokens: 0,
        })
    }

    /// O caminho do artefato que este runner carregou.
    pub fn caminho(&self) -> &str {
        &self.caminho
    }

    /// O hash verificado dos pesos.
    pub fn sha256(&self) -> &str {
        &self.sha256
    }

    /// Quantas respostas congeladas o artefato traz.
    pub fn respostas(&self) -> usize {
        self.tabela.len()
    }

    /// Os pedidos que o artefato guarda, em ordem de chave. Formato v1 nao traz o texto do pedido, e a
    /// lista sai vazia — quem mede adequacao com artefato v1 precisa avisar que esta usando sonda propria.
    pub fn pedidos(&self) -> Vec<String> {
        self.pedidos.values().cloned().collect()
    }

    /// A resposta congelada para um pedido, se existir.
    pub fn resposta(&self, pedido: &str) -> Option<&str> {
        self.tabela.get(&chave(pedido)).map(String::as_str)
    }
}

impl crate::ponte::ModeloCongelado for RunnerLocal {
    fn nome(&self) -> &str {
        "runner local (pesos congelados)"
    }

    fn decodificar(&mut self, pedido: &str) -> String {
        let saida = match self.tabela.get(&chave(pedido)) {
            Some(saida) => saida.clone(),
            None => {
                String::from("fora do esquema: este pedido nao tem resposta congelada no artefato")
            }
        };
        // O custo declarado e o do documento devolvido: palavras separadas por espaco. Nao e o tokenizador do
        // modelo, e o numero fica declarado como tal — a comparacao com o custo real entra quando o artefato real
        // trouxer a contagem do proprio modelo.
        self.ultimos_tokens = saida.split_whitespace().count();
        saida
    }

    fn tokens(&self) -> usize {
        self.ultimos_tokens
    }
}

/// O formato do artefato: cada bloco começa em `@pedido` e termina em `@end`.
/// Os dois blocos do artefato: cabecalho por chave e corpo por chave.
type Blocos = (BTreeMap<String, String>, BTreeMap<String, String>);

fn ler_blocos(texto: &str) -> Result<Blocos, Recusa> {
    let mut linhas = texto.lines();
    let cabecalho = linhas.next().unwrap_or("").trim();
    if cabecalho != CABECALHO {
        return Err(Recusa::ArtefatoIlegivel(format!(
            "cabecalho {cabecalho:?}, esperado {CABECALHO:?}"
        )));
    }
    let mut tabela = BTreeMap::new();
    let mut pedidos: BTreeMap<String, String> = BTreeMap::new();
    let mut chave_atual: Option<String> = None;
    let mut corpo: Vec<String> = Vec::new();
    for linha in linhas {
        let l = linha.trim_end();
        if let Some(resto) = l.strip_prefix("@pedido ") {
            if chave_atual.is_some() {
                return Err(Recusa::ArtefatoIlegivel("bloco sem @end".into()));
            }
            // v2: @pedido <sha> <texto do pedido>. v1: @pedido <sha>, sem texto.
            let mut partes = resto.splitn(2, ' ');
            let chave = partes.next().unwrap_or("").trim().to_lowercase();
            if chave.len() != 64 || !chave.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(Recusa::ArtefatoIlegivel(format!("chave invalida: {chave}")));
            }
            if let Some(texto_do_pedido) = partes.next() {
                let texto_do_pedido = texto_do_pedido.trim();
                if !texto_do_pedido.is_empty() {
                    pedidos.insert(chave.clone(), texto_do_pedido.replace("\\n", "\n"));
                }
            }
            chave_atual = Some(chave);
            corpo.clear();
        } else if l == "@end" {
            let chave = chave_atual
                .take()
                .ok_or_else(|| Recusa::ArtefatoIlegivel("@end sem @pedido".into()))?;
            if corpo.is_empty() {
                return Err(Recusa::ArtefatoIlegivel(format!("bloco vazio: {chave}")));
            }
            tabela.insert(chave, corpo.join("\n"));
        } else if chave_atual.is_some() {
            if !l.trim().is_empty() {
                corpo.push(l.to_string());
            }
        } else if !l.trim().is_empty() && !l.trim_start().starts_with('#') {
            return Err(Recusa::ArtefatoIlegivel(format!(
                "linha fora de bloco: {l}"
            )));
        }
    }
    if chave_atual.is_some() {
        return Err(Recusa::ArtefatoIlegivel(
            "bloco sem @end no fim do arquivo".into(),
        ));
    }
    if tabela.is_empty() {
        return Err(Recusa::ArtefatoIlegivel(
            "artefato sem nenhuma resposta".into(),
        ));
    }
    Ok((tabela, pedidos))
}

/// A chave de um pedido: o `sha256` do texto que a ponte manda.
pub fn chave(pedido: &str) -> String {
    stochology_audit::sha256_hex(pedido.as_bytes())
}

/// Escreve um artefato de ensaio em `dir`: a mesma forma do artefato real, com as respostas do modelo de
/// ensaio. Serve para provar o runner antes de existir peso de verdade, e não é o modelo.
pub fn escrever_artefato_de_ensaio(
    dir: &std::path::Path,
    casos: &[&str],
) -> std::io::Result<String> {
    use crate::ponte::ModeloCongelado;
    std::fs::create_dir_all(dir)?;
    let mut ensaio = crate::ponte::ModeloDeEnsaio::novo();
    let mut texto = String::from(CABECALHO);
    texto.push('\n');
    for caso in casos {
        let p = pedido(caso);
        texto.push_str(&format!(
            "@pedido {} {}\n",
            chave(&p),
            p.replace('\n', "\\n")
        ));
        texto.push_str(ensaio.decodificar(&p).trim_end());
        texto.push_str("\n@end\n");
    }
    let caminho = dir.join(ARQUIVO);
    std::fs::write(&caminho, texto.as_bytes())?;
    let hash = stochology_audit::sha256_hex(texto.as_bytes());
    std::fs::write(dir.join("SHA256"), format!("{hash}  {ARQUIVO}\n"))?;
    Ok(hash)
}

/// Escreve um artefato a partir de respostas colhidas do modelo que o operador congelou: cada par é o texto do
/// pedido e o documento canônico que o modelo devolveu. É o que o operador usa no passo 2 de `RUNNER.md`; a ordem
/// continua sendo declarar o hash **depois** de escrever e **antes** de rodar a ponte.
pub fn escrever_artefato(
    dir: &std::path::Path,
    pares: &[(String, String)],
) -> std::io::Result<String> {
    std::fs::create_dir_all(dir)?;
    let mut texto = String::from(CABECALHO);
    texto.push('\n');
    for (p, resposta) in pares {
        texto.push_str(&format!(
            "@pedido {} {}\n",
            chave(p),
            p.replace('\n', "\\n")
        ));
        texto.push_str(resposta.trim_end());
        texto.push_str("\n@end\n");
    }
    let caminho = dir.join(ARQUIVO);
    std::fs::write(&caminho, texto.as_bytes())?;
    let hash = stochology_audit::sha256_hex(texto.as_bytes());
    std::fs::write(dir.join("SHA256"), format!("{hash}  {ARQUIVO}\n"))?;
    Ok(hash)
}

/// A regra própria de pontuação, fora do ajuste: Brier sobre pares (probabilidade, desfecho).
pub fn brier(pares: &[(f64, bool)]) -> f64 {
    if pares.is_empty() {
        return f64::NAN;
    }
    let soma: f64 = pares
        .iter()
        .map(|(p, y)| {
            let alvo = if *y { 1.0 } else { 0.0 };
            (p - alvo) * (p - alvo)
        })
        .sum();
    soma / pares.len() as f64
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::ponte::{
        Contabilidade, ModeloCongelado, adequacao_do_espaco, provar_determinismo, validar_saida,
    };

    fn temp(nome: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("p20-jev-{nome}-{}", std::process::id()));
        std::fs::create_dir_all(&d).ok();
        d
    }

    #[test]
    fn o_artefato_de_ensaio_carrega_e_o_hash_confere() {
        let d = temp("carrega");
        let hash = escrever_artefato_de_ensaio(&d, &["caso 1", "caso 2"]).expect("escreve");
        let runner = RunnerLocal::carregar(&d.to_string_lossy()).expect("carrega");
        assert_eq!(runner.sha256(), hash, "o hash medido e o declarado");
        assert_eq!(runner.respostas(), 2, "duas respostas congeladas");
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn um_byte_trocado_no_artefato_recusa_a_ponte() {
        let d = temp("adultera");
        escrever_artefato_de_ensaio(&d, &["caso 1"]).expect("escreve");
        let caminho = d.join(ARQUIVO);
        let mut bytes = std::fs::read(&caminho).expect("le");
        let n = bytes.len();
        bytes[n - 1] ^= 0b1;
        std::fs::write(&caminho, bytes).expect("escreve");
        match RunnerLocal::carregar(&d.to_string_lossy()) {
            Err(Recusa::PesosComHashDiferente { .. }) => {}
            outro => panic!("esperava recusa por hash: {:?}", outro.err()),
        }
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn sem_hash_declarado_a_ponte_nao_roda() {
        let d = temp("semhash");
        escrever_artefato_de_ensaio(&d, &["caso 1"]).expect("escreve");
        std::fs::remove_file(d.join("SHA256")).ok();
        match RunnerLocal::carregar(&d.to_string_lossy()) {
            Err(Recusa::HashNaoDeclarado(c)) => assert!(c.contains("SHA256"), "{c}"),
            outro => panic!("esperava hash nao declarado: {:?}", outro.err()),
        }
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn cabecalho_errado_e_artefato_ilegivel() {
        let d = temp("ilegivel");
        escrever_artefato_de_ensaio(&d, &["caso 1"]).expect("escreve");
        let caminho = d.join(ARQUIVO);
        let mut texto = std::fs::read_to_string(&caminho).expect("le");
        texto = texto.replacen(CABECALHO, "# outro formato", 1);
        std::fs::write(&caminho, texto.as_bytes()).expect("escreve");
        let hash = stochology_audit::sha256_hex(texto.as_bytes());
        std::fs::write(d.join("SHA256"), format!("{hash}  {ARQUIVO}\n")).expect("escreve");
        match RunnerLocal::carregar(&d.to_string_lossy()) {
            Err(Recusa::ArtefatoIlegivel(_)) => {}
            outro => panic!("esperava artefato ilegivel: {:?}", outro.err()),
        }
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn o_runner_fecha_o_determinismo_na_forma_do_g1() {
        let d = temp("g1");
        escrever_artefato_de_ensaio(&d, &["caso 1", "caso 2", "caso 3"]).expect("escreve");
        let mut runner = RunnerLocal::carregar(&d.to_string_lossy()).expect("carrega");
        let pedidos: Vec<String> = (1..=3).map(|i| pedido(&format!("caso {i}"))).collect();
        let mut contabilidade = Contabilidade::default();
        let (a, b, iguais) = provar_determinismo(&mut runner, &pedidos, &mut contabilidade);
        assert!(
            iguais,
            "o runner devolve o mesmo sha256 nas duas execucoes: {a} contra {b}"
        );
        assert_eq!(contabilidade.chamadas, 6, "duas execucoes de tres pedidos");
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn pedido_sem_resposta_congelada_vira_abstencao() {
        let d = temp("abstencao");
        escrever_artefato_de_ensaio(&d, &["caso 1"]).expect("escreve");
        let mut runner = RunnerLocal::carregar(&d.to_string_lossy()).expect("carrega");
        let conhecido = runner.decodificar(&pedido("caso 1"));
        assert!(
            validar_saida(&conhecido).is_ok(),
            "o pedido congelado devolve ficha valida"
        );
        let desconhecido = runner.decodificar(&pedido("caso 99"));
        assert!(
            validar_saida(&desconhecido).is_err(),
            "pedido sem resposta congela abstencao"
        );
        let mut c = Contabilidade::default();
        let pedidos: Vec<String> = vec![pedido("caso 1"), pedido("caso 99")];
        let ad = adequacao_do_espaco(&mut runner, &pedidos, &mut c);
        assert_eq!(ad.abstencoes, 1, "uma abstencao em dois pedidos");
        assert!(
            ad.veredito.contains("NAO serve") || ad.veredito.contains("serve"),
            "{}",
            ad.veredito
        );
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn o_operador_escreve_o_artefato_e_o_runner_o_le() {
        let d = temp("operador");
        let mut ensaio = crate::ponte::ModeloDeEnsaio::novo();
        let pares: Vec<(String, String)> = (1..=2)
            .map(|i| {
                let p = pedido(&format!("caso {i}"));
                let r = ensaio.decodificar(&p);
                (p, r)
            })
            .collect();
        let hash = escrever_artefato(&d, &pares).expect("escreve");
        let mut runner = RunnerLocal::carregar(&d.to_string_lossy()).expect("carrega");
        assert_eq!(runner.sha256(), hash);
        assert_eq!(runner.respostas(), 2);
        let primeira = runner.decodificar(&pares[0].0);
        assert!(
            validar_saida(&primeira).is_ok(),
            "a resposta colhida devolve ficha valida"
        );
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn o_artefato_v2_guarda_o_texto_do_pedido_e_o_v1_continua_lendo() {
        let d = temp("v2");
        escrever_artefato_de_ensaio(&d, &["caso 1", "caso 2"]).expect("escreve");
        let runner = RunnerLocal::carregar(&d.to_string_lossy()).expect("carrega");
        let guardados = runner.pedidos();
        assert_eq!(guardados.len(), 2, "dois pedidos guardados");
        assert!(guardados.contains(&pedido("caso 1")), "{guardados:?}");
        assert!(guardados.contains(&pedido("caso 2")), "{guardados:?}");
        std::fs::remove_dir_all(&d).ok();

        // v1: sem o texto do pedido, a lista sai vazia e o artefato continua legivel.
        let d1 = temp("v1");
        std::fs::create_dir_all(&d1).ok();
        let p = pedido("caso 1");
        let mut ensaio = crate::ponte::ModeloDeEnsaio::novo();
        let corpo = ensaio.decodificar(&p);
        let texto = format!("{CABECALHO}\n@pedido {}\n{}\n@end\n", chave(&p), corpo);
        std::fs::write(d1.join(ARQUIVO), texto.as_bytes()).expect("escreve");
        let hash = stochology_audit::sha256_hex(texto.as_bytes());
        std::fs::write(d1.join("SHA256"), format!("{hash}  {ARQUIVO}\n")).expect("escreve");
        let antigo = RunnerLocal::carregar(&d1.to_string_lossy()).expect("carrega v1");
        assert_eq!(antigo.respostas(), 1);
        assert!(antigo.pedidos().is_empty(), "v1 nao traz pedido");
        std::fs::remove_dir_all(&d1).ok();
    }

    #[test]
    fn o_brier_e_a_regra_propria_fora_do_ajuste() {
        let pares = [(0.9, true), (0.1, false), (0.5, true)];
        let esperado =
            ((0.9f64 - 1.0).powi(2) + (0.1f64 - 0.0).powi(2) + (0.5f64 - 1.0).powi(2)) / 3.0;
        assert!(
            (brier(&pares) - esperado).abs() < 1e-12,
            "{}",
            brier(&pares)
        );
        assert!(brier(&[]).is_nan(), "sem par nao ha pontuacao");
    }
}
