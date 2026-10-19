//! Chapter 18 laboratory — the vault, the cryptographic seal and the audit.
//!
//! Ledger chained with pure SHA-256 (no dependencies) and a Merkle root, reused from ~stochology-core~.
//! Changing one character in the past brings the whole chain down.

use stochology_core::audit::{Ledger, merkle_root, sha256_hex};

pub fn run() {
    println!("== Chapter 18 :: the vault, the seal and the audit ==");

    // Known vectors: if SHA-256 were wrong, the rest would not be worth anything.
    println!("sha256(\"\")={}", sha256_hex(b""));
    println!("sha256(\"abc\")={}", sha256_hex(b"abc"));

    let mut ledger = Ledger::new("sessao-h3");
    ledger.record("h3.voos", "ECE=0.0048");
    ledger.record("h3.bolsa", "mediana=2.1x");
    ledger.record("h3.energia", "canario=ok");
    ledger.record("h3.prisoes", "ECE=0.036");
    ledger.record("h3.ri", "ECE=0.0048 habilidade=0.019");
    println!(
        "chain: events={} intact={} merkle_root={}",
        ledger.events.len(),
        ledger.verify(),
        &ledger.merkle_root()[..16]
    );
    for e in &ledger.events {
        println!(
            "  event {} card={} content={} hash={}",
            e.index,
            e.card_id,
            e.content,
            &e.current_hash[..16]
        );
    }

    // A comma in the past, with the hash of the event itself recalculated:
    // the content and the hash agree, and even so the chain falls.
    let mut tampered = ledger.clone();
    tampered.events[2].content = "canario=nok".to_string();
    tampered.events[2].current_hash = Ledger::event_hash(
        "sessao-h3",
        tampered.events[2].index,
        &tampered.events[2].card_id,
        &tampered.events[2].content,
        &tampered.events[2].previous_hash,
    );
    println!(
        "tampering in event 2 (canario=ok -> canario=nok, event hash recalculated): intact={} root_changed={}",
        tampered.verify(),
        tampered.merkle_root() != ledger.merkle_root()
    );

    // Recomputing the chain from the tampered point passes the local check:
    // the proof is not in the chain, it is in the root published before.
    let mut rewritten = tampered.clone();
    for i in 2..rewritten.events.len() {
        let previous = rewritten.events[i - 1].current_hash.clone();
        let e = rewritten.events[i].clone();
        rewritten.events[i].previous_hash = previous.clone();
        rewritten.events[i].current_hash =
            Ledger::event_hash("sessao-h3", e.index, &e.card_id, &e.content, &previous);
    }
    println!(
        "tampering + rewrite of {} following events: intact={} but it changes the published root ({})",
        rewritten.events.len() - 2,
        rewritten.verify(),
        rewritten.merkle_root() != ledger.merkle_root()
    );

    // How much of the chain falls when one character changes.
    let mut drops = 0usize;
    for i in 0..ledger.events.len() {
        let mut v = ledger.clone();
        let mut c = v.events[i].content.clone();
        c.push('!');
        v.events[i].content = c;
        if !v.verify() {
            drops += 1;
        }
    }
    println!(
        "one extra character in each event: {} of {} alterations invalidate the chain",
        drops,
        ledger.events.len()
    );

    // Merkle root of a batch of cards.
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
    let batch_root = merkle_root(&hashes);
    println!(
        "batch of cards: n={} merkle_root={}",
        hashes.len(),
        &batch_root[..16]
    );
    let mut swapped = 0usize;
    for i in 0..hashes.len() {
        let mut h = hashes.clone();
        h[i] = sha256_hex(format!("{}|trocado", corpus[i]).as_bytes());
        if merkle_root(&h) != batch_root {
            swapped += 1;
        }
    }
    println!(
        "swapping one card of the batch changes the root in {} of {} cases",
        swapped,
        hashes.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ledger_detects_tampering_in_the_past() {
        let mut l = Ledger::new("ch18-test");
        l.record("h0.a", "integro");
        l.record("h0.b", "segundo");
        assert!(l.verify());
        l.events[0].content.push('!');
        assert!(!l.verify());
    }
}
