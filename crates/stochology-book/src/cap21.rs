//! Capítulo 21 — o verificador da ficha do projeto final.
//!
//! Laboratório autocontido, sem dependências externas: um verificador que recusa uma ficha sem
//! pergunta do decisor, sem semente e sem cláusula de saída, e que mostra cada regra reprovando
//! ao menos uma ficha (um portão precisa saber reprovar).
//! Crate de rascunho: ~/tmp/labs-cap21 (com [workspace] vazio no Cargo.toml).
//! Rodar: cargo run --release

/// As sete regras, na ordem em que são aplicadas.
const REGRAS: [&str; 7] = [
    "R1 processo gerador declarado",
    "R2 unidade independente declarada",
    "R3 modelo nulo com a mesma semente",
    "R4 pergunta do decisor (quem decide, qual ação, pergunta)",
    "R5 semente fixa numerica",
    "R6 hash do pre-registro com 64 digitos hexadecimais",
    "R7 clausula de saida com gatilho objetivo",
];

/// Nomes genéricos que a regra do decisor recusa.
const GENERICOS: [&str; 9] = [
    "o decisor",
    "os decisores",
    "o usuario",
    "o cliente",
    "o modelo",
    "os dados",
    "a decisao",
    "quem decide",
    "ninguem",
];

struct Ficha {
    nome: &'static str,
    gerador: &'static str,
    unidade: &'static str,
    nulo: &'static str,
    decisor: &'static str,
    acao: &'static str,
    pergunta: &'static str,
    semente: &'static str,
    hash: &'static str,
    saida: &'static str,
}

fn hex64(s: &str) -> bool {
    s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit())
}

/// O verificador. Devolve a lista de problemas; vazia quer dizer ficha aceita.
fn verificar(f: &Ficha) -> Vec<String> {
    let mut p = Vec::new();

    // R1
    if f.gerador.trim().len() < 10 {
        p.push("R1 campo gerador vazio ou curto demais".to_string());
    }
    // R2
    if f.unidade.trim().len() < 5 {
        p.push("R2 campo unidade vazio".to_string());
    }
    // R3
    if f.nulo.trim().len() < 5 || !f.nulo.contains(&f.semente) && f.semente.len() > 0 {
        p.push("R3 modelo nulo ausente ou sem a mesma semente".to_string());
    }
    // R4 — a pergunta do decisor, em três partes
    let t = f.pergunta;
    if t.trim().is_empty() {
        p.push("R4 campo pergunta_do_decisor ausente".to_string());
    } else {
        if t.chars().count() < 60 {
            p.push("R4 pergunta curta demais (menos de 60 caracteres)".to_string());
        }
        match t.find("Quem decide:") {
            None => p.push("R4 o texto nao nomeia quem decide".to_string()),
            Some(i) => {
                let resto = &t[i + "Quem decide:".len()..];
                let fim = resto
                    .find("Ação que a decisão muda:")
                    .unwrap_or(resto.len())
                    .min(120);
                let nome = resto[..fim].trim();
                if nome.chars().count() < 3 {
                    p.push("R4 o texto nao nomeia quem decide".to_string());
                } else if GENERICOS.iter().any(|g| nome.to_lowercase().starts_with(g)) {
                    p.push("R4 quem decide e generico".to_string());
                }
            }
        }
        let etiqueta = format!("Ação que a decisão muda: {}", f.acao);
        if !t.contains(&etiqueta) {
            p.push("R4 a acao da decisao nao aparece literalmente".to_string());
        }
        if !t.trim_end().ends_with('?') {
            p.push("R4 o texto nao termina em pergunta".to_string());
        }
    }
    // R5
    if f.semente.parse::<u64>().is_err() {
        p.push("R5 sem semente fixa numerica".to_string());
    }
    // R6
    if !hex64(f.hash) {
        p.push("R6 hash do pre-registro ausente ou fora do formato".to_string());
    }
    // R7
    if f.saida.trim().is_empty() {
        p.push("R7 sem clausula de saida".to_string());
    } else if !f.saida.chars().any(|c| c.is_ascii_digit()) {
        p.push("R7 clausula de saida sem gatilho objetivo".to_string());
    }
    p
}

pub fn rodar() {
    println!("== Capítulo 21 :: o verificador da ficha do projeto final ==");
    println!("sete regras, e cada uma tem de reprovar ao menos uma ficha:\n");
    for (i, r) in REGRAS.iter().enumerate() {
        println!("  {}. {}", i + 1, r);
    }

    let base = Ficha {
        nome: "01 completa",
        gerador: "sorteio por escola, semente fixa 2026, ruido gaussiano",
        unidade: "a escola, nao o aluno",
        nulo: "o mesmo mundo com a semente 2026 e sem o efeito",
        decisor: "a secretaria municipal de educacao",
        acao: "reduzir as turmas das series iniciais para 15 alunos",
        pergunta: "Quem decide: a secretaria municipal de educacao. Ação que a decisão muda: reduzir as turmas das series iniciais para 15 alunos. A pergunta do decisor e: reduzir a turma para 15 alunos melhora a nota de leitura um ano depois?",
        semente: "2026",
        hash: "0f1e2d3c4b5a69788796a5b4c3d2e1f00f1e2d3c4b5a69788796a5b4c3d2e1f0",
        saida: "vale enquanto o efeito medido ficar acima de 0,10 dp em tres anos seguidos",
    };
    let casos: [Ficha; 12] = [
        Ficha { ..base },
        Ficha { nome: "02 sem pergunta do decisor", pergunta: "", ..base },
        Ficha {
            nome: "03 quem decide generico",
            pergunta: "Quem decide: o decisor. Ação que a decisão muda: reduzir as turmas das series iniciais para 15 alunos. A pergunta do decisor e: reduzi-las melhora a leitura?",
            ..base
        },
        Ficha {
            nome: "04 a acao nao aparece literal",
            pergunta: "Quem decide: a secretaria municipal de educacao. Ação que a decisão muda: contratar mais professores. A pergunta do decisor e: isso melhora a leitura um ano depois?",
            ..base
        },
        Ficha {
            nome: "05 sem interrogar",
            pergunta: "Quem decide: a secretaria municipal de educacao. Ação que a decisão muda: reduzir as turmas das series iniciais para 15 alunos. A pergunta do decisor e: isto melhora a leitura um ano depois.",
            ..base
        },
        Ficha { nome: "06 sem semente", semente: "", ..base },
        Ficha { nome: "07 sem clausula de saida", saida: "", ..base },
        Ficha {
            nome: "08 clausula sem gatilho",
            saida: "vale enquanto o modelo continuar bom e ninguem reclamar",
            ..base
        },
        Ficha { nome: "09 sem pre-registro", hash: "", ..base },
        Ficha { nome: "10 sem modelo nulo", nulo: "", ..base },
        Ficha { nome: "11 sem gerador", gerador: "sei la", ..base },
        Ficha { nome: "12 sem unidade", unidade: "?", ..base },
    ];

    let mut aceitas = 0usize;
    let mut reprovacoes = vec![0usize; 7];
    for f in &casos {
        let problemas = verificar(f);
        if problemas.is_empty() {
            aceitas += 1;
            println!("\n{:<28} ACEITA (quem decide: {})", f.nome, f.decisor);
        } else {
            println!("\n{:<28} RECUSA", f.nome);
            for p in &problemas {
                println!("    {}", p);
                if let Some(n) = p.get(1..2).and_then(|d| d.parse::<usize>().ok()) {
                    reprovacoes[n - 1] += 1;
                }
            }
        }
    }
    println!(
        "\n{} fichas: {} aceita, {} recusadas",
        casos.len(),
        aceitas,
        casos.len() - aceitas
    );
    let cobertas = reprovacoes.iter().filter(|n| **n > 0).count();
    println!(
        "regras que reprovaram ao menos uma ficha: {} de {} | reprovacoes por regra: {:?}",
        cobertas,
        REGRAS.len(),
        reprovacoes
    );
}
