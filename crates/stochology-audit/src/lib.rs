//! Trilha criptográfica: SHA-256, livro-razão encadeado e raiz de Merkle.

pub mod cofre;

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

/// SHA-256 puro, sem dependências.
pub fn sha256(dados: &[u8]) -> [u8; 32] {
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
            w[i] = u32::from_be_bytes([
                bloco[4 * i],
                bloco[4 * i + 1],
                bloco[4 * i + 2],
                bloco[4 * i + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];

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

pub fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

pub fn sha256_hex(dados: &[u8]) -> String {
    hex(&sha256(dados))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evento {
    pub indice: u64,
    pub ficha_id: String,
    pub conteudo: String,
    pub hash_anterior: String,
    pub hash_atual: String,
}

/// Livro-razão encadeado: cada evento carrega o hash do anterior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LivroRazao {
    pub id_sessao: String,
    pub eventos: Vec<Evento>,
}

impl LivroRazao {
    pub fn novo(id_sessao: &str) -> Self {
        LivroRazao {
            id_sessao: id_sessao.to_string(),
            eventos: Vec::new(),
        }
    }

    pub fn hash_do_evento(
        id_sessao: &str,
        indice: u64,
        ficha_id: &str,
        conteudo: &str,
        anterior: &str,
    ) -> String {
        sha256_hex(
            format!("{}|{}|{}|{}|{}", id_sessao, indice, ficha_id, conteudo, anterior).as_bytes(),
        )
    }

    pub fn registrar(&mut self, ficha_id: &str, conteudo: &str) -> String {
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

    /// Recomputa a corrente inteira: uma vírgula mudada no passado derruba tudo.
    pub fn verificar(&self) -> bool {
        let mut anterior = "0".repeat(64);
        for (i, e) in self.eventos.iter().enumerate() {
            if e.indice != i as u64 || e.hash_anterior != anterior {
                return false;
            }
            let esperado = Self::hash_do_evento(
                &self.id_sessao,
                e.indice,
                &e.ficha_id,
                &e.conteudo,
                &e.hash_anterior,
            );
            if esperado != e.hash_atual {
                return false;
            }
            anterior = e.hash_atual.clone();
        }
        true
    }

    pub fn raiz_merkle(&self) -> String {
        raiz_merkle(&self.eventos.iter().map(|e| e.hash_atual.clone()).collect::<Vec<_>>())
    }
}

/// Raiz de Merkle sobre uma lista de hashes hex.
pub fn raiz_merkle(hashes: &[String]) -> String {
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

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn sha256_bate_com_os_vetores_conhecidos() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            sha256_hex(b"o rato roeu a roupa do rei de roma"),
            sha256_hex(b"o rato roeu a roupa do rei de roma")
        );
    }

    #[test]
    fn corrente_detecta_adulteracao() {
        let mut l = LivroRazao::novo("sessao-h3");
        l.registrar("h3.voos", "ECE=0.0048");
        l.registrar("h3.bolsa", "mediana=2.1x");
        l.registrar("h3.energia", "canario=ok");
        assert!(l.verificar());

        let mut adulterado = l.clone();
        adulterado.eventos[1].conteudo = "ECE=0.0049".to_string();
        assert!(!adulterado.verificar());
    }

    #[test]
    fn merkle_e_reproduzivel() {
        let hashes: Vec<String> = (0..5).map(|i| sha256_hex(format!("no{}", i).as_bytes())).collect();
        assert_eq!(raiz_merkle(&hashes), raiz_merkle(&hashes));
        let mut outros = hashes.clone();
        outros[4] = sha256_hex(b"trocado");
        assert_ne!(raiz_merkle(&hashes), raiz_merkle(&outros));
    }
}
