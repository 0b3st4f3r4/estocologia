//! A ponte Jev-LLM: o arcabouço da ponte, com os portões que fecham sem os pesos.
//!
//! O item 13 pede o runner local de pesos congelados com o \`sha256\` dos pesos, a decodificação restrita ao esquema
//! da ficha, a prova de determinismo na forma do G1, a calibração contra o Brier do Jev determinístico, o custo por
//! chamada e a pergunta paralela de adequação do espaço de hipóteses. **O runner local existe, em `runner.rs`**; o
//! que não existe nesta árvore são os pesos do modelo, e é por isso que o portão dos pesos segue recusando. Este
//! módulo entrega o arcabouço com os portões que dependem do modelo, mais os que já fecham:
//!
//! - **o portão dos pesos** — sem o arquivo e sem o \`sha256\` batendo, a ponte **não roda**, e nenhum número sai;
//! - **a decodificação restrita** — a saída tem de ser o documento canônico da ficha, e a margem não pode encostar em
//!   0% nem em 100%: fora disso, a ponte **abstém** (\`Categoria::Outros\`, a abstenção que já existe em \`s1.rs\`);
//! - **a prova de determinismo** — duas execuções e o mesmo \`sha256\`, na forma do G1; e a prova **reprova** um
//!   modelo que muda de resposta, que é exatamente o que \`INTEGRACAO_JEV.md\` declara sobre o LLM livre;
//! - **o custo por chamada** e a **adequação do espaço de hipóteses** — medidos no modelo de ensaio, e declarados como
//!   regra para quando os pesos existirem.
//!
//! Rodar: ~labs --jev~

use stochology_core::s1::Categoria;
use std::time::Instant;

/// O que a ponte precisa de um modelo congelado.
pub trait ModeloCongelado {
    fn nome(&self) -> &str;
    /// Decodifica um pedido no texto canônico da ficha.
    fn decodificar(&mut self, pedido: &str) -> String;
    /// Os tokens gastos na última chamada.
    fn tokens(&self) -> usize;
}

/// O pedido que a ponte manda ao modelo: a pergunta do decisor, no esquema.
pub fn pedido(pergunta: &str) -> String {
    format!("responda no esquema canonico da ficha, sem texto livre: {pergunta}")
}

/// Por que a ponte recusa uma saída ou uma execução.
#[derive(Debug, Clone, PartialEq)]
pub enum Recusa {
    /// Sem o arquivo de pesos, a ponte não roda.
    PesosAusentes(String),
    /// Com o hash diferente do declarado, a ponte não roda.
    PesosComHashDiferente { esperado: String, medido: String },
    /// O operador não declarou o hash antes de rodar.
    HashNaoDeclarado(String),
    /// O artefato existe e o hash bate, e o formato não é o declarado.
    ArtefatoIlegivel(String),
    /// A saída não é o documento canônico da ficha.
    ForaDoEsquema(String),
    /// A saída fere a proibição dos extremos dogmáticos.
    ExtremoDogmatico(String),
}

impl Recusa {
    /// A abstenção declarada: o "não sei" auditável que já existe no Sistema 1.
    pub fn abstracao(&self) -> Categoria {
        Categoria::Outros
    }
}

/// O portão dos pesos: o arquivo tem de existir e o \`sha256\` tem de bater com o declarado.
pub fn verificar_pesos(caminho: &str, esperado: &str) -> Result<String, Recusa> {
    let dados = std::fs::read(caminho).map_err(|_| Recusa::PesosAusentes(caminho.to_string()))?;
    let medido = stochology_audit::sha256_hex(&dados);
    let esperado = esperado.trim().to_lowercase();
    if medido != esperado {
        return Err(Recusa::PesosComHashDiferente { esperado, medido });
    }
    Ok(medido)
}

/// A decodificação restrita: a saída tem de ser o documento canônico da ficha, e a margem não pode encostar nos
/// extremos. Fora disso, a ponte abstém em vez de aceitar texto livre.
pub fn validar_saida(saida: &str) -> Result<stochology_core::Ficha, Recusa> {
    let ficha = stochology_schema::de_texto(saida).map_err(|e| Recusa::ForaDoEsquema(format!("{e:?}")))?;
    if let Some(m) = ficha.margem {
        if !(0.0 < m && m < 1.0) {
            return Err(Recusa::ExtremoDogmatico(format!("margem {m}")));
        }
    }
    Ok(ficha)
}

/// A contabilidade da ponte: chamadas, tokens e tempo, com o custo por chamada.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Contabilidade {
    pub chamadas: usize,
    pub tokens: usize,
    pub ms: u128,
}

impl Contabilidade {
    /// Tokens e milissegundos por chamada.
    pub fn por_chamada(&self) -> (f64, f64) {
        let n = self.chamadas.max(1) as f64;
        (self.tokens as f64 / n, self.ms as f64 / n)
    }
}

/// A prova de determinismo na forma do G1: duas execuções, o mesmo \`sha256\`.
pub fn provar_determinismo(
    modelo: &mut dyn ModeloCongelado,
    pedidos: &[String],
    contabilidade: &mut Contabilidade,
) -> (String, String, bool) {
    let correr = |modelo: &mut dyn ModeloCongelado, contabilidade: &mut Contabilidade| -> String {
        let mut saidas = Vec::new();
        for p in pedidos {
            let t0 = Instant::now();
            let s = modelo.decodificar(p);
            contabilidade.chamadas += 1;
            contabilidade.tokens += modelo.tokens();
            contabilidade.ms += t0.elapsed().as_millis();
            saidas.push(s);
        }
        stochology_audit::sha256_hex(saidas.join("\n").as_bytes())
    };
    let a = correr(modelo, contabilidade);
    let b = correr(modelo, contabilidade);
    let iguais = a == b;
    (a, b, iguais)
}

/// A fração de abstenções acima da qual o espaço de hipóteses não serve ao modelo.
pub const ABSTENCOES_MAXIMAS: f64 = 0.30;

/// A pergunta paralela de adequação do espaço de hipóteses.
#[derive(Debug, Clone, PartialEq)]
pub struct Adequacao {
    pub pedidos: usize,
    pub abstencoes: usize,
    pub fracao: f64,
    pub veredito: String,
}

/// Mede a adequação: quantos pedidos a ponte teve de abster, e o veredito declarado antes de rodar.
pub fn adequacao_do_espaco(
    modelo: &mut dyn ModeloCongelado,
    pedidos: &[String],
    contabilidade: &mut Contabilidade,
) -> Adequacao {
    let mut abstencoes = 0usize;
    for p in pedidos {
        let t0 = Instant::now();
        let saida = modelo.decodificar(p);
        contabilidade.chamadas += 1;
        contabilidade.tokens += modelo.tokens();
        contabilidade.ms += t0.elapsed().as_millis();
        if validar_saida(&saida).is_err() {
            abstencoes += 1;
        }
    }
    let fracao = abstencoes as f64 / pedidos.len().max(1) as f64;
    let veredito = if fracao > ABSTENCOES_MAXIMAS {
        format!(
            "o esquema da ficha NAO serve a este modelo: {:.1}% de abstencoes, acima do limite declarado de {:.0}%",
            fracao * 100.0,
            ABSTENCOES_MAXIMAS * 100.0
        )
    } else {
        format!(
            "o esquema da ficha serve a este modelo: {:.1}% de abstencoes, dentro do limite declarado de {:.0}%",
            fracao * 100.0,
            ABSTENCOES_MAXIMAS * 100.0
        )
    };
    Adequacao { pedidos: pedidos.len(), abstencoes, fracao, veredito }
}

/// Um modelo de ensaio: determinístico, responde no esquema da ficha.
pub struct ModeloDeEnsaio {
    pub tokens_por_chamada: usize,
}

impl ModeloDeEnsaio {
    pub fn novo() -> Self {
        ModeloDeEnsaio { tokens_por_chamada: 42 }
    }
}

impl ModeloCongelado for ModeloDeEnsaio {
    fn nome(&self) -> &str {
        "modelo de ensaio (deterministico)"
    }
    fn decodificar(&mut self, pedido: &str) -> String {
        let ficha = stochology_core::Ficha::nova("jev.ponte.1", stochology_core::Camada::Modelo, stochology_core::TipoIncerteza::Epistemica)
            .completa()
            .com_cuidados();
        let mut ficha = ficha;
        ficha.definicao = pedido.to_string();
        ficha.pergunta_do_decisor = "a ponte responde?".into();
        ficha.acao_do_pacote = "abster quando a saida sai do esquema".into();
        stochology_schema::texto(&ficha)
    }
    fn tokens(&self) -> usize {
        self.tokens_por_chamada
    }
}

/// Um modelo de ensaio instável: muda de resposta a cada chamada — é o LLM livre do texto, que não fecha o G1.
pub struct ModeloDeEnsaioInstavel {
    contador: usize,
}

impl ModeloDeEnsaioInstavel {
    pub fn novo() -> Self {
        ModeloDeEnsaioInstavel { contador: 0 }
    }
}

impl ModeloCongelado for ModeloDeEnsaioInstavel {
    fn nome(&self) -> &str {
        "modelo de ensaio instavel (o LLM livre)"
    }
    fn decodificar(&mut self, pedido: &str) -> String {
        self.contador += 1;
        format!("{pedido} — tentativa {} (texto livre)", self.contador)
    }
    fn tokens(&self) -> usize {
        7
    }
}

/// O laboratório: os portões da ponte, com o que fecha sem os pesos e o que só fecha com eles.
pub fn rodar() {
    println!("== Ponto 20 :: a ponte Jev-LLM ==");

    println!("\n-- o portao dos pesos --");
    let caminho = "fundo/p20-jev/pesos/modelo.congelado";
    match verificar_pesos(caminho, "0000000000000000000000000000000000000000000000000000000000000000") {
        Ok(sha) => println!("  pesos verificados: {sha}"),
        Err(recusa) => match &recusa {
            Recusa::PesosAusentes(c) => println!("  RECUSADO: pesos ausentes em {c} | abstencao {:?} | sem os pesos, a ponte nao roda e nenhum numero sai", recusa.abstracao()),
            Recusa::PesosComHashDiferente { esperado, medido } => println!("  RECUSADO: hash declarado {esperado}, medido {medido}"),
            outra => println!("  RECUSADO: {outra:?}"),
        },
    }

    println!("\n-- a decodificacao restrita --");
    let mut ensaio = ModeloDeEnsaio::novo();
    let p = pedido("o posto de saude aguenta a demanda?");
    let saida = ensaio.decodificar(&p);
    match validar_saida(&saida) {
        Ok(ficha) => println!("  o modelo de ensaio responde no esquema: ficha {} | camada {} | palavras do documento: {}", ficha.id, ficha.camada.codigo(), saida.split_whitespace().count()),
        Err(r) => println!("  a saida do ensaio nao passou: {r:?}"),
    }
    let fora = "isto e texto livre, nao uma ficha";
    match validar_saida(fora) {
        Ok(_) => println!("  PROBLEMA: texto livre aceito"),
        Err(r) => println!("  texto livre recusado: {r:?} | abstencao {:?}", r.abstracao()),
    }
    let extremo: Vec<String> = saida
        .lines()
        .filter(|l| !l.starts_with("margem"))
        .map(|l| l.to_string())
        .chain(std::iter::once("margem = 1.0".to_string()))
        .collect();
    match validar_saida(&extremo.join("\n")) {
        Ok(_) => println!("  PROBLEMA: margem em 100% aceita"),
        Err(r) => println!("  margem em 100% recusada: {r:?} | abstencao {:?}", r.abstracao()),
    }

    println!("\n-- a prova de determinismo, na forma do G1 --");
    let pedidos: Vec<String> = (1..=4).map(|i| pedido(&format!("caso {i}"))).collect();
    let mut contabilidade = Contabilidade::default();
    let (a, b, iguais) = provar_determinismo(&mut ModeloDeEnsaio::novo(), &pedidos, &mut contabilidade);
    println!("  modelo de ensaio: execucao 1 {a:.16} | execucao 2 {b:.16} | identicas: {iguais}");
    let mut contabilidade2 = Contabilidade::default();
    let (c, d, iguais2) = provar_determinismo(&mut ModeloDeEnsaioInstavel::novo(), &pedidos, &mut contabilidade2);
    println!("  modelo instavel: execucao 1 {c:.16} | execucao 2 {d:.16} | identicas: {iguais2}");
    println!("  leitura: o portao G1 fecha com um modelo congelado e NAO fecha com um modelo que muda de resposta — e o que o INTEGRACAO_JEV.md declara sobre o LLM livre");

    println!("\n-- o custo por chamada e a adequacao do espaco --");
    let (tokens, ms) = contabilidade.por_chamada();
    println!("  modelo de ensaio: {} chamadas | {tokens:.1} tokens por chamada | {ms:.1} ms por chamada", contabilidade.chamadas);
    let mut contabilidade3 = Contabilidade::default();
    let ad = adequacao_do_espaco(&mut ModeloDeEnsaioInstavel::novo(), &pedidos, &mut contabilidade3);
    println!("  modelo instavel: {} pedidos | {} abstencoes ({:.1}%) | {}", ad.pedidos, ad.abstencoes, ad.fracao * 100.0, ad.veredito);
    let mut contabilidade4 = Contabilidade::default();
    let ad2 = adequacao_do_espaco(&mut ModeloDeEnsaio::novo(), &pedidos, &mut contabilidade4);
    println!("  modelo de ensaio: {} pedidos | {} abstencoes ({:.1}%) | {}", ad2.pedidos, ad2.abstencoes, ad2.fracao * 100.0, ad2.veredito);

    println!("\n-- o runner local dos pesos congelados --");
    match crate::runner::RunnerLocal::carregar(crate::runner::DIR_PADRAO) {
        Ok(mut runner) => {
            println!("  pesos verificados: {} | {} respostas congeladas no artefato", runner.sha256(), runner.respostas());
            let ps: Vec<String> = (1..=4).map(|i| pedido(&format!("caso {i}"))).collect();
            let mut c = Contabilidade::default();
            let (a, b, iguais) = provar_determinismo(&mut runner, &ps, &mut c);
            println!("  determinismo na forma do G1: {a:.16} contra {b:.16} | identicas: {iguais}");
            let mut c2 = Contabilidade::default();
            let ad = adequacao_do_espaco(&mut runner, &ps, &mut c2);
            let (tk, msq) = c2.por_chamada();
            println!("  adequacao: {} pedidos | {} abstencoes ({:.1}%) | {}", ad.pedidos, ad.abstencoes, ad.fracao * 100.0, ad.veredito);
            println!("  custo medido: {tk:.1} tokens por chamada | {msq:.1} ms por chamada");
            println!("  Brier: a regra esta em runner::brier e o conjunto rotulado em calibracao::carregar, com amostra minima e abstencao contada");
        }
        Err(recusa) => match &recusa {
            Recusa::HashNaoDeclarado(c) => println!("  RECUSADO: hash nao declarado em {c} | o operador declara o sha256 antes de rodar"),
            Recusa::PesosAusentes(c) => println!("  RECUSADO: pesos ausentes em {c} | sem os pesos, a ponte nao roda"),
            Recusa::PesosComHashDiferente { esperado, medido } => println!("  RECUSADO: hash declarado {esperado}, medido {medido}"),
            Recusa::ArtefatoIlegivel(m) => println!("  RECUSADO: artefato ilegivel: {m}"),
            outra => println!("  RECUSADO: {outra:?}"),
        },
    }
    println!("\n-- a calibracao: o Brier do modelo congelado --");
    match crate::calibracao::carregar(
        crate::calibracao::DIR_PADRAO,
        0,
        crate::calibracao::AMOSTRA_MINIMA,
    ) {
        Ok(cal) => println!("  {}", cal.leitura()),
        Err(recusa) => match &recusa {
            Recusa::HashNaoDeclarado(c) => println!("  sem conjunto rotulado: hash nao declarado em {c} | o Brier exige pares com procedencia declarada antes de rodar"),
            Recusa::PesosAusentes(c) => println!("  sem conjunto rotulado: {c} ausente | sem par (probabilidade, desfecho) nao ha calibracao, e o instrumento nao inventa uma"),
            outra => println!("  conjunto rotulado recusado: {outra:?}"),
        },
    }
    println!("  regra declarada: 0.0 e 1.0 recusam, id fora de ordem recusa, desfecho fora de 0 e 1 recusa, abaixo de {} pares o escore nao e publicado, abstencao sai do denominador", crate::calibracao::AMOSTRA_MINIMA);

    let dir = std::env::temp_dir().join(format!("p20-jev-ensaio-{}", std::process::id()));
    match crate::runner::escrever_artefato_de_ensaio(&dir, &["caso 1", "caso 2", "caso 3", "caso 4"]) {
        Ok(hash) => match crate::runner::RunnerLocal::carregar(&dir.to_string_lossy()) {
            Ok(mut r) => {
                let ps: Vec<String> = (1..=4).map(|i| pedido(&format!("caso {i}"))).collect();
                let mut c = Contabilidade::default();
                let (_, _, iguais) = provar_determinismo(&mut r, &ps, &mut c);
                let mut c2 = Contabilidade::default();
                let ad = adequacao_do_espaco(&mut r, &ps, &mut c2);
                let (tk, msq) = c2.por_chamada();
                println!("  ensaio do runner (nao sao os pesos reais): {hash:.16} | {} respostas | G1 identicas: {iguais} | abstencoes {:.1}% | {tk:.1} tokens por chamada | {msq:.1} ms por chamada", r.respostas(), ad.fracao * 100.0);
                println!("  leitura: o caminho artefato -> hash declarado -> tabela congelada -> portao roda de ponta a ponta; o que falta e o arquivo do modelo");
            }
            Err(e) => println!("  ensaio do runner: o artefato de ensaio nao carregou: {e:?}"),
        },
        Err(e) => println!("  ensaio do runner: nao foi possivel escrever o artefato de ensaio: {e}"),
    }
    std::fs::remove_dir_all(&dir).ok();

    println!("\n-- o estado declarado --");
    println!("  runner local: existe em crates/stochology-s2/src/runner.rs, com artefato, hash declarado antes da execucao e abstencao por pedido sem resposta");
    println!("  pesos congelados: ausentes da arvore | o operador declara o sha256 em fundo/p20-jev/pesos/SHA256 e deixa o modelo em fundo/p20-jev/pesos/modelo.congelado; ate la, o portao recusa e nenhum numero de calibracao real e publicado");
    println!("  o que ja fecha: o portao dos pesos, a decodificacao restrita com abstencao, a prova de determinismo, o custo por chamada, a regra de adequacao do espaco e o runner que liga todos eles");
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn sem_pesos_verificados_a_ponte_nao_roda() {
        let r = verificar_pesos("fundo/p20-jev/pesos/nao-existe.bin", "abc");
        match r {
            Err(Recusa::PesosAusentes(c)) => assert!(c.contains("nao-existe")),
            outro => panic!("esperava pesos ausentes: {outro:?}"),
        }
        assert_eq!(verificar_pesos("fundo/p20-jev/pesos/nao-existe.bin", "abc").unwrap_err().abstracao(), Categoria::Outros);
    }

    #[test]
    fn pesos_com_hash_diferente_recusam() {
        let caminho = std::env::temp_dir().join("pesos_de_ensaio.bin");
        std::fs::write(&caminho, b"pesos de ensaio").expect("escreve");
        let esperado = stochology_audit::sha256_hex(b"outra coisa qualquer");
        let r = verificar_pesos(&caminho.to_string_lossy(), &esperado);
        assert!(matches!(r, Err(Recusa::PesosComHashDiferente { .. })), "{r:?}");
        let certo = stochology_audit::sha256_hex(b"pesos de ensaio");
        assert!(verificar_pesos(&caminho.to_string_lossy(), &certo).is_ok(), "com o hash certo, passa");
        std::fs::remove_file(&caminho).ok();
    }

    #[test]
    fn a_saida_fora_do_esquema_vira_abstencao() {
        let r = validar_saida("isto e texto livre, nao uma ficha");
        assert!(matches!(r, Err(Recusa::ForaDoEsquema(_))), "{r:?}");
        assert_eq!(r.unwrap_err().abstracao(), Categoria::Outros, "a abstencao e o \"nao sei\" auditavel");
    }

    #[test]
    fn a_margem_no_extremo_e_recusada() {
        let mut ensaio = ModeloDeEnsaio::novo();
        let limpa = ensaio.decodificar(&pedido("caso"));
        assert!(limpa.contains("margem = "), "a ficha de ensaio declara margem");
        let com_extremo: Vec<String> = limpa
            .lines()
            .filter(|l| !l.starts_with("margem"))
            .map(|l| l.to_string())
            .chain(std::iter::once("margem = 1.0".to_string()))
            .collect();
        let recusa = validar_saida(&com_extremo.join("\n")).expect_err("a margem no extremo tinha de ser recusada");
        assert!(matches!(recusa, Recusa::ExtremoDogmatico(_)), "{recusa:?}");
        assert_eq!(recusa.abstracao(), Categoria::Outros);
    }

    #[test]
    fn a_prova_de_determinismo_passa_no_modelo_de_ensaio() {
        let mut contabilidade = Contabilidade::default();
        let pedidos: Vec<String> = (1..=3).map(|i| pedido(&format!("caso {i}"))).collect();
        let (a, b, iguais) = provar_determinismo(&mut ModeloDeEnsaio::novo(), &pedidos, &mut contabilidade);
        assert!(iguais, "o modelo congelado fecha o G1: {a} contra {b}");
        assert_eq!(contabilidade.chamadas, 6, "duas execucoes de tres pedidos");
    }

    #[test]
    fn a_prova_de_determinismo_reprova_o_modelo_instavel() {
        let mut contabilidade = Contabilidade::default();
        let pedidos: Vec<String> = (1..=3).map(|i| pedido(&format!("caso {i}"))).collect();
        let (a, b, iguais) = provar_determinismo(&mut ModeloDeEnsaioInstavel::novo(), &pedidos, &mut contabilidade);
        assert!(!iguais, "o modelo que muda de resposta NAO fecha o G1: {a} contra {b}");
    }

    #[test]
    fn o_custo_por_chamada_e_a_adequacao_saem_do_ensaio() {
        let mut contabilidade = Contabilidade::default();
        let pedidos: Vec<String> = (1..=4).map(|i| pedido(&format!("caso {i}"))).collect();
        let ad = adequacao_do_espaco(&mut ModeloDeEnsaio::novo(), &pedidos, &mut contabilidade);
        assert_eq!(ad.abstencoes, 0, "o modelo de ensaio responde no esquema");
        assert!(ad.veredito.contains("serve a este modelo"), "{}", ad.veredito);
        let (tokens, _) = contabilidade.por_chamada();
        assert!((tokens - 42.0).abs() < 1e-9, "tokens por chamada: {tokens}");
        let mut contabilidade2 = Contabilidade::default();
        let ad2 = adequacao_do_espaco(&mut ModeloDeEnsaioInstavel::novo(), &pedidos, &mut contabilidade2);
        assert_eq!(ad2.abstencoes, 4, "o modelo instavel abstem em todos");
        assert!(ad2.veredito.contains("NAO serve"), "{}", ad2.veredito);
    }
}
