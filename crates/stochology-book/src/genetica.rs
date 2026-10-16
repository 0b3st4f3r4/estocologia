//! Capítulo 19 — a resolução fina da genética de levedura: o que a **segunda linha** muda.
//!
//! O registro diz, na letra, que a réplica achou *um gene num mutante*, e não um efeito medido com
//! resolução: cada mutante é um ambiente de uma linha só e o menor p-valor possível ali é 1/1 000.
//! Este laboratório mede o que a segunda linha muda no desenho da validação por intervenção, com
//! dois casos plantados pelo instrumento — o **mover verdadeiro** (o efeito aparece nas duas linhas
//! dele) e o **artefato de linha** (o efeito aparece numa linha só). A pergunta é qual dos dois a
//! validação consegue distinguir.
//!
//! Rodar: ~labs --genetica~

use stochology_h2::genetica::{Desenho, Veredito, comparar};

/// A semente do capítulo, a mesma do laboratório do ICP.
const SEMENTE: u64 = 2019;

fn marca(comparacao: &stochology_h2::genetica::Comparacao, m: usize) -> &'static str {
    if m == comparacao.mover_verdadeiro {
        " <- mover verdadeiro (efeito nas duas linhas)"
    } else if m == comparacao.artefato_de_linha {
        " <- artefato de linha (efeito numa linha so)"
    } else {
        ""
    }
}

fn uma_linha(c: &stochology_h2::genetica::Comparacao, desenho: &Desenho) -> Vec<String> {
    c.sem_segunda_linha
        .iter()
        .enumerate()
        .filter(|(_, v)| v.passa)
        .map(|(m, v)| {
            format!(
                "  mutante {:>2} | efeito {:>+7.3} | p {:>9.6} | {}{}",
                m,
                v.efeito_por_linha[0],
                v.p,
                if v.p <= (1.0 / (desenho.permutacoes as f64 + 1.0)) + 1e-12 { "passa no piso" } else { "passa" },
                marca(c, m)
            )
        })
        .collect()
}

fn duas_linhas(c: &stochology_h2::genetica::Comparacao, quem: &[usize], alfa: f64) -> Vec<String> {
    quem.iter()
        .map(|&m| {
            let v: &Veredito = &c.com_segunda_linha[m];
            format!(
                "  mutante {:>2} | efeito l1 {:>+7.3} | efeito l2 {:>+7.3} | faixa {:>6.3} | p {:>9.6} | {}{}",
                m,
                v.efeito_por_linha[0],
                v.efeito_por_linha[1],
                v.largura_do_intervalo,
                v.p,
                if v.passa { "passa" } else { "rejeitado" },
                if v.passa && v.p >= alfa { " (mas no piso)" } else { "" }
            ) + marca(c, v.mutante)
        })
        .collect()
}

pub fn rodar() {
    println!("== Capítulo 19 :: a resolução fina da validação por intervenção ==");
    let uma = Desenho::default();
    let duas = Desenho { linhas_por_mutante: 2, ..Desenho::default() };
    let c_uma = comparar(uma, SEMENTE);
    let c_duas = comparar(duas, SEMENTE);
    println!(
        "desenho: {} mutantes | {} individuos por linha | {} sorteios por teste | alfa = {:.2} | Bonferroni alfa/{} = {:.6}",
        uma.mutantes, uma.individuos_por_linha, uma.permutacoes, uma.alfa, uma.mutantes, c_uma.alfa_corrigido
    );
    println!(
        "plantados pelo instrumento: mover verdadeiro = mutante {} | artefato de linha = mutante {}",
        c_uma.mover_verdadeiro, c_uma.artefato_de_linha
    );

    println!(
        "\n-- a validacao de hoje: cada mutante e um ambiente de uma linha so --"
    );
    println!(
        "  {} mutantes x {} individuos | piso do desenho = {:.6} | limiar = {:.6}",
        uma.mutantes, uma.individuos_por_linha, c_uma.piso_sem, c_uma.alfa_corrigido
    );
    for linha in uma_linha(&c_uma, &uma) {
        println!("{linha}");
    }
    println!("  passam: {} de {}", c_uma.quantos_passam_sem(), uma.mutantes);

    println!("\n-- o mesmo mundo, com a segunda linha: o efeito tem de estar nas duas --");
    println!(
        "  {} mutantes x 2 linhas x {} individuos | piso do desenho = {:.6} | limiar = {:.6}",
        duas.mutantes, duas.individuos_por_linha, c_duas.piso_com, c_duas.alfa_corrigido
    );
    let mut olhar = vec![c_duas.mover_verdadeiro, c_duas.artefato_de_linha];
    for (m, v) in c_duas.com_segunda_linha.iter().enumerate() {
        if v.passa && !olhar.contains(&m) {
            olhar.push(m);
        }
    }
    olhar.sort_unstable();
    for linha in duas_linhas(&c_duas, &olhar, c_duas.alfa_corrigido) {
        println!("{linha}");
    }
    println!("  passam: {} de {}", c_duas.quantos_passam_com(), duas.mutantes);
    println!(
        "  efeito na segunda linha: artefato {:>+7.3} (nao se reproduz) | mover {:>+7.3} (se reproduz) | na validacao de uma linha a estimativa e um ponto e a faixa e zero",
        c_duas.com_segunda_linha[c_duas.artefato_de_linha].efeito_por_linha[1],
        c_duas.com_segunda_linha[c_duas.mover_verdadeiro].efeito_por_linha[1]
    );

    println!(
        "\nleitura: com uma linha o piso de {:.6} e o limiar de {:.6} deixam passar {} dos {} mutantes — o mover verdadeiro, o artefato de linha e dois mutantes que sao so ruido. Todos encostam no chao do desenho, e o p nao distingue nenhum dos tres do que se procura. Com a segunda linha o artefato sai da lista, porque o efeito dele na segunda linha e zero, e o piso cai para {:.9}: passa {} dos {} mutantes. O p nunca foi o problema: o problema era a linha ser uma so.",
        c_uma.piso_sem,
        c_uma.alfa_corrigido,
        c_uma.quantos_passam_sem(),
        uma.mutantes,
        c_duas.piso_com,
        c_duas.quantos_passam_com(),
        duas.mutantes
    );
}
