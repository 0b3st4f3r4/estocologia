//! Laboratorio do Capitulo 15 — o cofre, a senha criptografica e a auditoria.
//!
//! Livro-razao encadeado com SHA-256 puro (sem dependencias) e raiz de Merkle.
//! Trocar um caractere no passado derruba a corrente inteira.

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// SHA-256 puro, sem dependencias.
fn sha256(dados: &[u8]) -> [u8; 32] {
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    let bitlen = (dados.len() as u64).wrapping_mul(8);
    let mut msg = dados.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bitlen.to_be_bytes());

    for bloco in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([bloco[4 * i], bloco[4 * i + 1], bloco[4 * i + 2], bloco[4 * i + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
    let mut saida = [0u8; 32];
    for i in 0..8 {
        saida[4 * i..4 * i + 4].copy_from_slice(&h[i].to_be_bytes());
    }
    saida
}

fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

fn sha256_hex(dados: &[u8]) -> String {
    hex(&sha256(dados))
}

#[derive(Debug, Clone, PartialEq)]
struct Evento {
    indice: u64,
    ficha_id: String,
    conteudo: String,
    hash_anterior: String,
    hash_atual: String,
}

#[derive(Debug, Clone, PartialEq)]
struct LivroRazao {
    id_sessao: String,
    eventos: Vec<Evento>,
}

impl LivroRazao {
    fn novo(id_sessao: &str) -> Self {
        LivroRazao {
            id_sessao: id_sessao.to_string(),
            eventos: Vec::new(),
        }
    }

    fn hash_do_evento(
        id_sessao: &str,
        indice: u64,
        ficha_id: &str,
        conteudo: &str,
        anterior: &str,
    ) -> String {
        sha256_hex(format!("{}|{}|{}|{}|{}", id_sessao, indice, ficha_id, conteudo, anterior).as_bytes())
    }

    fn registrar(&mut self, ficha_id: &str, conteudo: &str) -> String {
        let indice = self.eventos.len() as u64;
        let anterior = match self.eventos.last() {
            Some(e) => e.hash_atual.clone(),
            None => "0".repeat(64),
        };
        let hash_atual = Self::hash_do_evento(&self.id_sessao, indice, ficha_id, conteudo, &anterior);
        self.eventos.push(Evento {
            indice,
            ficha_id: ficha_id.to_string(),
            conteudo: conteudo.to_string(),
            hash_anterior: anterior,
            hash_atual: hash_atual.clone(),
        });
        hash_atual
    }

    /// Recomputa a corrente inteira.
    fn verificar(&self) -> bool {
        let mut anterior = "0".repeat(64);
        for (i, e) in self.eventos.iter().enumerate() {
            if e.indice != i as u64 || e.hash_anterior != anterior {
                return false;
            }
            let esperado =
                Self::hash_do_evento(&self.id_sessao, e.indice, &e.ficha_id, &e.conteudo, &e.hash_anterior);
            if esperado != e.hash_atual {
                return false;
            }
            anterior = e.hash_atual.clone();
        }
        true
    }

    fn raiz_merkle(&self) -> String {
        raiz_merkle(&self.eventos.iter().map(|e| e.hash_atual.clone()).collect::<Vec<String>>())
    }
}

fn raiz_merkle(hashes: &[String]) -> String {
    if hashes.is_empty() {
        return sha256_hex(b"");
    }
    let mut nivel: Vec<String> = hashes.to_vec();
    while nivel.len() > 1 {
        let mut proximo: Vec<String> = Vec::new();
        let mut i = 0;
        while i < nivel.len() {
            let esquerda = nivel[i].clone();
            let direita = if i + 1 < nivel.len() {
                nivel[i + 1].clone()
            } else {
                nivel[i].clone()
            };
            proximo.push(sha256_hex(format!("{}{}", esquerda, direita).as_bytes()));
            i += 2;
        }
        nivel = proximo;
    }
    nivel[0].clone()
}

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
