//! Os ensaios sinteticos dos tres protocolos humanos: o modelo nulo de cada um.
//!
//! Os itens 3, 5 e 9 de PROPOSTAS_DECLARADAS.md dependem de gente, e sessao de agente nao ocupa o lugar de
//! ninguem. O que ela pode fazer e responder outra pergunta: **quanto tira um respondente fluente e sem
//! calibracao no mesmo instrumento?** Sem esse piso, especialista real nao tem contra o que ser comparado, e o
//! protocolo pode estar medindo fluencia em vez de conhecimento.
//!
//! Tres ensaios, todos com entrada sintetica e com o rotulo na propria saida:
//!
//! - shelf: quatro respondentes declarados, pontuados nos itens reais do pre-teste, com Brier, habilidade e
//!   ECE pelo mesmo avaliador que o painel usa;
//! - escuta: a aritmetica da cobertura, e o limite do portao, que confere campo e **nao** proveniencia;
//! - ponte: tres registros sinteticos, e o portao so abre com papeis designados, casos no minimo e status
//!   compativel com o teste justo por fazer.
//!
//! Regra dura: nenhuma saida daqui conta cobertura. Os ensaios nao escrevem em fundo/ e nao selam nada.

use std::path::{Path, PathBuf};

use stochology_h3::preteste::{PreTeste, avaliar};

use crate::escuta;
use crate::ponte;

/// Os cinco conjuntos de itens do pre-teste, com as resolucoes que o mundo ja deu.
pub const ITENS: [(&str, &str); 5] = [
    (
        "desenvolvimento_preston",
        "fundo/p12-shelf/preteste/desenvolvimento_preston.yaml",
    ),
    (
        "direito_prisoes",
        "fundo/p12-shelf/preteste/direito_prisoes.yaml",
    ),
    (
        "direito_teto_beneficio",
        "fundo/p12-shelf/preteste/direito_teto_beneficio.yaml",
    ),
    (
        "educacao_star",
        "fundo/p12-shelf/preteste/educacao_star.yaml",
    ),
    ("h1_real_c", "fundo/p12-shelf/preteste/h1_real_c.yaml"),
];

/// Raiz do repositorio, a mesma convencao dos instrumentos que leem dados-reais/.
pub fn raiz() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("raiz do repositorio")
        .to_path_buf()
}

/// Um respondente sintetico, com a regra de resposta declarada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Respondente {
    /// p = 0,50 para todo item: o nulo declarado do protocolo.
    Moeda,
    /// p = 0,75 constante: fala com confianca e nao tem informacao.
    Fluente,
    /// p = 0,90 constante: o vies de sim, que parece bom quando o pool e desequilibrado.
    ViesDeSim,
    /// Sabe 80% dos itens (p = 0,85, acerta) e chuta os outros 20% (p = 0,50): o especialista plausivel.
    Especialista,
    /// Acerta sempre com p = 0,99: o teto, que o avaliador le como habilidade 0 por 0/0.
    Oraculo,
}

impl Respondente {
    pub const TODOS: [Respondente; 5] = [
        Respondente::Moeda,
        Respondente::Fluente,
        Respondente::ViesDeSim,
        Respondente::Especialista,
        Respondente::Oraculo,
    ];

    pub fn nome(self) -> &'static str {
        match self {
            Respondente::Moeda => "moeda p=0.50",
            Respondente::Fluente => "fluente p=0.75",
            Respondente::ViesDeSim => "vies-de-sim p=0.90",
            Respondente::Especialista => "especialista p=0.85/0.50",
            Respondente::Oraculo => "oraculo p=0.99",
        }
    }

    /// A resposta que o respondente da ao item, com a confianca que declara nela.
    ///
    /// Moeda e Fluente chutam sim ou nao sem olhar a resolucao, e por isso acertam cerca de metade das vezes;
    /// o que os separa e a confianca declarada. ViesDeSim responde sim sempre. Oraculo responde a resolucao.
    pub fn responde_item(self, resolucao: bool, rng: &mut stochology_core::Rng) -> (f64, bool) {
        match self {
            Respondente::Moeda => {
                let resposta = rng.uniforme(2) == 1;
                (0.50, resposta == resolucao)
            }
            Respondente::Fluente => {
                let resposta = rng.uniforme(2) == 1;
                (0.75, resposta == resolucao)
            }
            Respondente::ViesDeSim => (0.90, resolucao),
            Respondente::Especialista => {
                if rng.uniforme(100) < 80 {
                    (0.85, true)
                } else {
                    let resposta = rng.uniforme(2) == 1;
                    (0.50, resposta == resolucao)
                }
            }
            Respondente::Oraculo => (0.99, true),
        }
    }

    /// Responde o pool inteiro: a confianca declarada e o acerto de cada item.
    pub fn responde(self, resolucoes: &[bool], semente: u64) -> (Vec<f64>, Vec<bool>) {
        let mut rng = stochology_core::Rng::novo(semente);
        let mut confianca = Vec::with_capacity(resolucoes.len());
        let mut acerto = Vec::with_capacity(resolucoes.len());
        for &sim in resolucoes {
            let (p, acertou) = self.responde_item(sim, &mut rng);
            confianca.push(p);
            acerto.push(acertou);
        }
        (confianca, acerto)
    }
}

/// Le as resolucoes de um arquivo de itens: uma linha "resolucao: sim|nao" por item.
pub fn ler_resolucoes(texto: &str) -> Vec<bool> {
    let mut resolucoes = Vec::new();
    for linha in texto.lines() {
        let l = linha.trim();
        if let Some(valor) = l.strip_prefix("resolucao:") {
            match valor.trim() {
                "sim" => resolucoes.push(true),
                "nao" => resolucoes.push(false),
                _ => {}
            }
        }
    }
    resolucoes
}

fn ler(caminho: &Path) -> Result<String, String> {
    std::fs::read_to_string(caminho).map_err(|e| format!("{}: {e}", caminho.display()))
}

/// O ensaio do painel SHELF: os quatro respondentes nos cinco pools reais.
pub fn shelf() -> Result<String, String> {
    let raiz = raiz();
    let mut saida = String::new();
    saida.push_str("== ensaio sintetico :: o painel SHELF, com respondente declarado ==\n");
    saida.push_str(
        "rotulo: SINTETICO. Nenhuma linha abaixo conta cobertura de painel; o que se mede e o piso de um\n\
         respondente fluente e sem calibracao, para o painel real ter contra o que ser comparado.\n\n",
    );
    saida.push_str(&format!(
        "{:<22} {:<26} {:>3} {:>8} {:>8} {:>8} {:>9}\n",
        "respondente", "pool de itens", "n", "brier", "habilid.", "ece", "aprovado"
    ));

    let mut total_confianca: Vec<Vec<f64>> = vec![Vec::new(); Respondente::TODOS.len()];
    let mut total_acerto: Vec<Vec<bool>> = vec![Vec::new(); Respondente::TODOS.len()];

    for (indice, (nome, caminho)) in ITENS.iter().enumerate() {
        let texto = ler(&raiz.join(caminho))?;
        let resolucoes = ler_resolucoes(&texto);
        if resolucoes.is_empty() {
            return Err(format!("pool sem itens: {caminho}"));
        }
        for (r, respondente) in Respondente::TODOS.iter().enumerate() {
            let semente = 0x5EED_0000 + indice as u64 * 16 + r as u64;
            let (confianca, acerto) = respondente.responde(&resolucoes, semente);
            let avaliado = avaliar(&confianca, &acerto);
            total_confianca[r].extend_from_slice(&confianca);
            total_acerto[r].extend_from_slice(&acerto);
            saida.push_str(&format!(
                "{:<22} {:<26} {:>3} {:>8.4} {:>8.4} {:>8.4} {:>9}\n",
                respondente.nome(),
                nome,
                avaliado.n,
                avaliado.brier,
                avaliado.habilidade,
                avaliado.ece,
                if avaliado.aprovado { "sim" } else { "nao" }
            ));
        }
    }

    saida.push_str(&format!(
        "\n{:<22} {:<26} {:>3} {:>8} {:>8} {:>8} {:>9}\n",
        "respondente", "todos os pools", "n", "brier", "habilid.", "ece", "aprovado"
    ));
    for (r, respondente) in Respondente::TODOS.iter().enumerate() {
        let avaliado: PreTeste = avaliar(&total_confianca[r], &total_acerto[r]);
        saida.push_str(&format!(
            "{:<22} {:<26} {:>3} {:>8.4} {:>8.4} {:>8.4} {:>9}\n",
            respondente.nome(),
            "todos os pools",
            avaliado.n,
            avaliado.brier,
            avaliado.habilidade,
            avaliado.ece,
            if avaliado.aprovado { "sim" } else { "nao" }
        ));
    }
    saida.push_str(
        "\nLeitura: o respondente fluente declara 0,75 sem saber nada. Se ele passa no mesmo limiar que o\n\
         painel usa, o limiar mede confianca declarada e nao conhecimento. O especialista mostra o piso que um\n\
         painel real precisa bater: habilidade positiva com 80% dos itens sabidos e o resto chutado.\n\
         Divergencia declarada: o oraculo perfeito aparece com habilidade 0 e reprovado, porque com acerto 1 a\n\
         variancia do clima e zero e a razao 0/0 e lida como 0 pela guarda do avaliador.\n",
    );
    Ok(saida)
}

/// Um registro sintetico de escuta com ouvidos linhas cobertas.
fn registro_com(ouvidos: usize, fabricado: bool) -> String {
    let mut texto = String::from(
        "# prazo\t2026-12-15\n# grupos_minimos\t2\n# conducao\tprotocolo cego\n# registros\t2026-10-16\ndecisao\tgrupos\ttitulares\touvidos\tregistro\tpode_emitir\n",
    );
    let decisoes = [
        "h2g/saida/direito_teto_beneficio",
        "h2g/saida/educacao_star_turmas",
        "h2g/saida/saude_rand",
        "h1-real/saida/b",
        "h3/prisoes",
        "h3/prisoes20",
        "h3/prisoes_ri",
    ];
    for (k, decisao) in decisoes.iter().enumerate() {
        if k < ouvidos {
            let quem = if fabricado {
                "equivalente estrutural sintetico"
            } else {
                "equivalente estrutural ouvido"
            };
            let arquivo = format!("transcricao-{:02}.md", k + 1);
            texto.push_str(&format!(
                "{decisao}\ttitulares\t1\t{quem}\t{arquivo}\tnao\n"
            ));
        } else {
            texto.push_str(&format!("{decisao}\ttitulares\t1\tnenhum\t-\tnao\n"));
        }
    }
    texto
}

/// O ensaio da escuta: a aritmetica da cobertura e o limite do portao.
pub fn escuta() -> Result<String, String> {
    let real = escuta::ler(&ler(&raiz().join(escuta::REGISTRO))?)?;
    let veredito = real.veredito();
    let mut saida = String::new();
    saida.push_str("== ensaio sintetico :: a escuta de quem paga o erro ==\n");
    saida.push_str(
        "rotulo: SINTETICO. Os registros abaixo sao construidos no codigo; nenhum conta cobertura.\n\n",
    );
    saida.push_str(&format!(
        "registro real: {} decisoes | cobertura {} de {} | minimo {} | {}\n",
        real.linhas.len(),
        veredito.cobertura,
        real.linhas.len(),
        real.grupos_minimos,
        if veredito.lacrado {
            "CAIXA LACRADA"
        } else {
            "CAIXA ABERTA"
        }
    ));

    saida.push_str("\naritmetica da cobertura, com registro sintetico:\n");
    for k in 0..=3usize {
        let registro = escuta::ler(&registro_com(k, false))?;
        let v = registro.veredito();
        saida.push_str(&format!(
            "  {} grupo(s) ouvido(s) | cobertura {} de {} | minimo {} | {}\n",
            k,
            v.cobertura,
            registro.linhas.len(),
            registro.grupos_minimos,
            if v.lacrado {
                "CAIXA LACRADA"
            } else {
                "CAIXA ABERTA"
            }
        ));
    }

    saida.push_str("\n-- o limite do portao: ele confere campo, nao proveniencia --\n");
    let fabricado = escuta::ler(&registro_com(7, true))?;
    let vf = fabricado.veredito();
    saida.push_str(&format!(
        "  sete registros fabricados, com nome plausivel e arquivo de transcricao: cobertura {} de {} | {}\n",
        vf.cobertura,
        fabricado.linhas.len(),
        if vf.lacrado { "CAIXA LACRADA" } else { "CAIXA ABERTA" }
    ));
    saida.push_str(
        "  leitura: o portao nao distingue transcricao assinada de transcricao inventada, porque so olha os\n\
         campos. A cobertura real precisa de proveniencia -- hash da transcricao assinada, encadeado no\n\
         registro de governanca -- e isso nao existe hoje. Fica declarado como o proximo campo do registro:\n\
         hash_da_transcricao, conferido como os lacres, e nao um novo carimbo de confianca.\n",
    );

    saida.push_str("\n-- o roteiro congelado contra o vocabulario proibido --\n");
    let roteiro = [
        "Qual e o seu pior cenario que ainda e plausivel?",
        "O que faria voce recusar essa intervencao?",
        "Quando essa politica falha, quem de fato arca com o custo?",
    ];
    let proibido = [
        "quantil", "beta", "brier", "evpi", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8",
    ];
    let mut vazamentos = 0;
    for pergunta in roteiro {
        let baixa = pergunta.to_lowercase();
        let achados: Vec<&str> = proibido
            .iter()
            .copied()
            .filter(|p| baixa.contains(p))
            .collect();
        saida.push_str(&format!(
            "  {:<70} {}\n",
            pergunta,
            if achados.is_empty() {
                "sem vazamento".to_string()
            } else {
                format!("VAZA: {achados:?}")
            }
        ));
        if !achados.is_empty() {
            vazamentos += 1;
        }
    }
    saida.push_str(&format!(
        "  perguntas com vazamento de vocabulario: {} de {}\n",
        vazamentos,
        roteiro.len()
    ));
    Ok(saida)
}

const PONTE_CABECALHO: &str = "# registros\t2026-10-16\n# prazo\t2027-01-14\n# casos_minimos\t12\n# veredito_se_o_prazo_vencer\tnao sustentada em definitivo\npapel\texigencia\tquem\testado\n";

/// Um registro sintetico da ponte: papeis designados, casos feitos e status declarado.
fn registro_da_ponte(papeis_designados: usize, casos: usize, status: &str) -> String {
    let mut texto = String::from(PONTE_CABECALHO);
    let papeis = [
        ("escolhedor_dos_casos", "de fora do programa"),
        ("analistas_braco_A", "dois humanos"),
        ("analistas_braco_B", "dois humanos"),
        ("juiz", "humano de fora, cego ao braco"),
    ];
    for (k, (papel, exigencia)) in papeis.iter().enumerate() {
        let quem = if k < papeis_designados {
            "pessoa-sintetica"
        } else {
            "-"
        };
        let estado = if k < papeis_designados {
            "designado"
        } else {
            "nao designado"
        };
        texto.push_str(&format!("{papel}\t{exigencia}\t{quem}\t{estado}\n"));
    }
    texto.push_str(&format!("casos\tcasos mais dificeis\t-\t{casos} de 12\n"));
    texto.push_str(&format!(
        "status_da_tese\tnao sustentada ate o teste justo\t-\t{status}\n"
    ));
    texto
}

/// Quantos dos quatro papeis estao designados no veredito.
fn papeis_designados_contados(designados: &[String]) -> usize {
    designados
        .iter()
        .filter(|p| !p.starts_with("casos"))
        .count()
}

/// O ensaio da ponte: quatro registros sinteticos e o portao que so abre com o teste justo.
pub fn ponte() -> Result<String, String> {
    let real = ponte::ler_ponte(&ler(&raiz().join(ponte::REGISTRO))?)?;
    let vr = real.veredito();
    let mut saida = String::new();
    saida.push_str("== ensaio sintetico :: a tese da ponte em escala ==\n");
    saida.push_str(
        "rotulo: SINTETICO. Nenhum registro abaixo conta caso escolhido nem papel designado.\n\n",
    );
    saida.push_str(&format!(
        "registro real: {} papeis | pendentes {:?} | casos faltando {} | status \"{}\" | {}\n",
        real.papeis.len(),
        vr.papeis_pendentes,
        vr.casos_faltando,
        vr.status_declarado,
        if vr.caixa_fechada {
            "TESE NAO SUSTENTADA"
        } else {
            "CAIXA ABERTA"
        }
    ));
    let casos = [
        (
            0usize,
            0usize,
            "nao sustentada",
            "nenhum papel, nenhum caso",
        ),
        (4, 0, "em teste", "papeis completos, casos faltando"),
        (4, 12, "em teste", "teste justo montado"),
        (0, 0, "sustentada", "status forjado sem o teste justo"),
    ];
    saida.push_str("\nregistros sinteticos:\n");
    for (papeis, casos_feitos, status, rotulo) in casos {
        let registro = ponte::ler_ponte(&registro_da_ponte(papeis, casos_feitos, status))?;
        let v = registro.veredito();
        let divergencia = if v.divergencias.is_empty() {
            String::new()
        } else {
            format!(" | DIVERGENCIA: {}", v.divergencias.join("; "))
        };
        saida.push_str(&format!(
            "  {:<38} papeis {} de 4 | casos {:>2} de 12 | status \"{}\": {}{}\n",
            rotulo,
            papeis_designados_contados(&v.papeis_designados),
            casos_feitos,
            v.status_declarado,
            if v.caixa_fechada {
                "caixa fechada"
            } else {
                "caixa aberta"
            },
            divergencia
        ));
    }
    saida.push_str(
        "\nLeitura: o portao separa tres coisas que um parecer confunde -- papel preenchido, caso escolhido\n\
         e status declarado. Status \"sustentada\" com zero caso e reprovado, como o registro real ja reprova.\n",
    );
    Ok(saida)
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn moeda_nao_tem_habilidade() {
        let resolucoes: Vec<bool> = (0..40).map(|i| i % 2 == 0).collect();
        let (confianca, acerto) = Respondente::Moeda.responde(&resolucoes, 7);
        let avaliado = avaliar(&confianca, &acerto);
        // Chute com metade de acerto: a habilidade fica em torno de zero, nunca muito acima.
        assert!(
            avaliado.habilidade.abs() < 0.10,
            "habilidade {}",
            avaliado.habilidade
        );
    }

    #[test]
    fn fluente_e_vies_de_sim_ficam_abaixo_do_oraculo() {
        let resolucoes: Vec<bool> = (0..40).map(|i| i % 2 == 0).collect();
        let mut habilidades = Vec::new();
        for r in [
            Respondente::Fluente,
            Respondente::ViesDeSim,
            Respondente::Especialista,
        ] {
            let (confianca, acerto) = r.responde(&resolucoes, 11);
            habilidades.push(avaliar(&confianca, &acerto).habilidade);
        }
        assert!(
            habilidades[0] < habilidades[2],
            "fluente {} vs especialista {}",
            habilidades[0],
            habilidades[2]
        );
        assert!(
            habilidades[1] < habilidades[2],
            "vies {} vs especialista {}",
            habilidades[1],
            habilidades[2]
        );
    }

    #[test]
    fn os_cinco_pools_reais_tem_itens() {
        let raiz = raiz();
        for (nome, caminho) in ITENS {
            let texto = std::fs::read_to_string(raiz.join(caminho)).expect("pool do pre-teste");
            let resolucoes = ler_resolucoes(&texto);
            assert!(
                resolucoes.len() >= 10,
                "{nome} tem {} itens",
                resolucoes.len()
            );
        }
    }

    #[test]
    fn o_registro_real_esta_lacrado_e_o_sintetico_de_sete_abre() {
        let texto = std::fs::read_to_string(raiz().join(escuta::REGISTRO)).expect("registro");
        let real = escuta::ler(&texto).expect("leitura");
        assert!(real.veredito().lacrado);
        let fabricado = escuta::ler(&registro_com(7, true)).expect("sintetico");
        assert_eq!(fabricado.veredito().cobertura, 7);
        assert!(
            !fabricado.veredito().lacrado,
            "o portao confere campo, nao proveniencia: e o que este teste registra"
        );
    }

    #[test]
    fn a_ponte_sintetica_so_abre_com_tudo_e_reprova_status_forjado() {
        let vazio = ponte::ler_ponte(&registro_da_ponte(0, 0, "nao sustentada")).expect("vazio");
        assert!(vazio.veredito().caixa_fechada);
        let completo = ponte::ler_ponte(&registro_da_ponte(4, 12, "em teste")).expect("completo");
        assert!(!completo.veredito().caixa_fechada);
        assert!(completo.veredito().divergencias.is_empty());
        let forjado = ponte::ler_ponte(&registro_da_ponte(0, 0, "sustentada")).expect("forjado");
        assert!(!forjado.veredito().divergencias.is_empty());
    }

    #[test]
    fn os_tres_ensaios_rodam_e_se_rotulam_sinteticos() {
        for saida in [
            shelf().expect("shelf"),
            escuta().expect("escuta"),
            ponte().expect("ponte"),
        ] {
            assert!(saida.contains("SINTETICO"), "cada ensaio carrega o rotulo");
        }
    }
}
