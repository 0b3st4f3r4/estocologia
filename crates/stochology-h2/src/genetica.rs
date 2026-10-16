//! A validação por intervenção na genética, com e sem a **segunda linha**.
//!
//! O registro da pesquisa diz, na letra: "A réplica mostra *um gene num mutante*, e não um efeito medido com
//! resolução. Cada mutante é um ambiente de uma linha só, e o menor p-valor possível ali é 1/1 000." Este módulo
//! mede o que a segunda linha muda — e mede em desenho, não em retórica: os dois números que importam são
//! **quantos mutantes passam** e **qual é o piso do desenho** em cada arranjo.
//!
//! # O desenho
//!
//! \`G\` mutantes, \`L\` linhas por mutante e \`n\` indivíduos por linha. O fenótipo é
//! \`base + efeito do mutante + efeito da linha + ruído\`. Dois mutantes são plantados de propósito:
//!
//! - **o mover verdadeiro**: o efeito aparece nas duas linhas dele;
//! - **o artefato de linha**: o efeito aparece em **uma** linha só — é o caso que a validação de uma linha não
//!   consegue distinguir de um mover, e que a segunda linha pega.
//!
//! A validação, em cada arranjo:
//!
//! - **sem a segunda linha**: usa a primeira linha de cada mutante e aceita quem fica abaixo de `alfa/G`
//!   (Bonferroni sobre os mutantes). O efeito é **um ponto**, e o p sai da cauda da média do mutante na
//!   distribuição da linha, **preso no piso do desenho** de `1/(B+1)` — o piso é o que os `B` sorteios
//!   resolvem, e é ele que decide quando o efeito é grande o bastante para encostar no chão.
//! - **com a segunda linha**: exige o efeito nas **duas** linhas, e o p combinado é o produto — o piso cai para
//!   \`(1/(B+1))^2\` e cada efeito ganha **intervalo** (a faixa entre as duas estimativas de linha).
//!
//! Declarado e não implementado: a leitura dos dados reais de levedura (a base saiu com o andaime Python) e o
//! desenho com mais de duas linhas, que o instrumento aceita como parâmetro mas não é o caso declarado.

use stochology_core::Rng;

/// O arranjo do experimento.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Desenho {
    /// Quantos genes deletados entram na varredura.
    pub mutantes: usize,
    /// Quantas linhas independentes por mutante (1 é o desenho de hoje).
    pub linhas_por_mutante: usize,
    /// Quantos indivíduos por linha.
    pub individuos_por_linha: usize,
    /// Quantos sorteios de permutação por teste.
    pub permutacoes: usize,
    /// Nível declarado antes de Bonferroni.
    pub alfa: f64,
}

impl Default for Desenho {
    fn default() -> Self {
        Desenho { mutantes: 40, linhas_por_mutante: 1, individuos_por_linha: 60, permutacoes: 999, alfa: 0.05 }
    }
}

impl Desenho {
    /// O piso do desenho: o menor p-valor que a validação consegue produzir. Cai com a potência do número de
    /// linhas, porque as linhas são independentes e o p combinado é o produto.
    pub fn piso(&self) -> f64 {
        (1.0 / (self.permutacoes as f64 + 1.0)).powi(self.linhas_por_mutante as i32)
    }

    /// O piso do desenho quando a segunda linha entra: o produto dos dois pisos de linha.
    pub fn piso_com_a_segunda_linha(&self) -> f64 {
        (1.0 / (self.permutacoes as f64 + 1.0)).powi(2)
    }

    /// O nível de cada teste depois de Bonferroni sobre os mutantes.
    pub fn alfa_corrigido(&self) -> f64 {
        self.alfa / self.mutantes as f64
    }
}

/// O que a validação devolve para um mutante.
#[derive(Debug, Clone, PartialEq)]
pub struct Veredito {
    pub mutante: usize,
    /// A estimativa do efeito em cada linha.
    pub efeito_por_linha: Vec<f64>,
    /// O p-valor combinado das linhas testadas.
    pub p: f64,
    /// A faixa entre a maior e a menor estimativa de linha; zero quando há uma linha só.
    pub largura_do_intervalo: f64,
    pub passa: bool,
}

/// O resultado dos dois arranjos, lado a lado.
#[derive(Debug, Clone, PartialEq)]
pub struct Comparacao {
    pub sem_segunda_linha: Vec<Veredito>,
    pub com_segunda_linha: Vec<Veredito>,
    pub piso_sem: f64,
    pub piso_com: f64,
    pub alfa_corrigido: f64,
    /// O mutante plantado cujo efeito se reproduz nas duas linhas.
    pub mover_verdadeiro: usize,
    /// O mutante plantado cujo efeito aparece numa linha só.
    pub artefato_de_linha: usize,
}

impl Comparacao {
    pub fn quantos_passam_sem(&self) -> usize {
        self.sem_segunda_linha.iter().filter(|v| v.passa).count()
    }
    pub fn quantos_passam_com(&self) -> usize {
        self.com_segunda_linha.iter().filter(|v| v.passa).count()
    }
}

/// Um mundo sintético: os mutantes, as linhas e os indivíduos, com os dois casos plantados.
struct Mundo {
    desenho: Desenho,
    /// fenótipo[mutante][linha][indivíduo]
    fenotipo: Vec<Vec<Vec<f64>>>,
    /// o mutante cujo efeito se reproduz nas duas linhas
    mover_verdadeiro: usize,
    /// o mutante cujo efeito aparece em uma linha só
    artefato_de_linha: usize,
}

fn gerar(desenho: Desenho, semente: u64) -> Mundo {
    let mut rng = Rng::novo(semente);
    let normal = |rng: &mut Rng| {
        let u1 = ((rng.proximo() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
        let u2 = ((rng.proximo() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
        (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
    };
    let mover_verdadeiro = 7 % desenho.mutantes;
    let artefato_de_linha = 23 % desenho.mutantes;
    let mut fenotipo = Vec::with_capacity(desenho.mutantes);
    for m in 0..desenho.mutantes {
        let efeito_do_mutante = if m == mover_verdadeiro { 0.9 } else { 0.0 };
        let mut linhas = Vec::with_capacity(desenho.linhas_por_mutante);
        for l in 0..desenho.linhas_por_mutante {
            let efeito_da_linha = if m == artefato_de_linha && l == 0 { 0.9 } else { 0.0 };
            let deslocamento = 0.15 * normal(&mut rng);
            let mut individuos = Vec::with_capacity(desenho.individuos_por_linha);
            for _ in 0..desenho.individuos_por_linha {
                individuos.push(
                    5.0 + efeito_do_mutante + efeito_da_linha + deslocamento + normal(&mut rng),
                );
            }
            linhas.push(individuos);
        }
        fenotipo.push(linhas);
    }
    Mundo { desenho, fenotipo, mover_verdadeiro, artefato_de_linha }
}

/// O efeito de cada mutante numa linha e o p da linha: a média do mutante contra a distribuição da linha
/// (cauda normal da média, com o desvio agrupado), **presa no piso do desenho** — que é o que os 999 sorteios
/// resolvem. É esse piso que a segunda linha muda.
fn efeitos_da_linha(mundo: &Mundo, linha: usize, _semente: u64) -> Vec<(f64, f64)> {
    let n = mundo.desenho.individuos_por_linha as f64;
    let medias: Vec<f64> = (0..mundo.desenho.mutantes)
        .map(|m| {
            let valores = &mundo.fenotipo[m][linha];
            valores.iter().sum::<f64>() / n
        })
        .collect();
    let referencia = medias.iter().sum::<f64>() / medias.len() as f64;
    // desvio agrupado dentro da linha, entre os indivíduos de todos os mutantes
    let mut soma = 0.0;
    let mut graus: f64 = 0.0;
    for m in 0..mundo.desenho.mutantes {
        let deslocamento = medias[m] - referencia;
        for v in &mundo.fenotipo[m][linha] {
            soma += (v - referencia - deslocamento).powi(2);
            graus += 1.0;
        }
    }
    let desvio = (soma / graus.max(1.0)).sqrt().max(1e-9);
    let piso = 1.0 / (mundo.desenho.permutacoes as f64 + 1.0);
    medias
        .iter()
        .map(|media| {
            let efeito = media - referencia;
            let z = efeito.abs() * n.sqrt() / desvio;
            let p = 2.0 * (0.5 * (1.0 - erf(z / std::f64::consts::SQRT_2))).max(0.0);
            (efeito, p.max(piso))
        })
        .collect()
}

/// A função erro, por Abramowitz-Stegun 7.1.26 (erro < 1,5e-7).
fn erf(x: f64) -> f64 {
    let sinal = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.327_591_1 * x);
    let polinomio = t
        * (0.254_829_592
            + t * (-0.284_496_736
                + t * (1.421_413_741 + t * (-1.453_152_027 + t * 1.061_405_429))));
    sinal * (1.0 - polinomio * (-x * x).exp())
}

/// Roda os dois arranjos sobre o mesmo mundo sintético e devolve o que cada um decide.
pub fn comparar(desenho: Desenho, semente: u64) -> Comparacao {
    let mundo = gerar(desenho, semente);
    let alfa_corrigido = desenho.alfa_corrigido();
    let primeira = efeitos_da_linha(&mundo, 0, semente ^ 11);
    let segunda = efeitos_da_linha(&mundo, 1.min(desenho.linhas_por_mutante - 1), semente ^ 977);
    let mut sem = Vec::with_capacity(desenho.mutantes);
    let mut com = Vec::with_capacity(desenho.mutantes);
    for m in 0..desenho.mutantes {
        let (efeito, p) = primeira[m];
        sem.push(Veredito {
            mutante: m,
            efeito_por_linha: vec![efeito],
            p,
            largura_do_intervalo: 0.0,
            passa: p < alfa_corrigido,
        });
        let (efeito_2, p_2) = segunda[m];
        // com a segunda linha o efeito tem de estar nas DUAS: o p combinado é o produto (e cai ao piso novo),
        // e a aprovação é conjuntiva — é isso que separa o mover do artefato de linha
        let p_combinado = (p * p_2).max(desenho.piso_com_a_segunda_linha());
        com.push(Veredito {
            mutante: m,
            efeito_por_linha: vec![efeito, efeito_2],
            p: p_combinado,
            largura_do_intervalo: (efeito - efeito_2).abs(),
            passa: p < alfa_corrigido && p_2 < alfa_corrigido,
        });
    }
    Comparacao {
        sem_segunda_linha: sem,
        com_segunda_linha: com,
        piso_sem: Desenho { linhas_por_mutante: 1, ..desenho }.piso(),
        piso_com: desenho.piso_com_a_segunda_linha(),
        alfa_corrigido,
        mover_verdadeiro: mundo.mover_verdadeiro,
        artefato_de_linha: mundo.artefato_de_linha,
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn o_piso_cai_com_a_potencia_das_linhas() {
        let d = Desenho::default();
        let sem = Desenho { linhas_por_mutante: 1, ..d }.piso();
        let com = Desenho { linhas_por_mutante: 2, ..d }.piso();
        assert!((sem - 1.0 / 1000.0).abs() < 1e-12, "piso sem: {sem}");
        assert!((com - 1.0 / 1_000_000.0).abs() < 1e-15, "piso com: {com}");
        assert!(com < sem);
    }

    #[test]
    fn sem_a_segunda_linha_o_intervalo_e_um_ponto() {
        let c = comparar(Desenho::default(), 2019);
        assert!(c.sem_segunda_linha.iter().all(|v| v.efeito_por_linha.len() == 1));
        assert!(c.sem_segunda_linha.iter().all(|v| v.largura_do_intervalo == 0.0));
    }

    #[test]
    fn com_a_segunda_linha_todo_efeito_tem_intervalo() {
        let c = comparar(Desenho { linhas_por_mutante: 2, ..Desenho::default() }, 2019);
        assert!(c.com_segunda_linha.iter().all(|v| v.efeito_por_linha.len() == 2));
        let com_intervalo = c.com_segunda_linha.iter().filter(|v| v.largura_do_intervalo > 0.0).count();
        assert!(com_intervalo > c.com_segunda_linha.len() / 2, "quase todos com faixa: {com_intervalo}");
    }

    #[test]
    fn o_artefato_de_linha_passa_sem_a_segunda_linha() {
        let c = comparar(Desenho::default(), 2019);
        let artefato = c.artefato_de_linha;
        let v = &c.sem_segunda_linha[artefato];
        assert!(v.passa, "o artefato de linha passa na validação de uma linha: {v:?}");
    }

    #[test]
    fn a_segunda_linha_separa_o_artefato_do_mover() {
        let c = comparar(Desenho { linhas_por_mutante: 2, ..Desenho::default() }, 2019);
        let artefato = c.artefato_de_linha;
        let mover = c.mover_verdadeiro;
        let a = &c.com_segunda_linha[artefato];
        assert!(
            a.largura_do_intervalo > 0.3,
            "o artefato tem faixa grande entre as linhas: {a:?}"
        );
        assert!(!a.passa, "e por isso não passa: {a:?}");
        let m = &c.com_segunda_linha[mover];
        assert!(m.passa, "o mover verdadeiro passa: {m:?}");
    }

    #[test]
    fn o_desenho_e_deterministico() {
        let d = Desenho::default();
        assert_eq!(comparar(d, 2019), comparar(d, 2019));
        assert_ne!(comparar(d, 2019), comparar(d, 2020));
    }
}
