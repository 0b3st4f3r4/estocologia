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

/// O que a ponte precisa de um modelo congelado.
pub trait ModeloCongelado {
    fn nome(&self) -> &str;
    /// Decodifica um pedido no texto canônico da ficha.
    fn decodificar(&mut self, pedido: &str) -> String;
    /// Os tokens gastos na última chamada.
    fn tokens(&self) -> usize;
    /// O custo de tempo declarado por chamada, em milissegundos (0 na tabela em memória, sem relógio).
    fn ms(&self) -> u128 {
        0
    }
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
            let s = modelo.decodificar(p);
            contabilidade.chamadas += 1;
            contabilidade.tokens += modelo.tokens();
            contabilidade.ms += modelo.ms();
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
        let saida = modelo.decodificar(p);
        contabilidade.chamadas += 1;
        contabilidade.tokens += modelo.tokens();
        contabilidade.ms += modelo.ms();
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
    let dir_pesos = crate::runner::DIR_PADRAO;
    let caminho = format!("{dir_pesos}/{}", crate::runner::ARQUIVO);
    match crate::runner::hash_declarado(dir_pesos) {
        Err(recusa) => println!(
            "  RECUSADO: {recusa:?} | o operador declara o sha256 em {dir_pesos}/SHA256 antes de rodar, e nenhum numero sai"
        ),
        Ok(declarado) => match verificar_pesos(&caminho, &declarado) {
            Ok(sha) => println!("  pesos verificados: {sha}"),
            Err(recusa) => match &recusa {
                Recusa::PesosComHashDiferente { esperado, medido } => {
                    println!("  RECUSADO: hash declarado {esperado}, medido {medido}")
                }
                outra => println!("  RECUSADO: {outra:?}"),
            },
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
            // A adequacao e o G1 sao medidos sobre os casos do proprio artefato quando ele os guarda (v2). Com
            // artefato v1, que so tem o hash, cai na sonda propria — e o instrumento diz qual dos dois usou.
            let do_artefato = runner.pedidos();
            let (ps, sonda) = if do_artefato.is_empty() {
                (
                    (1..=4).map(|i| pedido(&format!("caso {i}"))).collect::<Vec<String>>(),
                    "sonda propria do instrumento (artefato v1, sem o texto do pedido)",
                )
            } else {
                (do_artefato, "os casos do proprio artefato")
            };
            println!("  adequacao medida sobre: {sonda} ({} casos)", ps.len());
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
    let tem_pesos = crate::runner::RunnerLocal::carregar(crate::runner::DIR_PADRAO).is_ok();
    let cal = crate::calibracao::carregar(crate::calibracao::DIR_PADRAO, 0, crate::calibracao::AMOSTRA_MINIMA);
    let tem_escore = matches!(cal, Ok(crate::calibracao::Calibracao::Escore { .. }));
    if tem_pesos {
        println!("  pesos congelados: artefato verificado em {dir_pesos} | a ponte roda com ele, e o hash fica declarado antes de cada execucao");
    } else {
        println!("  pesos congelados: ausentes da arvore | o operador declara o sha256 em {dir_pesos}/SHA256 e deixa o modelo em {dir_pesos}/{}; ate la, o portao recusa e nenhum numero de calibracao real e publicado", crate::runner::ARQUIVO);
    }
    if tem_escore {
        println!("  conjunto rotulado: com escore publicado | o Brier acima e o do conjunto verificado, nao do ensaio");
    } else {
        println!("  conjunto rotulado: sem escore publicado | o leitor existe e o conjunto e o que falta");
    }
    println!("  o que ja fecha: o portao dos pesos, a decodificacao restrita com abstencao, a prova de determinismo, o custo por chamada, a regra de adequacao do espaco, o runner e o leitor do conjunto rotulado");
}


/// A medida do instrumento, em forma legivel por maquina.
///
/// O `labs --jev` imprime para humano; esta estrutura existe para o registro nao ser digitado a mao. Ela sai
/// como ficha (campo = valor), que e o formato que o repositorio ja usa, e volta a entrar por `de_ficha`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Medida {
    pub pesos: Option<String>,
    pub respostas: usize,
    pub pedidos: usize,
    pub sonda: String,
    pub abstencoes: usize,
    pub fracao_de_abstencao: f64,
    pub g1_identicas: bool,
    pub g1_hash: String,
    pub tokens_por_chamada: f64,
    pub ms_por_chamada: f64,
    pub n_pares: usize,
    pub brier: Option<f64>,
    pub ece: Option<f64>,
    pub amostra_minima: usize,
}

/// Mede o que o instrumento sabe medir, sem imprimir nada.
pub fn medir() -> Medida {
    let mut m = Medida {
        amostra_minima: crate::calibracao::AMOSTRA_MINIMA,
        sonda: String::from("nenhuma: sem artefato"),
        ..Medida::default()
    };
    if let Ok(mut runner) = crate::runner::RunnerLocal::carregar(crate::runner::DIR_PADRAO) {
        m.pesos = Some(runner.sha256().to_string());
        m.respostas = runner.respostas();
        let do_artefato = runner.pedidos();
        let (ps, sonda) = if do_artefato.is_empty() {
            (
                (1..=4).map(|i| pedido(&format!("caso {i}"))).collect::<Vec<String>>(),
                String::from("sonda propria (artefato v1)"),
            )
        } else {
            (do_artefato, String::from("casos do proprio artefato"))
        };
        m.pedidos = ps.len();
        m.sonda = sonda;
        let mut c = Contabilidade::default();
        let (a, _, iguais) = provar_determinismo(&mut runner, &ps, &mut c);
        m.g1_identicas = iguais;
        m.g1_hash = a;
        let mut c2 = Contabilidade::default();
        let ad = adequacao_do_espaco(&mut runner, &ps, &mut c2);
        m.abstencoes = ad.abstencoes;
        m.fracao_de_abstencao = ad.fracao;
        let (tk, msq) = c2.por_chamada();
        m.tokens_por_chamada = tk;
        m.ms_por_chamada = msq;
    }
    if let Ok(cal) = crate::calibracao::carregar(
        crate::calibracao::DIR_PADRAO,
        0,
        crate::calibracao::AMOSTRA_MINIMA,
    ) {
        if let crate::calibracao::Calibracao::Escore { n, brier, ece, .. } = cal {
            m.n_pares = n;
            m.brier = Some(brier);
            m.ece = Some(ece);
        }
    }
    m
}

/// Casas decimais declaradas por campo. A escrita e a leitura saem daqui; nao ha epsilon solto.
pub const CASAS_FRACAO: usize = 6;
pub const CASAS_ESCORE: usize = 6;
pub const CASAS_CUSTO: usize = 1;

/// Arredonda para as casas declaradas, para que a ficha seja exata na volta.
///
/// Sem isto, escrever `1.0 / 33.0` com seis casas e ler de volta da `0.030303`, que nao e o mesmo
/// `f64` (`0.030303030303030304`): o teste vira loteria de representacao e o registro ganha um epsilon
/// que ninguem declarou. Quantizando na borda, a comparacao volta a ser de igualdade.
pub fn quantizar(x: f64, casas: usize) -> f64 {
    if !x.is_finite() {
        return x;
    }
    let escala = 10f64.powi(casas as i32);
    (x * escala).round() / escala
}

impl Medida {
    /// A medida com todo campo numerico reduzido as casas que a ficha carrega.
    pub fn quantizada(mut self) -> Medida {
        self.fracao_de_abstencao = quantizar(self.fracao_de_abstencao, CASAS_FRACAO);
        self.tokens_por_chamada = quantizar(self.tokens_por_chamada, CASAS_CUSTO);
        self.ms_por_chamada = quantizar(self.ms_por_chamada, CASAS_CUSTO);
        self.brier = self.brier.map(|v| quantizar(v, CASAS_ESCORE));
        self.ece = self.ece.map(|v| quantizar(v, CASAS_ESCORE));
        self
    }
}

fn escore(v: Option<f64>, casas: usize) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{:.*}", casas, x),
        _ => String::from("(ausente)"),
    }
}

/// A medida como ficha, uma linha por campo.
pub fn ficha_da_medida(m: &Medida) -> String {
    let m = m.clone().quantizada();
    let vazio = String::from("(ausente)");
    let mut s = String::new();
    s.push_str(&format!("pesos = {}\n", m.pesos.clone().unwrap_or(vazio.clone())));
    s.push_str(&format!("respostas = {}\n", m.respostas));
    s.push_str(&format!("pedidos = {}\n", m.pedidos));
    s.push_str(&format!("sonda = {}\n", m.sonda));
    s.push_str(&format!("abstencoes = {}\n", m.abstencoes));
    s.push_str(&format!("fracao_de_abstencao = {:.*}\n", CASAS_FRACAO, m.fracao_de_abstencao));
    s.push_str(&format!("g1_identicas = {}\n", m.g1_identicas));
    s.push_str(&format!("g1_hash = {}\n", m.g1_hash));
    s.push_str(&format!("tokens_por_chamada = {:.*}\n", CASAS_CUSTO, m.tokens_por_chamada));
    s.push_str(&format!("ms_por_chamada = {:.*}\n", CASAS_CUSTO, m.ms_por_chamada));
    s.push_str(&format!("n_pares = {}\n", m.n_pares));
    s.push_str(&format!("brier = {}\n", escore(m.brier, CASAS_ESCORE)));
    s.push_str(&format!("ece = {}\n", escore(m.ece, CASAS_ESCORE)));
    s.push_str(&format!("amostra_minima = {}\n", m.amostra_minima));
    s
}

/// Le a ficha de volta para a medida. Campo ausente vira o padrao, campo numerico invalido e erro.
pub fn de_ficha(texto: &str) -> Result<Medida, String> {
    let mut m = Medida::default();
    for linha in texto.lines() {
        let (chave, valor) = match linha.split_once('=') {
            Some(par) => (par.0.trim(), par.1.trim()),
            None => continue,
        };
        let numero = |v: &str| -> Result<f64, String> {
            let x = v.parse::<f64>().map_err(|_| format!("{chave}: {v:?} nao e numero"))?;
            if !x.is_finite() {
                return Err(format!("{chave}: {v:?} nao e numero finito; ausencia se escreve (ausente)"));
            }
            Ok(x)
        };
        let opcional = |v: &str| -> Result<Option<f64>, String> {
            if v == "(ausente)" || v.is_empty() {
                return Ok(None);
            }
            numero(v).map(Some)
        };
        let inteiro = |v: &str| v.parse::<usize>().map_err(|_| format!("{chave}: {v:?} nao e inteiro"));
        match chave {
            "pesos" => m.pesos = if valor == "(ausente)" || valor.is_empty() { None } else { Some(valor.to_string()) },
            "respostas" => m.respostas = inteiro(valor)?,
            "pedidos" => m.pedidos = inteiro(valor)?,
            "sonda" => m.sonda = valor.to_string(),
            "abstencoes" => m.abstencoes = inteiro(valor)?,
            "fracao_de_abstencao" => m.fracao_de_abstencao = numero(valor)?,
            "g1_identicas" => m.g1_identicas = valor == "true",
            "g1_hash" => m.g1_hash = valor.to_string(),
            "tokens_por_chamada" => m.tokens_por_chamada = numero(valor)?,
            "ms_por_chamada" => m.ms_por_chamada = numero(valor)?,
            "n_pares" => m.n_pares = inteiro(valor)?,
            "brier" => m.brier = opcional(valor)?,
            "ece" => m.ece = opcional(valor)?,
            "amostra_minima" => m.amostra_minima = inteiro(valor)?,
            _ => {}
        }
    }
    Ok(m)
}

/// A linha de registro de uma rodada, ou o motivo de nao haver linha.
///
/// A regra e declarada antes: sem peso verificado, sem G1 fechado, sem amostra minima e sem Brier e ECE, nao ha
/// linha. Registrar meia medida seria pior do que nao registrar nada.
pub fn linha_de_registro(m: &Medida, data: &str) -> Result<String, String> {
    let data = data.trim();
    if data.len() != 10 || data.chars().filter(|c| *c == '-').count() != 2 {
        return Err(String::from("data tem de vir no formato AAAA-MM-DD"));
    }
    let pesos = m.pesos.clone().ok_or("sem peso verificado: o registro exige o sha256 do artefato")?;
    if !m.g1_identicas {
        return Err(String::from("G1 nao fechou: duas execucoes com respostas diferentes nao viram registro"));
    }
    if m.n_pares < m.amostra_minima {
        return Err(format!(
            "amostra insuficiente: {} pares contra o minimo de {}",
            m.n_pares, m.amostra_minima
        ));
    }
    let brier = m.brier.ok_or("sem Brier medido")?;
    let ece = m.ece.ok_or("sem ECE medido")?;
    if !brier.is_finite() || !ece.is_finite() {
        return Err(String::from("Brier ou ECE nao finito nao vira registro: escore tem de ser numero"));
    }
    Ok(format!(
        "{data}\t{pesos}\t{}\t{}\t{}\t{:.1}\t{:.1}\t{brier:.6}\t{ece:.6}",
        m.pedidos, m.abstencoes, m.n_pares, m.tokens_por_chamada, m.ms_por_chamada
    ))
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
    fn a_medida_vai_e_volta_pela_ficha() {
        let m = Medida {
            pesos: Some(String::from("abc")),
            respostas: 32,
            pedidos: 32,
            sonda: String::from("casos do proprio artefato"),
            abstencoes: 1,
            fracao_de_abstencao: 1.0 / 33.0,
            g1_identicas: true,
            g1_hash: String::from("deadbeef"),
            tokens_por_chamada: 95.25,
            ms_por_chamada: 1.5,
            n_pares: 32,
            brier: Some(0.25),
            ece: Some(0.125),
            amostra_minima: 30,
        };
        let ficha = ficha_da_medida(&m);
        let volta = de_ficha(&ficha).expect("le de volta");
        assert_eq!(volta.pesos, m.pesos);
        assert_eq!(volta.pedidos, 32);
        assert_eq!(volta.abstencoes, 1);
        assert!(volta.g1_identicas);
        assert!((volta.brier.unwrap() - 0.25).abs() < 1e-12);
        assert!((volta.ece.unwrap() - 0.125).abs() < 1e-12);
        assert_eq!(volta, m.quantizada(), "a ficha volta identica apos a quantizacao declarada");
    }

    #[test]
    fn a_quantizacao_declarada_fecha_o_um_terco() {
        let m = Medida {
            fracao_de_abstencao: 1.0 / 33.0,
            tokens_por_chamada: 95.25,
            ms_por_chamada: 1.04,
            ..Medida::default()
        };
        let volta = de_ficha(&ficha_da_medida(&m)).expect("le de volta");
        assert_eq!(volta.fracao_de_abstencao, 0.030303, "seis casas, sem epsilon");
        assert_eq!(volta.tokens_por_chamada, 95.3, "uma casa: 95.25 sobe");
        assert_eq!(volta.ms_por_chamada, 1.0, "uma casa: 1.04 desce");
        assert_eq!(volta, m.quantizada(), "quantizar na borda faz a volta ser igualdade");
        assert_ne!(volta.fracao_de_abstencao, 1.0 / 33.0, "a ficha e lossy por declaracao, nao por acidente");
    }

    #[test]
    fn ausencia_volta_ausente_e_nan_e_recusado() {
        let m = Medida { pesos: Some(String::from("abc")), ..Medida::default() };
        let ficha = ficha_da_medida(&m);
        assert!(ficha.contains("brier = (ausente)"), "{ficha}");
        let volta = de_ficha(&ficha).expect("le de volta");
        assert_eq!(volta.brier, None, "ausencia nao ressuscita como NaN");
        assert_eq!(volta.ece, None, "ausencia nao ressuscita como NaN");
        assert!(de_ficha("brier = NaN\n").is_err(), "NaN nao entra pela leitura");
        assert!(de_ficha("ece = inf\n").is_err(), "infinito nao entra pela leitura");
        assert!(de_ficha("fracao_de_abstencao = (ausente)\n").is_err(), "campo obrigatorio nao fica ausente");
    }

    #[test]
    fn escore_nao_finito_nao_vira_registro() {
        let m = Medida {
            pesos: Some(String::from("abc")),
            g1_identicas: true,
            n_pares: 32,
            amostra_minima: 30,
            brier: Some(f64::NAN),
            ece: Some(0.1),
            ..Medida::default()
        };
        let motivo = linha_de_registro(&m, "2026-10-17").expect_err("NaN nao vira linha");
        assert!(motivo.contains("finito"), "{motivo}");
    }

    #[test]
    fn o_registro_recusa_meia_medida() {
        let completo = Medida {
            pesos: Some(String::from("abc")),
            g1_identicas: true,
            n_pares: 32,
            amostra_minima: 30,
            brier: Some(0.2),
            ece: Some(0.1),
            ..Medida::default()
        };
        assert!(linha_de_registro(&completo, "2026-10-17").is_ok(), "medida completa registra");

        let sem_peso = Medida { pesos: None, ..completo.clone() };
        assert!(linha_de_registro(&sem_peso, "2026-10-17").unwrap_err().contains("sha256"));

        let sem_g1 = Medida { g1_identicas: false, ..completo.clone() };
        assert!(linha_de_registro(&sem_g1, "2026-10-17").unwrap_err().contains("G1"));

        let amostra_curta = Medida { n_pares: 3, ..completo.clone() };
        assert!(linha_de_registro(&amostra_curta, "2026-10-17").unwrap_err().contains("amostra"));

        let sem_escore = Medida { brier: None, ..completo.clone() };
        assert!(linha_de_registro(&sem_escore, "2026-10-17").unwrap_err().contains("Brier"));

        assert!(linha_de_registro(&completo, "17/10/2026").unwrap_err().contains("AAAA-MM-DD"));
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
