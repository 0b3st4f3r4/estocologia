//! Bancada sem orquestrador: propor aberto, vetar caro, fechar mecanico.
//!
//! O orquestrador `M` decide em tres atos: escolhe a peca, distribui as lentes e assina o fechamento.
//! Aqui os tres atos se separam e o meio desaparece. Propor e livre, desde que pelo esquema da ficha.
//! Vetar custa: queima cota finita da janela, exige artefato falseador que exista na peca e registra
//! aposta `p_falha` fora dos extremos dogmaticos. Fechar nao tem assinatura: e uma funcao pura sobre o
//! estado da bancada, e o lacre do fechamento e o `sha256` da transcricao canonica.
//!
//! # As quatro regras que o compilador cobra
//!
//! - **G8.1** a atribuicao lente -> avaliador sai identica da mesma semente e da mesma peca;
//! - **G8.2** veto sem artefato da peca, ou leitura sem artefato, e erro de protocolo, nao decisao;
//! - **G8.3** veto acima da cota da janela, ou veto repetido na mesma lente, e erro de protocolo;
//! - **G8.4** silencio nao decide: antes do prazo o fecho e `Pendente`, depois do prazo sem quorum de
//!   leitura o fecho e `Arquivada`, e nada avanca por inercia.
//!
//! Erro de protocolo nao vira `Fecho`: sai como `Err(Recusa)`, e quem consome a bancada trata como
//! `Exit 1`. Foi o que o desenho inicial errava ao devolver `Fecho::Arquivada` para cota estourada:
//! arquivar e o destino de uma peca sem deliberacao completa, nao o castigo de um avaliador trapaca.
//!
//! # Pureza e auditoria
//!
//! `fechar` nao muta orcamento. Devolve um `Recibo` com o consumo que a decisao implica, e o chamador
//! aplica com `aplicar`, que e a unica funcao com `&mut`. Assim a mesma entrada reproduz o mesmo lacre
//! quantas vezes for rodada, e a cota nunca depende da ordem em que alguem decidiu chamar a funcao.
//!
//! Declarado e nao implementado: o rebaixamento de cota por reversao empirica (a metrica de tensao
//! entre reversao e abertura de artefato) e a janela rotativa entre bancadas. A bancada mede o fecho;
//! o preco da reversao e registro de pesquisa, e nao campo deste modulo.

use stochology_audit::sha256_hex;
use stochology_core::{AgenteA, Ficha, Rng};

/// Cota de veto por avaliador e por janela.
pub const VETOS_POR_JANELA: u8 = 2;

/// Leituras minimas distintas para uma peca sustentada existir. Abaixo disto, nao ha decisao: ha arquivo.
pub const LEITURA_MINIMA: usize = 6;

/// Identificador da lente: 1 a 8, na mesma ordem de `AgenteA`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lente(pub u8);

impl Lente {
    pub const TODAS: [Lente; 8] = [
        Lente(1),
        Lente(2),
        Lente(3),
        Lente(4),
        Lente(5),
        Lente(6),
        Lente(7),
        Lente(8),
    ];

    /// A lente de um agente da banca A1-A8. A lente nao muda de sentido por sorteio.
    pub fn de_agente(agente: AgenteA) -> Lente {
        match agente {
            AgenteA::A1Codigo => Lente(1),
            AgenteA::A2Dados => Lente(2),
            AgenteA::A3CicloVida => Lente(3),
            AgenteA::A4Inferencia => Lente(4),
            AgenteA::A5Governanca => Lente(5),
            AgenteA::A6Justica => Lente(6),
            AgenteA::A7Dominio => Lente(7),
            AgenteA::A8Decisao => Lente(8),
        }
    }

    pub fn agente(self) -> Option<AgenteA> {
        AgenteA::TODOS.iter().copied().find(|a| Lente::de_agente(*a) == self)
    }
}

/// Quem avalia. Numero estavel, sem cargo.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Avaliador(pub u32);

/// Unidade que absorve o dano do erro (Phi4). Nao e a pessoa que assina: e a sala, o grupo, a reparticao.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Unidade(String);

impl Unidade {
    pub fn nova(rotulo: &str) -> Result<Unidade, Recusa> {
        let limpo = rotulo.trim();
        if limpo.is_empty() {
            return Err(Recusa::UnidadeAusente);
        }
        Ok(Unidade(limpo.to_string()))
    }

    pub fn rotulo(&self) -> &str {
        &self.0
    }
}

/// A peca em deliberacao: conteudo fechado, artefatos declarados e a unidade que paga.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Peca {
    pub id: String,
    pub hash_conteudo: String,
    pub unidade: Unidade,
    /// Hashes que um veto ou uma leitura pode citar como prova material.
    pub artefatos: Vec<String>,
}

impl Peca {
    pub fn nova(id: &str, conteudo: &[u8], unidade: Unidade) -> Peca {
        Peca {
            id: id.to_string(),
            hash_conteudo: sha256_hex(conteudo),
            unidade,
            artefatos: Vec::new(),
        }
    }

    /// Declara um artefato citavel e devolve o proprio hash.
    pub fn com_artefato(mut self, conteudo: &[u8]) -> (Peca, String) {
        let hash = sha256_hex(conteudo);
        if !self.artefatos.contains(&hash) {
            self.artefatos.push(hash.clone());
        }
        (self, hash)
    }

    /// Converte a ficha do contrato em peca. Sem `quem_paga_pelo_erro`, a peca nao entra na bancada.
    pub fn de_ficha(ficha: &Ficha) -> Result<Peca, Recusa> {
        let unidade = Unidade::nova(ficha.quem_paga_pelo_erro.as_deref().unwrap_or(""))?;
        let canonica = format!(
            "{}|{}|{}|{}|{}",
            ficha.id,
            ficha.camada.codigo(),
            ficha.incerteza.codigo(),
            ficha.definicao,
            ficha.pergunta_do_decisor
        );
        let mut peca = Peca::nova(&ficha.id, canonica.as_bytes(), unidade);
        if let Some(artefato) = &ficha.artefato_simulavel {
            peca = peca.com_artefato(artefato.as_bytes()).0;
        }
        Ok(peca)
    }

    pub fn conhece(&self, artefato: &str) -> bool {
        artefato == self.hash_conteudo || self.artefatos.iter().any(|a| a == artefato)
    }
}

/// Atribuicao publicada antes da leitura: a lente e o avaliador que a ocupa.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Designacao {
    pub lente: Lente,
    pub avaliador: Avaliador,
}

/// Leitura registrada: o avaliador do posto leu o artefato, e o hash prova qual.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Leitura {
    pub avaliador: Avaliador,
    pub lente: Lente,
    pub artefato: String,
}

impl Leitura {
    pub fn nova(avaliador: Avaliador, lente: Lente, artefato: &str) -> Leitura {
        Leitura { avaliador, lente, artefato: artefato.to_string() }
    }
}

/// Veto: caro, material e apostado. Sem os tres, e erro de protocolo.
#[derive(Clone, Debug, PartialEq)]
pub struct Veto {
    pub avaliador: Avaliador,
    pub lente: Lente,
    /// Hash do artefato que falseia a peca.
    pub artefato_falseador: String,
    /// Probabilidade declarada de que a peca falhe no teste. Fora de (0, 1) e dogma, e nao entra.
    pub p_falha: f64,
}

impl Veto {
    pub fn novo(avaliador: Avaliador, lente: Lente, artefato_falseador: &str, p_falha: f64) -> Veto {
        Veto { avaliador, lente, artefato_falseador: artefato_falseador.to_string(), p_falha }
    }
}

/// Cota de veto de um avaliador na janela corrente.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Autoridade {
    pub avaliador: Avaliador,
    pub janela: u32,
    pub vetos_restantes: u8,
}

impl Autoridade {
    pub fn nova(avaliador: Avaliador, janela: u32) -> Autoridade {
        Autoridade { avaliador, janela, vetos_restantes: VETOS_POR_JANELA }
    }
}

/// Erro de protocolo. Nao e fecho: e `Exit 1` para quem consome.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Recusa {
    /// G8.1: bancada menor que as oito lentes.
    BancadaInsuficiente { presentes: usize },
    /// G8.2: o artefato citado nao existe na peca.
    ArtefatoDesconhecido { lente: Lente },
    /// G8.3: veto acima da cota da janela.
    CotaEsgotada { avaliador: Avaliador },
    /// G8.3: dois vetos do mesmo avaliador na mesma lente.
    VetoDuplicado { avaliador: Avaliador },
    /// G8.5: aposta em 0 ou 1, ou fora do intervalo aberto.
    ApostaDogmatica { avaliador: Avaliador },
    /// G8.7: a lente nao pertence a este avaliador na atribuicao publicada.
    LenteForaDaAtribuicao { lente: Lente, avaliador: Avaliador },
    /// G8.4/Phi4: veto ou decisao sem unidade que pague.
    UnidadeAusente,
    /// Voto de quem nao esta na atribuicao nem na cota.
    SemAutoridade { avaliador: Avaliador },
}

impl Recusa {
    pub fn codigo(&self) -> &'static str {
        match self {
            Recusa::BancadaInsuficiente { .. } => "G8.1",
            Recusa::ArtefatoDesconhecido { .. } => "G8.2",
            Recusa::CotaEsgotada { .. } => "G8.3",
            Recusa::VetoDuplicado { .. } => "G8.3",
            Recusa::ApostaDogmatica { .. } => "G8.5",
            Recusa::LenteForaDaAtribuicao { .. } => "G8.7",
            Recusa::UnidadeAusente => "Phi4",
            Recusa::SemAutoridade { .. } => "G8.3",
        }
    }

    pub fn mensagem(&self) -> String {
        match self {
            Recusa::BancadaInsuficiente { presentes } => {
                format!("bancada com {presentes} avaliadores; oito lentes exigem oito corpos")
            }
            Recusa::ArtefatoDesconhecido { lente } => {
                format!("lente {} citou artefato que a peca nao declara", lente.0)
            }
            Recusa::CotaEsgotada { avaliador } => {
                format!("avaliador {} estourou a cota de veto da janela", avaliador.0)
            }
            Recusa::VetoDuplicado { avaliador } => {
                format!("avaliador {} vetou duas vezes a mesma lente", avaliador.0)
            }
            Recusa::ApostaDogmatica { avaliador } => {
                format!("avaliador {} apostou 0 ou 1: extremo dogmatico", avaliador.0)
            }
            Recusa::LenteForaDaAtribuicao { lente, avaliador } => format!(
                "lente {} nao pertence ao avaliador {} na atribuicao publicada",
                lente.0, avaliador.0
            ),
            Recusa::UnidadeAusente => "peca sem unidade que absorve o erro (Phi4)".to_string(),
            Recusa::SemAutoridade { avaliador } => {
                format!("avaliador {} nao tem posto nem cota nesta bancada", avaliador.0)
            }
        }
    }
}

/// Fecho: o que a bancada decidiu. Nao tem assinatura humana.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fecho {
    /// Prazo aberto e nenhum veto valido: nada decidido ainda.
    Pendente,
    /// Nenhum veto valido, prazo vencido e quorum de leitura cumprido.
    Sustentada { lacre: String, leituras: usize },
    /// Um veto valido basta: a conjuncao da banca A1-A8 ja era assim.
    NaoSustentada { motivo: String, vetos: usize },
    /// Prazo vencido sem quorum de leitura, ou peca sem condicao de deliberar.
    Arquivada { motivo: String },
}

impl Fecho {
    pub fn codigo(&self) -> &'static str {
        match self {
            Fecho::Pendente => "pendente",
            Fecho::Sustentada { .. } => "sustentada",
            Fecho::NaoSustentada { .. } => "nao-sustentada",
            Fecho::Arquivada { .. } => "arquivada",
        }
    }

    pub fn decide(&self) -> bool {
        !matches!(self, Fecho::Pendente)
    }
}

/// O que o fecho custou: consumo de cota a aplicar fora da funcao pura, e a transcricao lacrada.
#[derive(Clone, Debug, PartialEq)]
pub struct Recibo {
    pub consumos: Vec<(Avaliador, u8)>,
    pub leituras_validas: usize,
    pub vetos_validos: Vec<Avaliador>,
    pub lacre_transcricao: String,
}

impl Recibo {
    pub fn linha_markdown(&self, fecho: &Fecho) -> String {
        format!(
            "| {} | {} | {} | {} | {} |",
            fecho.codigo(),
            self.leituras_validas,
            self.vetos_validos.len(),
            self.consumos.iter().map(|(a, c)| format!("{}:-{}", a.0, c)).collect::<Vec<_>>().join(" "),
            &self.lacre_transcricao[..12]
        )
    }
}

/// Estado da bancada numa rodada: atribuicao publicada, leituras, vetos, cotas e o relogio do prazo.
#[derive(Clone, Debug, PartialEq)]
pub struct EstadoBancada {
    pub designacoes: [Designacao; 8],
    pub leituras: Vec<Leitura>,
    pub vetos: Vec<Veto>,
    pub orcamentos: Vec<Autoridade>,
    pub prazo_vencido: bool,
}

impl EstadoBancada {
    pub fn nova(designacoes: [Designacao; 8], janela: u32) -> EstadoBancada {
        let mut orcamentos: Vec<Autoridade> = Vec::new();
        for d in designacoes {
            if !orcamentos.iter().any(|a| a.avaliador == d.avaliador) {
                orcamentos.push(Autoridade::nova(d.avaliador, janela));
            }
        }
        EstadoBancada {
            designacoes,
            leituras: Vec::new(),
            vetos: Vec::new(),
            orcamentos,
            prazo_vencido: false,
        }
    }

    pub fn posto(&self, lente: Lente) -> Option<Avaliador> {
        self.designacoes.iter().find(|d| d.lente == lente).map(|d| d.avaliador)
    }

    pub fn cota(&self, avaliador: Avaliador) -> u8 {
        self.orcamentos
            .iter()
            .find(|a| a.avaliador == avaliador)
            .map(|a| a.vetos_restantes)
            .unwrap_or(0)
    }

    pub fn ler(mut self, leitura: Leitura) -> Self {
        self.leituras.push(leitura);
        self
    }

    pub fn vetar(mut self, veto: Veto) -> Self {
        self.vetos.push(veto);
        self
    }

    pub fn com_prazo_vencido(mut self) -> Self {
        self.prazo_vencido = true;
        self
    }
}

/// Sorteio auditado da atribuicao: Fisher-Yates sobre os avaliadores, semente misturada com a peca.
///
/// A lente nao e sorteada: a lente 1 continua sendo o cuidado de codigo. O que se sorteia e o corpo
/// que ocupa cada posto. Mesma peca, mesma semente, mesma matriz ate o ultimo bit.
pub fn atribuir(
    peca: &Peca,
    semente: u64,
    avaliadores: &[Avaliador],
) -> Result<[Designacao; 8], Recusa> {
    let mut pool: Vec<Avaliador> = avaliadores.to_vec();
    pool.sort();
    pool.dedup();
    if pool.len() < Lente::TODAS.len() {
        return Err(Recusa::BancadaInsuficiente { presentes: pool.len() });
    }

    let mut rng = Rng::novo(misturar(semente, &peca.hash_conteudo));
    for i in (1..pool.len()).rev() {
        let j = rng.uniforme((i + 1) as u64) as usize;
        pool.swap(i, j);
    }

    let mut designacoes = [Designacao { lente: Lente(1), avaliador: Avaliador(0) }; 8];
    for (indice, posto) in designacoes.iter_mut().enumerate() {
        *posto = Designacao { lente: Lente(indice as u8 + 1), avaliador: pool[indice] };
    }
    Ok(designacoes)
}

fn misturar(semente: u64, hash_hex: &str) -> u64 {
    let mut h = semente ^ 0x9E37_79B9_7F4A_7C15;
    for byte in hash_hex.as_bytes() {
        h = (h.rotate_left(5) ^ (*byte as u64)).wrapping_mul(0x0000_0100_0000_01B3);
    }
    h
}

/// Funcao pura de fecho: valida a rodada, devolve o fecho e o recibo. Sem estado, sem arbitro.
///
/// A ordem das checagens e a ordem da intransigencia: primeiro a rodada e valida (posto, artefato,
/// aposta, cota); depois um veto valido derruba; depois o prazo; depois o quorum. Nenhum caminho
/// devolve `Sustentada` por inercia.
pub fn fechar(peca: &Peca, estado: &EstadoBancada) -> Result<(Fecho, Recibo), Recusa> {
    let mut leituras_validas = 0usize;
    let mut lentes_lidas: Vec<Lente> = Vec::new();

    for leitura in &estado.leituras {
        exigir_posto(estado, leitura.lente, leitura.avaliador)?;
        if !peca.conhece(&leitura.artefato) {
            return Err(Recusa::ArtefatoDesconhecido { lente: leitura.lente });
        }
        if !lentes_lidas.contains(&leitura.lente) {
            lentes_lidas.push(leitura.lente);
            leituras_validas += 1;
        }
    }

    let mut cotas: Vec<(Avaliador, u8)> =
        estado.orcamentos.iter().map(|a| (a.avaliador, a.vetos_restantes)).collect();
    let mut vetos_validos: Vec<Avaliador> = Vec::new();
    let mut lentes_vetadas: Vec<Lente> = Vec::new();

    for veto in &estado.vetos {
        exigir_posto(estado, veto.lente, veto.avaliador)?;
        if !peca.conhece(&veto.artefato_falseador) {
            return Err(Recusa::ArtefatoDesconhecido { lente: veto.lente });
        }
        if !(veto.p_falha > 0.0 && veto.p_falha < 1.0) {
            return Err(Recusa::ApostaDogmatica { avaliador: veto.avaliador });
        }
        if lentes_vetadas.contains(&veto.lente) {
            return Err(Recusa::VetoDuplicado { avaliador: veto.avaliador });
        }
        let Some(cota) = cotas.iter_mut().find(|(a, _)| *a == veto.avaliador) else {
            return Err(Recusa::SemAutoridade { avaliador: veto.avaliador });
        };
        if cota.1 == 0 {
            return Err(Recusa::CotaEsgotada { avaliador: veto.avaliador });
        }
        cota.1 -= 1;
        lentes_vetadas.push(veto.lente);
        vetos_validos.push(veto.avaliador);
    }

    let mut consumos: Vec<(Avaliador, u8)> = Vec::new();
    for (avaliador, restante) in &cotas {
        let inicial = estado.cota(*avaliador);
        if *restante < inicial {
            consumos.push((*avaliador, inicial - *restante));
        }
    }

    let lacre = lacrar(peca, estado, leituras_validas, &vetos_validos);
    let recibo = Recibo {
        consumos,
        leituras_validas,
        vetos_validos: vetos_validos.clone(),
        lacre_transcricao: lacre.clone(),
    };

    if !vetos_validos.is_empty() {
        return Ok((
            Fecho::NaoSustentada {
                motivo: "veto valido com prova material e cota queimada".to_string(),
                vetos: vetos_validos.len(),
            },
            recibo,
        ));
    }

    if !estado.prazo_vencido {
        return Ok((Fecho::Pendente, recibo));
    }

    if leituras_validas < LEITURA_MINIMA {
        return Ok((
            Fecho::Arquivada {
                motivo: format!(
                    "prazo vencido com {leituras_validas} de {LEITURA_MINIMA} leituras exigidas"
                ),
            },
            recibo,
        ));
    }

    Ok((Fecho::Sustentada { lacre, leituras: leituras_validas }, recibo))
}

fn exigir_posto(estado: &EstadoBancada, lente: Lente, avaliador: Avaliador) -> Result<(), Recusa> {
    match estado.posto(lente) {
        Some(posto) if posto == avaliador => Ok(()),
        _ => Err(Recusa::LenteForaDaAtribuicao { lente, avaliador }),
    }
}

fn lacrar(peca: &Peca, estado: &EstadoBancada, leituras: usize, vetos: &[Avaliador]) -> String {
    let mut postos: Vec<String> = estado
        .designacoes
        .iter()
        .map(|d| format!("{}:{}", d.lente.0, d.avaliador.0))
        .collect();
    postos.sort();

    let mut apostas: Vec<String> = estado
        .vetos
        .iter()
        .map(|v| format!("{}:{}:{:?}", v.avaliador.0, v.lente.0, v.p_falha))
        .collect();
    apostas.sort();

    let mut vetantes: Vec<u32> = vetos.iter().map(|a| a.0).collect();
    vetantes.sort();

    let corpo = format!(
        "peca={};conteudo={};unidade={};artefatos={};postos={};leituras={};vetantes={};apostas={};prazo={}",
        peca.id,
        peca.hash_conteudo,
        peca.unidade.rotulo(),
        peca.artefatos.join(","),
        postos.join(","),
        leituras,
        vetantes.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(","),
        apostas.join(","),
        estado.prazo_vencido
    );
    sha256_hex(corpo.as_bytes())
}

/// Aplica o consumo do recibo as cotas. Unica funcao com `&mut` na bancada.
pub fn aplicar(recibo: &Recibo, orcamentos: &mut [Autoridade]) -> Result<(), Recusa> {
    for (avaliador, consumo) in &recibo.consumos {
        match orcamentos.iter_mut().find(|a| a.avaliador == *avaliador) {
            Some(a) if a.vetos_restantes >= *consumo => a.vetos_restantes -= *consumo,
            Some(_) => return Err(Recusa::CotaEsgotada { avaliador: *avaliador }),
            None => return Err(Recusa::SemAutoridade { avaliador: *avaliador }),
        }
    }
    Ok(())
}

/// A bancada inteira: peca, atribuicao publicada e estado corrente. O que o piloto roda por 14 dias.
#[derive(Clone, Debug, PartialEq)]
pub struct Bancada {
    pub peca: Peca,
    pub semente: u64,
    pub estado: EstadoBancada,
}

impl Bancada {
    pub fn nova(
        peca: Peca,
        semente: u64,
        avaliadores: &[Avaliador],
        janela: u32,
    ) -> Result<Bancada, Recusa> {
        let designacoes = atribuir(&peca, semente, avaliadores)?;
        let estado = EstadoBancada::nova(designacoes, janela);
        Ok(Bancada { peca, semente, estado })
    }

    pub fn fechar(&self) -> Result<(Fecho, Recibo), Recusa> {
        fechar(&self.peca, &self.estado)
    }

    /// Fecha e consome: e o unico caminho que gasta cota, e ele gasta de uma vez.
    pub fn fechar_e_consumir(&mut self) -> Result<(Fecho, Recibo), Recusa> {
        let (fecho, recibo) = self.fechar()?;
        aplicar(&recibo, &mut self.estado.orcamentos)?;
        Ok((fecho, recibo))
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    fn avaliadores() -> Vec<Avaliador> {
        (1..=10).map(Avaliador).collect()
    }

    fn peca() -> (Peca, String) {
        let unidade = Unidade::nova("sala-3").expect("unidade da sala");
        let p = Peca::nova("p-2026-01", b"conteudo-de-teste", unidade);
        p.com_artefato(b"contraprova-de-teste")
    }

    fn bancada(p: Peca, semente: u64) -> Bancada {
        Bancada::nova(p, semente, &avaliadores(), 1).expect("bancada com dez corpos")
    }

    fn ler_tudo(b: &Bancada) -> EstadoBancada {
        let mut estado = b.estado.clone();
        for d in estado.designacoes {
            let leitura = Leitura::nova(d.avaliador, d.lente, &b.peca.hash_conteudo);
            estado = estado.ler(leitura);
        }
        estado
    }

    fn veto_de(b: &Bancada, indice: usize, artefato: &str, p_falha: f64) -> Veto {
        let d = b.estado.designacoes[indice];
        Veto::novo(d.avaliador, d.lente, artefato, p_falha)
    }

    #[test]
    fn g8_1_atribuicao_reproduzivel() {
        let (p, _) = peca();
        let a = atribuir(&p, 2026, &avaliadores()).expect("atribui");
        let b = atribuir(&p, 2026, &avaliadores()).expect("atribui");
        assert_eq!(a, b);
    }

    #[test]
    fn g8_1_lente_preservada_e_corpos_distintos() {
        let (p, _) = peca();
        let d = atribuir(&p, 7, &avaliadores()).expect("atribui");
        for (indice, posto) in d.iter().enumerate() {
            assert_eq!(posto.lente, Lente(indice as u8 + 1));
        }
        let mut corpos: Vec<Avaliador> = d.iter().map(|x| x.avaliador).collect();
        corpos.sort();
        corpos.dedup();
        assert_eq!(corpos.len(), 8);
    }

    #[test]
    fn g8_1_semente_diferente_muda_matriz() {
        let (p, _) = peca();
        let base = atribuir(&p, 1, &avaliadores()).expect("atribui");
        let diverge = (2..40u64)
            .map(|s| atribuir(&p, s, &avaliadores()).expect("atribui"))
            .any(|m| m != base);
        assert!(diverge, "sementes diferentes sempre deveriam sortear outra matriz");
    }

    #[test]
    fn g8_1_bancada_curta_recusa() {
        let (p, _) = peca();
        let curta: Vec<Avaliador> = (1..=7).map(Avaliador).collect();
        let erro = atribuir(&p, 1, &curta).expect_err("sete corpos nao cobrem oito lentes");
        assert_eq!(erro.codigo(), "G8.1");
    }

    #[test]
    fn g8_2_veto_com_artefato_desconhecido_recusa() {
        let (p, _) = peca();
        let b = bancada(p, 3);
        let estado = b.estado.clone().vetar(veto_de(&b, 0, "hash-que-nao-existe", 0.7));
        let erro = fechar(&b.peca, &estado).expect_err("veto sem prova material");
        assert_eq!(erro.codigo(), "G8.2");
    }

    #[test]
    fn g8_2_leitura_sem_artefato_recusa() {
        let (p, _) = peca();
        let b = bancada(p, 3);
        let d = b.estado.designacoes[2];
        let estado = b.estado.clone().ler(Leitura::nova(d.avaliador, d.lente, "inventado"));
        let erro = fechar(&b.peca, &estado).expect_err("leitura de artefato inexistente");
        assert_eq!(erro.codigo(), "G8.2");
    }

    #[test]
    fn g8_3_cota_estourada_recusa() {
        let (p, artefato) = peca();
        let b = bancada(p, 5);
        let alvo = b.estado.designacoes[0].avaliador;

        // A cota e da janela, nao da rodada: duas pecas anteriores ja queimaram os dois vetos.
        let mut estado = b.estado.clone().vetar(veto_de(&b, 0, &artefato, 0.6));
        let (_, primeiro) = fechar(&b.peca, &estado).expect("primeiro veto vale");
        aplicar(&primeiro, &mut estado.orcamentos).expect("queima a primeira cota");
        estado.vetos.clear();
        estado = estado.vetar(veto_de(&b, 0, &artefato, 0.65));
        let (_, segundo) = fechar(&b.peca, &estado).expect("segundo veto vale");
        aplicar(&segundo, &mut estado.orcamentos).expect("queima a segunda cota");
        estado.leituras.clear();
        estado.vetos = vec![veto_de(&b, 0, &artefato, 0.7)];

        let erro = fechar(&b.peca, &estado).expect_err("cota da janela ja zerada");
        assert_eq!(erro.codigo(), "G8.3");
        assert_eq!(estado.cota(alvo), 0);
    }

    #[test]
    fn g8_3_cota_de_janela_cheia_zera_o_poder_de_veto() {
        let (p, artefato) = peca();
        let b = bancada(p, 5);
        let mut estado = b.estado.clone();
        let alvo = estado.designacoes[3].avaliador;
        let cota = estado.orcamentos.iter_mut().find(|a| a.avaliador == alvo).expect("posto");
        cota.vetos_restantes = 0;
        estado = estado.vetar(veto_de(&b, 3, &artefato, 0.8));

        let erro = fechar(&b.peca, &estado).expect_err("veto de quem ja gastou a cota");
        assert_eq!(erro.codigo(), "G8.3");
    }

    #[test]
    fn g8_3_veto_duplicado_recusa() {
        let (p, artefato) = peca();
        let b = bancada(p, 5);
        let estado = b
            .estado
            .clone()
            .vetar(veto_de(&b, 0, &artefato, 0.6))
            .vetar(veto_de(&b, 0, &artefato, 0.55));
        let erro = fechar(&b.peca, &estado).expect_err("dois vetos na mesma lente");
        assert_eq!(erro.codigo(), "G8.3");
    }

    #[test]
    fn g8_5_aposta_dogmatica_recusa() {
        let (p, artefato) = peca();
        let b = bancada(p, 11);
        for aposta in [0.0, 1.0, -0.2, 1.4, f64::NAN] {
            let estado = b.estado.clone().vetar(veto_de(&b, 0, &artefato, aposta));
            let erro = fechar(&b.peca, &estado).expect_err("extremo dogmatico no veto");
            assert_eq!(erro.codigo(), "G8.5");
        }
    }

    #[test]
    fn g8_7_lente_fora_da_atribuicao_recusa() {
        let (p, artefato) = peca();
        let b = bancada(p, 13);
        let d0 = b.estado.designacoes[0];
        let d1 = b.estado.designacoes[1];
        let invasao = Veto::novo(d1.avaliador, d0.lente, &artefato, 0.5);
        let estado = b.estado.clone().vetar(invasao);
        let erro = fechar(&b.peca, &estado).expect_err("voto em lente alheia");
        assert_eq!(erro.codigo(), "G8.7");
    }

    #[test]
    fn g8_4_silencio_antes_do_prazo_pendente() {
        let (p, _) = peca();
        let b = bancada(p, 17);
        let (fecho, recibo) = b.fechar().expect("rodada valida");
        assert_eq!(fecho, Fecho::Pendente);
        assert!(recibo.consumos.is_empty());
        assert!(!fecho.decide());
    }

    #[test]
    fn g8_4_silencio_apos_prazo_arquiva() {
        let (p, _) = peca();
        let b = bancada(p, 17);
        let estado = b.estado.clone().com_prazo_vencido();
        let (fecho, _) = fechar(&b.peca, &estado).expect("rodada valida");
        match fecho {
            Fecho::Arquivada { motivo } => assert!(motivo.contains("prazo vencido")),
            outro => panic!("silencio com prazo vencido nao sustenta: {outro:?}"),
        }
    }

    #[test]
    fn g8_4_quorum_completo_sustenta() {
        let (p, _) = peca();
        let b = bancada(p, 19);
        let estado = ler_tudo(&b).com_prazo_vencido();
        let (fecho, recibo) = fechar(&b.peca, &estado).expect("rodada valida");
        assert_eq!(recibo.leituras_validas, 8);
        assert!(matches!(fecho, Fecho::Sustentada { leituras: 8, .. }));
    }

    #[test]
    fn g8_4_quorum_insuficiente_arquiva() {
        let (p, _) = peca();
        let b = bancada(p, 19);
        let mut estado = b.estado.clone();
        for d in b.estado.designacoes.iter().take(LEITURA_MINIMA - 1) {
            estado = estado.ler(Leitura::nova(d.avaliador, d.lente, &b.peca.hash_conteudo));
        }
        let estado = estado.com_prazo_vencido();
        let (fecho, _) = fechar(&b.peca, &estado).expect("rodada valida");
        assert!(matches!(fecho, Fecho::Arquivada { .. }));
    }

    #[test]
    fn g8_6_veto_valido_derruba_e_consome() {
        let (p, artefato) = peca();
        let mut b = bancada(p, 23);
        b.estado = b.estado.clone().vetar(veto_de(&b, 4, &artefato, 0.72));
        let alvo = b.estado.designacoes[4].avaliador;
        let antes = b.estado.cota(alvo);

        let (fecho, recibo) = b.fechar_e_consumir().expect("rodada valida");
        assert!(matches!(fecho, Fecho::NaoSustentada { vetos: 1, .. }));
        assert_eq!(recibo.consumos, vec![(alvo, 1)]);
        assert_eq!(b.estado.cota(alvo), antes - 1);
    }

    #[test]
    fn g8_6_veto_vence_mesmo_sem_quorum() {
        let (p, artefato) = peca();
        let b = bancada(p, 29);
        let estado = b.estado.clone().vetar(veto_de(&b, 0, &artefato, 0.9));
        let (fecho, _) = fechar(&b.peca, &estado).expect("rodada valida");
        assert_eq!(fecho.codigo(), "nao-sustentada");
    }

    #[test]
    fn g8_8_lacre_deterministico_e_sensivel() {
        let (p, _) = peca();
        let b = bancada(p, 31);
        let estado = ler_tudo(&b).com_prazo_vencido();
        let (f1, r1) = fechar(&b.peca, &estado).expect("rodada valida");
        let (f2, r2) = fechar(&b.peca, &estado).expect("rodada valida");
        assert_eq!(f1, f2);
        assert_eq!(r1.lacre_transcricao, r2.lacre_transcricao);

        let mut adulterado = b.peca.clone();
        adulterado.artefatos.push(sha256_hex(b"outro-artefato"));
        let (_, r3) = fechar(&adulterado, &estado).expect("rodada valida");
        assert_ne!(r1.lacre_transcricao, r3.lacre_transcricao);
    }

    #[test]
    fn g8_8_aplicar_fora_da_funcao_pura() {
        let (p, artefato) = peca();
        let b = bancada(p, 37);
        let estado = b.estado.clone().vetar(veto_de(&b, 1, &artefato, 0.4));
        let (_, recibo) = fechar(&b.peca, &estado).expect("rodada valida");

        let mut orcamentos = b.estado.orcamentos.clone();
        aplicar(&recibo, &mut orcamentos).expect("consome uma vez");
        aplicar(&recibo, &mut orcamentos).expect("consome a segunda cota da janela");
        let erro = aplicar(&recibo, &mut orcamentos).expect_err("cota ja consumida");
        assert_eq!(erro.codigo(), "G8.3");
    }

    #[test]
    fn phi4_peca_sem_unidade_recusa() {
        let erro = Unidade::nova("   ").expect_err("unidade vazia");
        assert_eq!(erro.codigo(), "Phi4");
    }

    #[test]
    fn phi4_ficha_sem_quem_paga_recusa() {
        use stochology_core::{Camada, TipoIncerteza};
        let ficha = Ficha::nova("f-1", Camada::Modelo, TipoIncerteza::Epistemica);
        let erro = Peca::de_ficha(&ficha).expect_err("ficha sem quem paga");
        assert_eq!(erro.codigo(), "Phi4");
    }

    #[test]
    fn recibo_gera_linha_sem_parecer() {
        let (p, _) = peca();
        let b = bancada(p, 41);
        let estado = ler_tudo(&b).com_prazo_vencido();
        let (fecho, recibo) = fechar(&b.peca, &estado).expect("rodada valida");
        let linha = recibo.linha_markdown(&fecho);
        assert!(linha.starts_with("| sustentada | 8 | 0 |"));
    }
}
