//! Cofre fora do tempo: separacao declarada da serie em **treino** e **cofre**, com abertura unica.
//!
//! Declarado nos Capitulos 15 e 20 ("o cofre fora do tempo", "o cofre e a senha") e no inventario de
//! propostas ainda nao exigidas: em Rust existiam o livro-razao encadeado e a raiz de Merkle, e o cofre
//! era descrito e nao componente. Este modulo fecha a lacuna com a regra dura do projeto:
//!
//! > a particao futura fica sob custodia exclusiva do agente de ciclo de vida; nenhum agente de modelagem
//! > a le antes do pre-registro; ela e aberta **uma unica vez**.
//!
//! **A regra que este modulo prova:** errar depois da abertura exige **cofre novo**, nunca reajuste.
//!
//! * A abertura aceita entra no livro-razao com hash, e o conteudo do evento amarra o pre-registro
//!   (id, corte, hash da especificacao, n de itens, agente): trocar qualquer um desses campos fora do
//!   livro-razao faz \`verificar()\` reprovar.
//! * A segunda abertura devolve erro **e fica registrada** (\`FICHA_RECUSA\`), porque a tentacao de olhar
//!   duas vezes tambem e parte do registro: esconder a tentativa seria reescrever a trilha.
//! * \`corrigir()\` depois da abertura e recusado do mesmo jeito, com \`ErroCofre::CofreNovoNecessario\`.
//!   O unico conserto legitimo e \`sucessor()\`: id novo e pre-registro novo. Foi o que o H3 fez tres
//!   vezes (voos de 2023, prisoes de Toronto 2020-2021, Rhode Island 2013-2015).
//!
//! Determinismo: sem relogio, sem rede, sem semente aleatoria; o corte temporal e um carimbo inteiro
//! fornecido por quem chama.

use crate::{LivroRazao, sha256_hex};

/// Ficha do evento de abertura aceita.
pub const FICHA_ABERTURA: &str = "cofre.abertura";
/// Ficha do evento que registra a recusa (segunda abertura, correcao tardia).
pub const FICHA_RECUSA: &str = "cofre.recusa";

/// A regra do projeto, em uma linha, para quem le o codigo antes do teste.
pub const REGRA_DO_COFRE: &str = "cofre aberto uma unica vez; depois da abertura, o conserto e um cofre novo, nunca reajuste no mesmo dado";

/// Onde cai o corte entre treino e cofre, declarado antes de abrir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Corte {
    /// Corte por posicao: treino = [0, corte), cofre = [corte, n).
    Indice(usize),
    /// Corte temporal: treino = t < corte, cofre = t >= corte (t_disp, como no H3).
    Tempo(i64),
}

impl Corte {
    pub fn rotulo(&self) -> String {
        match self {
            Corte::Indice(i) => format!("indice:{}", i),
            Corte::Tempo(t) => format!("tempo:{}", t),
        }
    }
}

/// O que impede a separacao ou a abertura.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErroCofre {
    /// Serie sem nenhuma unidade.
    SerieVazia,
    /// Um dos lados sairia vazio: nao ha treino sem historia nem cofre sem futuro.
    CorteDegenerado { corte: Corte, n: usize },
    /// Serie temporal fora de ordem: o corte por tempo deixaria de ser um corte.
    SerieDesordenada {
        posicao: usize,
        anterior: i64,
        atual: i64,
    },
    /// Cofre sem conteudo sob custodia.
    CofreVazio,
    /// Abertura sem agente identificado: a custodia e de alguem.
    AgenteNaoDeclarado,
    /// Correcao pedida antes de abrir.
    NaoAberto,
    /// Segunda abertura do mesmo cofre: recusada e registrada.
    JaAberto { abertura: u64, hash: String },
    /// Correcao depois da abertura: exige cofre novo.
    CofreNovoNecessario {
        cofre: String,
        abertura: u64,
        hash: String,
    },
    /// Sucessor com o mesmo id do cofre ja aberto.
    CofreReutilizado { id: String },
    /// Sucessor com o mesmo pre-registro: seria reabrir o mesmo dado.
    PreRegistroRepetido { cofre: String },
}

impl std::fmt::Display for ErroCofre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErroCofre::SerieVazia => write!(f, "serie vazia: nao ha o que separar"),
            ErroCofre::CorteDegenerado { corte, n } => {
                write!(
                    f,
                    "corte degenerado {} sobre serie de {}",
                    corte.rotulo(),
                    n
                )
            }
            ErroCofre::SerieDesordenada {
                posicao,
                anterior,
                atual,
            } => write!(
                f,
                "serie desordenada na posicao {}: {} depois de {}",
                posicao, atual, anterior
            ),
            ErroCofre::CofreVazio => write!(f, "cofre vazio: nada sob custodia"),
            ErroCofre::AgenteNaoDeclarado => write!(f, "abertura sem agente declarado"),
            ErroCofre::NaoAberto => write!(f, "cofre ainda fechado: nao ha o que corrigir"),
            ErroCofre::JaAberto { abertura, hash } => write!(
                f,
                "segunda abertura recusada: a primeira foi o evento {} (hash {})",
                abertura, hash
            ),
            ErroCofre::CofreNovoNecessario {
                cofre,
                abertura,
                hash,
            } => write!(
                f,
                "correcao depois da abertura {} do cofre {} (hash {}): exige cofre novo",
                abertura, cofre, hash
            ),
            ErroCofre::CofreReutilizado { id } => {
                write!(f, "cofre {} ja foi aberto: sucessor precisa de id novo", id)
            }
            ErroCofre::PreRegistroRepetido { cofre } => write!(
                f,
                "cofre {} repete o pre-registro do antecessor: seria o mesmo dado",
                cofre
            ),
        }
    }
}

impl std::error::Error for ErroCofre {}

/// Serie cortada em dois lados que somam a serie inteira e nao compartilham unidade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Particao<T> {
    pub treino: Vec<T>,
    pub cofre: Vec<T>,
    pub corte: Corte,
    pub n: usize,
}

impl<T> Particao<T> {
    pub fn n_treino(&self) -> usize {
        self.treino.len()
    }

    pub fn n_cofre(&self) -> usize {
        self.cofre.len()
    }

    /// Os dois lados somam a serie inteira, sem unidade perdida.
    pub fn e_inteira(&self) -> bool {
        self.treino.len() + self.cofre.len() == self.n
    }
}

impl<T: PartialEq> Particao<T> {
    /// Alguma unidade aparece nos dois lados.
    pub fn se_sobrepoem(&self) -> bool {
        self.treino.iter().any(|t| self.cofre.contains(t))
    }

    /// A separacao declarada: inteira e sem sobreposicao.
    pub fn e_limpa(&self) -> bool {
        self.e_inteira() && !self.se_sobrepoem()
    }
}

/// Separa por posicao: treino = [0, corte), cofre = [corte, n).
pub fn separar_por_indice<T: Clone>(serie: &[T], corte: usize) -> Result<Particao<T>, ErroCofre> {
    if serie.is_empty() {
        return Err(ErroCofre::SerieVazia);
    }
    if corte == 0 || corte >= serie.len() {
        return Err(ErroCofre::CorteDegenerado {
            corte: Corte::Indice(corte),
            n: serie.len(),
        });
    }
    Ok(Particao {
        treino: serie[..corte].to_vec(),
        cofre: serie[corte..].to_vec(),
        corte: Corte::Indice(corte),
        n: serie.len(),
    })
}

/// Separa por tempo: treino = carimbo < corte, cofre = carimbo >= corte.
///
/// A serie tem de vir ordenada pelo carimbo; fora de ordem, o corte temporal nao e um corte e a
/// operacao e recusada.
pub fn separar_por_tempo<T: Clone, F: Fn(&T) -> i64>(
    serie: &[T],
    corte: i64,
    carimbo: F,
) -> Result<Particao<T>, ErroCofre> {
    if serie.is_empty() {
        return Err(ErroCofre::SerieVazia);
    }
    let mut anterior: Option<i64> = None;
    for (posicao, item) in serie.iter().enumerate() {
        let atual = carimbo(item);
        if let Some(a) = anterior
            && atual < a
        {
            return Err(ErroCofre::SerieDesordenada {
                posicao,
                anterior: a,
                atual,
            });
        }
        anterior = Some(atual);
    }
    let primeiro_do_cofre = serie
        .iter()
        .position(|x| carimbo(x) >= corte)
        .unwrap_or(serie.len());
    if primeiro_do_cofre == 0 || primeiro_do_cofre >= serie.len() {
        return Err(ErroCofre::CorteDegenerado {
            corte: Corte::Tempo(corte),
            n: serie.len(),
        });
    }
    Ok(Particao {
        treino: serie[..primeiro_do_cofre].to_vec(),
        cofre: serie[primeiro_do_cofre..].to_vec(),
        corte: Corte::Tempo(corte),
        n: serie.len(),
    })
}

/// Declara cofre e treino de uma vez: a particao e o lado do cofre ja sob custodia.
pub fn declarar_cofre<T: Clone>(
    id: &str,
    especificacao: &str,
    serie: &[T],
    corte: usize,
) -> Result<(Particao<T>, Cofre<T>), ErroCofre> {
    let particao = separar_por_indice(serie, corte)?;
    let cofre = Cofre::novo(
        id,
        Corte::Indice(corte),
        especificacao,
        particao.cofre.clone(),
    )?;
    Ok((particao, cofre))
}

/// A leitura devolvida pela abertura unica: o conteudo sai do cofre uma vez, e fica o hash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abertura<T> {
    pub cofre: String,
    /// Posicao do evento de abertura no livro-razao.
    pub ordem: u64,
    pub hash: String,
    pub raiz_merkle: String,
    pub agente: String,
    pub corte: Corte,
    pub hash_pre_registro: String,
    pub itens: Vec<T>,
}

impl<T> Abertura<T> {
    pub fn n_itens(&self) -> usize {
        self.itens.len()
    }
}

/// Particao futura sob custodia, aberta uma unica vez e auditada no livro-razao.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cofre<T> {
    pub id: String,
    pub corte: Corte,
    /// Hash SHA-256 da especificacao congelada antes de qualquer leitura.
    pub hash_pre_registro: String,
    conteudo: Vec<T>,
    agente: Option<String>,
    aberto: bool,
    livro: LivroRazao,
}

impl<T> Cofre<T> {
    pub fn n_itens(&self) -> usize {
        self.conteudo.len()
    }

    pub fn aberto(&self) -> bool {
        self.aberto
    }

    /// Aberturas aceitas: tem de ser 0 ou 1, nunca 2.
    pub fn aberturas(&self) -> u64 {
        self.livro
            .eventos
            .iter()
            .filter(|e| e.ficha_id == FICHA_ABERTURA)
            .count() as u64
    }

    /// Recusas registradas: segunda abertura e correcao tardia.
    pub fn recusas(&self) -> u64 {
        self.livro
            .eventos
            .iter()
            .filter(|e| e.ficha_id == FICHA_RECUSA)
            .count() as u64
    }

    pub fn livro(&self) -> &LivroRazao {
        &self.livro
    }

    pub fn raiz_merkle(&self) -> String {
        self.livro.raiz_merkle()
    }

    fn primeira_abertura(&self) -> Option<(u64, String)> {
        self.livro
            .eventos
            .iter()
            .find(|e| e.ficha_id == FICHA_ABERTURA)
            .map(|e| (e.indice, e.hash_atual.clone()))
    }

    /// Conteudo canonico do evento de abertura: amarra o pre-registro ao livro-razao.
    fn conteudo_da_abertura(&self, agente: &str) -> String {
        format!(
            "aberta|{}|{}|{}|{}|{}",
            self.id,
            self.corte.rotulo(),
            self.hash_pre_registro,
            self.conteudo.len(),
            agente
        )
    }

    /// Recomputa a corrente e cobra o proprio cofre: no maximo uma abertura aceita, e o estado
    /// declarado tem de bater com o evento gravado.
    pub fn verificar(&self) -> bool {
        if !self.livro.verificar() {
            return false;
        }
        let mut aceitas = self
            .livro
            .eventos
            .iter()
            .filter(|e| e.ficha_id == FICHA_ABERTURA);
        let primeira = aceitas.next();
        if aceitas.next().is_some() {
            return false;
        }
        match (primeira, self.aberto) {
            (None, false) => true,
            (Some(evento), true) => match &self.agente {
                Some(a) => evento.conteudo == self.conteudo_da_abertura(a),
                None => false,
            },
            _ => false,
        }
    }
}

impl<T: Clone> Cofre<T> {
    /// Declara o cofre: guarda o conteudo e congela o pre-registro pelo hash. Nada foi lido ainda.
    pub fn novo(
        id: &str,
        corte: Corte,
        especificacao: &str,
        conteudo: Vec<T>,
    ) -> Result<Self, ErroCofre> {
        if conteudo.is_empty() {
            return Err(ErroCofre::CofreVazio);
        }
        Ok(Cofre {
            id: id.to_string(),
            corte,
            hash_pre_registro: sha256_hex(especificacao.as_bytes()),
            conteudo,
            agente: None,
            aberto: false,
            livro: LivroRazao::novo(id),
        })
    }

    /// Abre o cofre. Na segunda vez devolve erro, registra a recusa e nao entrega nada de novo.
    pub fn abrir(&mut self, agente: &str) -> Result<Abertura<T>, ErroCofre> {
        if agente.is_empty() {
            return Err(ErroCofre::AgenteNaoDeclarado);
        }
        if self.aberto {
            let (ordem, hash) = self
                .primeira_abertura()
                .unwrap_or((u64::MAX, "0".repeat(64)));
            self.livro.registrar(
                FICHA_RECUSA,
                &format!(
                    "segunda_abertura|{}|agente={}|primeira={}",
                    self.id, agente, hash
                ),
            );
            return Err(ErroCofre::JaAberto {
                abertura: ordem,
                hash,
            });
        }
        let conteudo = self.conteudo_da_abertura(agente);
        let ordem = self.livro.eventos.len() as u64;
        let hash = self.livro.registrar(FICHA_ABERTURA, &conteudo);
        self.aberto = true;
        self.agente = Some(agente.to_string());
        Ok(Abertura {
            cofre: self.id.clone(),
            ordem,
            hash,
            raiz_merkle: self.raiz_merkle(),
            agente: agente.to_string(),
            corte: self.corte.clone(),
            hash_pre_registro: self.hash_pre_registro.clone(),
            itens: self.conteudo.clone(),
        })
    }

    /// A regra dura, em codigo: depois de aberto, reajuste no mesmo dado e recusado e a tentativa fica
    /// no registro. O conserto legitimo e um cofre novo.
    pub fn corrigir(&mut self, agente: &str, ajuste: &str) -> Result<(), ErroCofre> {
        if !self.aberto {
            return Err(ErroCofre::NaoAberto);
        }
        let (ordem, hash) = self
            .primeira_abertura()
            .unwrap_or((u64::MAX, "0".repeat(64)));
        self.livro.registrar(
            FICHA_RECUSA,
            &format!(
                "correcao_tardia|{}|agente={}|ajuste={}|primeira={}",
                self.id, agente, ajuste, hash
            ),
        );
        Err(ErroCofre::CofreNovoNecessario {
            cofre: self.id.clone(),
            abertura: ordem,
            hash,
        })
    }

    /// O sucessor: id novo e pre-registro novo, nascido fechado. Reprovar aqui e o portao.
    pub fn sucessor(
        &self,
        id: &str,
        corte: Corte,
        especificacao: &str,
        conteudo: Vec<T>,
    ) -> Result<Cofre<T>, ErroCofre> {
        if id == self.id {
            return Err(ErroCofre::CofreReutilizado { id: id.to_string() });
        }
        if sha256_hex(especificacao.as_bytes()) == self.hash_pre_registro {
            return Err(ErroCofre::PreRegistroRepetido {
                cofre: id.to_string(),
            });
        }
        Cofre::novo(id, corte, especificacao, conteudo)
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    fn serie(n: usize) -> Vec<String> {
        (0..n).map(|i| format!("d{:03}", i)).collect()
    }

    /// Especificacao congelada do cofre de teste, declarada antes de qualquer leitura.
    const ESP13: &str = "espec: voos out-dez 2013, G2 declarado antes de abrir";

    fn cofre_de_teste() -> Cofre<String> {
        let (_, cofre) = declarar_cofre("h3.voos13", ESP13, &serie(10), 6)
            .expect("cofre novo com quatro unidades no lado do cofre");
        cofre
    }

    #[test]
    fn separacao_por_indice_e_por_tempo_nao_se_sobrepoem() {
        let s = serie(10);
        let p = separar_por_indice(&s, 6).expect("corte valido");
        assert_eq!(p.n_treino(), 6);
        assert_eq!(p.n_cofre(), 4);
        assert_eq!(p.n, 10);
        assert!(p.e_limpa());
        assert!(!p.se_sobrepoem());
        assert_eq!(p.corte, Corte::Indice(6));
        assert_eq!(p.treino[0], "d000");
        assert_eq!(p.cofre[3], "d009");

        // corte temporal: o cofre comeca em t >= corte, como o t_disp do H3
        let itens: Vec<(i64, String)> = s
            .iter()
            .enumerate()
            .map(|(i, x)| (i as i64 * 10, x.clone()))
            .collect();
        let q = separar_por_tempo(&itens, 30, |x| x.0).expect("corte valido");
        assert_eq!(q.n_treino(), 3);
        assert_eq!(q.n_cofre(), 7);
        assert!(q.e_limpa());
        assert!(q.treino.iter().all(|x| x.0 < 30));
        assert!(q.cofre.iter().all(|x| x.0 >= 30));

        // o cofre declarado guarda exatamente o lado do cofre, e nasce fechado
        let (particao, cofre) = declarar_cofre("h3.voos23", "espec: voos out-dez 2023", &s, 6)
            .expect("declaracao valida");
        assert_eq!(cofre.n_itens(), particao.n_cofre());
        assert!(!cofre.aberto());
        assert_eq!(cofre.aberturas(), 0);
        assert_eq!(
            cofre.hash_pre_registro,
            sha256_hex("espec: voos out-dez 2023".as_bytes())
        );
    }

    #[test]
    fn corte_degenerado_serie_vazia_e_serie_desordenada_sao_recusados() {
        let s = serie(5);
        assert_eq!(
            separar_por_indice(&s, 0),
            Err(ErroCofre::CorteDegenerado {
                corte: Corte::Indice(0),
                n: 5
            })
        );
        assert_eq!(
            separar_por_indice(&s, 5),
            Err(ErroCofre::CorteDegenerado {
                corte: Corte::Indice(5),
                n: 5
            })
        );
        let vazia: &[String] = &[];
        assert_eq!(separar_por_indice(vazia, 1), Err(ErroCofre::SerieVazia));

        let desordenada = vec![(0i64, "a"), (10i64, "b"), (5i64, "c")];
        assert_eq!(
            separar_por_tempo(&desordenada, 10, |x| x.0),
            Err(ErroCofre::SerieDesordenada {
                posicao: 2,
                anterior: 10,
                atual: 5
            })
        );
        let sem_cofre = vec![(10i64, "a"), (20i64, "b")];
        assert_eq!(
            separar_por_tempo(&sem_cofre, 30, |x| x.0),
            Err(ErroCofre::CorteDegenerado {
                corte: Corte::Tempo(30),
                n: 2
            })
        );

        // cofre sem conteudo e cofre reutilizado tambem reprovam
        assert_eq!(
            Cofre::novo("vazio", Corte::Indice(1), "esp", Vec::<String>::new()),
            Err(ErroCofre::CofreVazio)
        );
    }

    #[test]
    fn abertura_unica_e_a_segunda_falha_registrada() {
        let mut c = cofre_de_teste();
        assert!(!c.aberto());
        let a = c.abrir("F3").expect("primeira abertura");
        assert_eq!(a.n_itens(), 4);
        assert_eq!(a.itens, vec!["d006", "d007", "d008", "d009"]);
        assert_eq!(a.ordem, 0);
        assert_eq!(a.cofre, "h3.voos13");
        assert_eq!(a.hash, c.livro().eventos[0].hash_atual);
        assert_eq!(a.raiz_merkle, c.raiz_merkle());
        assert_eq!(c.aberturas(), 1);
        assert_eq!(c.recusas(), 0);
        assert!(c.aberto());
        assert!(c.verificar());

        let raiz_antes = c.raiz_merkle();
        match c.abrir("F3") {
            Err(ErroCofre::JaAberto { abertura, hash }) => {
                assert_eq!(abertura, 0);
                assert_eq!(hash, a.hash);
            }
            outro => panic!("a segunda abertura devia falhar, veio {:?}", outro),
        }
        assert_eq!(c.aberturas(), 1, "nunca duas aberturas aceitas");
        assert_eq!(c.recusas(), 1);
        assert_ne!(c.raiz_merkle(), raiz_antes, "a recusa entra no registro");
        assert!(c.verificar());
        let ultimo = c.livro().eventos.last().expect("recusa registrada");
        assert_eq!(ultimo.ficha_id, FICHA_RECUSA);
        assert!(ultimo.conteudo.contains("segunda_abertura"));
        assert_eq!(ultimo.indice, 1);

        // sem agente declarado a abertura nem comeca: nada e registrado
        let mut novo = cofre_de_teste();
        assert_eq!(novo.abrir(""), Err(ErroCofre::AgenteNaoDeclarado));
        assert!(!novo.aberto());
        assert_eq!(novo.livro().eventos.len(), 0);
        assert!(novo.verificar());
    }

    #[test]
    fn adulterar_o_registro_invalida_a_verificacao() {
        let mut c = cofre_de_teste();
        c.abrir("F3").expect("primeira abertura");
        assert!(c.verificar());

        // (i) conteudo do evento de abertura reescrito: a corrente cai
        let mut mexido = c.clone();
        mexido.livro.eventos[0].conteudo = "aberta|h3.voos13|contra_a_especificacao".to_string();
        assert!(!mexido.verificar());

        // (ii) o registro da recusa tambem e auditado: apagar a tentativa derruba tudo
        assert!(c.abrir("F3").is_err());
        assert!(c.verificar());
        let mut mexido2 = c.clone();
        let ultimo = mexido2.livro.eventos.len() - 1;
        mexido2.livro.eventos[ultimo].conteudo = "segunda_abertura|nunca_aconteceu".to_string();
        assert!(!mexido2.verificar());

        // (iii) trocar o pre-registro fora do livro-razao tambem e pego
        let mut mexido3 = c.clone();
        mexido3.hash_pre_registro = sha256_hex(b"pre-registro reescrito depois do resultado");
        assert!(!mexido3.verificar());

        // (iv) apagar a abertura deixa o estado do cofre mentindo
        let mut mexido4 = c.clone();
        mexido4.livro.eventos[0].ficha_id = "cofre.outro".to_string();
        assert!(!mexido4.verificar());

        // (v) e o cofre intacto continua passando
        assert!(c.verificar());
    }

    #[test]
    fn raiz_de_merkle_muda_quando_o_conteudo_muda() {
        let esp = "espec: voos out-dez 2013";
        let mut a = Cofre::novo("cofre-a", Corte::Indice(6), esp, serie(10)).expect("a");
        let mut b = Cofre::novo("cofre-a", Corte::Indice(6), esp, serie(10)).expect("b");
        a.abrir("F3").expect("abertura de a");
        b.abrir("F3").expect("abertura de b");
        // mesma declaracao, mesmo conteudo: raiz reproduzivel
        assert_eq!(a.raiz_merkle(), b.raiz_merkle());
        assert_eq!(a.raiz_merkle(), a.livro().raiz_merkle());
        let hashes: Vec<String> = a
            .livro()
            .eventos
            .iter()
            .map(|e| e.hash_atual.clone())
            .collect();
        assert_eq!(crate::raiz_merkle(&hashes), a.raiz_merkle());

        // conteudo a mais no registro (a recusa): raiz muda
        assert!(b.abrir("F3").is_err());
        assert_ne!(a.raiz_merkle(), b.raiz_merkle());

        // outro agente na abertura: conteudo diferente, raiz diferente
        let mut d = Cofre::novo("cofre-a", Corte::Indice(6), esp, serie(10)).expect("d");
        d.abrir("F1").expect("abertura de d");
        assert_ne!(a.raiz_merkle(), d.raiz_merkle());

        // conteudo sob custodia diferente: raiz diferente
        let mut e = Cofre::novo("cofre-a", Corte::Indice(6), esp, serie(9)).expect("e");
        e.abrir("F3").expect("abertura de e");
        assert_ne!(a.raiz_merkle(), e.raiz_merkle());
    }

    #[test]
    fn errar_depois_da_abertura_exige_cofre_novo_e_nao_reajuste() {
        // a regra do projeto, declarada e cobrada aqui
        assert!(REGRA_DO_COFRE.contains("cofre novo"));

        let mut c = cofre_de_teste();
        // fechado, nao ha o que corrigir
        assert_eq!(c.corrigir("F3", "delta=-0.027"), Err(ErroCofre::NaoAberto));

        c.abrir("F3").expect("primeira abertura");
        let raiz = c.raiz_merkle();
        // reajustar no mesmo dado: recusado, e a tentacao fica no registro
        match c.corrigir("F3", "delta=-0.027") {
            Err(ErroCofre::CofreNovoNecessario {
                cofre,
                abertura,
                hash,
            }) => {
                assert_eq!(cofre, "h3.voos13");
                assert_eq!(abertura, 0);
                assert_eq!(hash, c.livro().eventos[0].hash_atual);
            }
            outro => panic!("reajuste pos-abertura devia ser recusado, veio {:?}", outro),
        }
        assert_eq!(c.recusas(), 1);
        assert_eq!(c.aberturas(), 1);
        assert!(c.verificar());
        assert_ne!(c.raiz_merkle(), raiz);
        assert!(c.livro().eventos[1].conteudo.contains("correcao_tardia"));

        // a unica saida legitima: cofre novo, com id novo e pre-registro novo
        assert_eq!(
            c.sucessor("h3.voos13", Corte::Indice(6), "espec nova", serie(10)),
            Err(ErroCofre::CofreReutilizado {
                id: "h3.voos13".to_string()
            })
        );
        assert_eq!(
            c.sucessor("h3.voos23", Corte::Indice(6), ESP13, serie(10)),
            Err(ErroCofre::PreRegistroRepetido {
                cofre: "h3.voos23".to_string()
            })
        );
        let mut sucessor = c
            .sucessor(
                "h3.voos23",
                Corte::Indice(6),
                "espec: voos out-dez 2023, mesmo trimestre de 2013",
                serie(10),
            )
            .expect("cofre novo");
        assert!(!sucessor.aberto());
        assert_ne!(sucessor.hash_pre_registro, c.hash_pre_registro);
        assert_eq!(sucessor.aberturas(), 0);
        sucessor.abrir("F3").expect("abertura do sucessor");
        assert!(sucessor.verificar());

        // e o registro do cofre antigo nao foi reescrito
        assert_eq!(c.aberturas(), 1);
        assert_eq!(c.recusas(), 1);
        assert!(c.verificar());
    }
}
