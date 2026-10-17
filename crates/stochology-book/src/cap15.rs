//! Laboratorio do Capitulo 15 — o cofre, a senha criptografica e a auditoria.
//!
//! Livro-razao encadeado com SHA-256 puro (sem dependencias) e raiz de Merkle, re-usados de `stochology-audit`.
//! Trocar um caractere no passado derruba a corrente inteira.

use stochology_audit::{LivroRazao, raiz_merkle, sha256_hex};

pub fn rodar() {
    println!("== Capitulo 15 :: o cofre, a senha e a auditoria ==");

    // Vetores conhecidos: se o SHA-256 estivesse errado, o resto nao valeria.
    println!(
        "sha256(\"\")={}",
        sha256_hex(b"")
    );
    println!(
        "sha256(\"abc\")={}",
        sha256_hex(b"abc")
    );

    let mut livro = LivroRazao::novo("sessao-h3");
    livro.registrar("h3.voos", "ECE=0.0048");
    livro.registrar("h3.bolsa", "mediana=2.1x");
    livro.registrar("h3.energia", "canario=ok");
    livro.registrar("h3.prisoes", "ECE=0.036");
    livro.registrar("h3.ri", "ECE=0.0048 habilidade=0.019");
    println!(
        "corrente: eventos={} integra={} raiz_merkle={}",
        livro.eventos.len(),
        livro.verificar(),
        &livro.raiz_merkle()[..16]
    );
    for e in &livro.eventos {
        println!(
            "  evento {} ficha={} conteudo={} hash={}",
            e.indice,
            e.ficha_id,
            e.conteudo,
            &e.hash_atual[..16]
        );
    }

    // Uma virgula no passado, com o hash do proprio evento recalculado:
    // o conteudo e o hash concordam, e mesmo assim a corrente cai.
    let mut adulterado = livro.clone();
    adulterado.eventos[2].conteudo = "canario=nok".to_string();
    adulterado.eventos[2].hash_atual = LivroRazao::hash_do_evento(
        "sessao-h3",
        adulterado.eventos[2].indice,
        &adulterado.eventos[2].ficha_id,
        &adulterado.eventos[2].conteudo,
        &adulterado.eventos[2].hash_anterior,
    );
    println!(
        "adulteracao no evento 2 (canario=ok -> canario=nok, hash do evento recalculado): integra={} raiz_mudou={}",
        adulterado.verificar(),
        adulterado.raiz_merkle() != livro.raiz_merkle()
    );

    // Recalcular a corrente a partir do ponto adulterado passa na conferencia
    // local: a prova nao esta na corrente, esta na raiz publicada antes.
    let mut reescrito = adulterado.clone();
    for i in 2..reescrito.eventos.len() {
        let anterior = reescrito.eventos[i - 1].hash_atual.clone();
        let e = reescrito.eventos[i].clone();
        reescrito.eventos[i].hash_anterior = anterior.clone();
        reescrito.eventos[i].hash_atual =
            LivroRazao::hash_do_evento("sessao-h3", e.indice, &e.ficha_id, &e.conteudo, &anterior);
    }
    println!(
        "adulteracao + reescrita de {} eventos seguintes: integra={} mas muda a raiz publicada ({})",
        reescrito.eventos.len() - 2,
        reescrito.verificar(),
        reescrito.raiz_merkle() != livro.raiz_merkle()
    );

    // Quanto da corrente cai quando um caractere muda.
    let mut queda = 0usize;
    for i in 0..livro.eventos.len() {
        let mut v = livro.clone();
        let mut c = v.eventos[i].conteudo.clone();
        c.push('!');
        v.eventos[i].conteudo = c;
        if !v.verificar() {
            queda += 1;
        }
    }
    println!(
        "um caractere a mais em cada evento: {} de {} alteracoes invalidam a corrente",
        queda,
        livro.eventos.len()
    );

    // Raiz de Merkle de um lote de fichas.
    let corpus = [
        "ficha=h0.posto-saude|decisao=abrir uma segunda sala",
        "ficha=h0.banco-2003|decisao=reforcar o atendimento",
        "ficha=h1.a-multiplicativo|decisao=reduzir a alavancagem",
        "ficha=h1.b-lalonde|decisao=manter o programa",
        "ficha=h1.c-incendios|decisao=triplicar a aposta",
        "ficha=h3.voos23|decisao=manter o modelo",
        "ficha=h3.prisoes|decisao=trocar o cofre",
    ];
    let hashes: Vec<String> = corpus.iter().map(|c| sha256_hex(c.as_bytes())).collect();
    let raiz_lote = raiz_merkle(&hashes);
    println!(
        "lote de fichas: n={} raiz_merkle={}",
        hashes.len(),
        &raiz_lote[..16]
    );
    let mut trocados = 0usize;
    for i in 0..hashes.len() {
        let mut h = hashes.clone();
        h[i] = sha256_hex(format!("{}|trocado", corpus[i]).as_bytes());
        if raiz_merkle(&h) != raiz_lote {
            trocados += 1;
        }
    }
    println!(
        "trocar uma ficha do lote muda a raiz em {} de {} casos",
        trocados,
        hashes.len()
    );
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn livro_razao_detecta_adulteracao_no_passado() {
        let mut l = LivroRazao::novo("cap15-teste");
        l.registrar("h0.a", "integro");
        l.registrar("h0.b", "segundo");
        assert!(l.verificar());
        l.eventos[0].conteudo.push('!');
        assert!(!l.verificar());
    }
}

