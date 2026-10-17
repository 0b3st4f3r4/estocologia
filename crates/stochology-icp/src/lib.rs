//! stochology-icp — predicao causal invariante sobre ambientes declarados.
//!
//! O metodo: dentro de cada ambiente, regride o alvo no conjunto candidato; se a distribuicao do
//! residuo nao muda entre ambientes, o conjunto e aceito como invariante. O acaso entra pela
//! estatistica de Kolmogorov-Smirnov, com limiar que depende de n e de alfa.
//!
//! Declarado e nao implementado: teste de permutacao do modelo nulo (aqui o nulo e o proprio limiar
//! assintotico de KS) e os ambientes que vem de base externa, que moram em `dados-reais/`.

use stochology_core::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum Erro {
    SemLinhas,
    LinhasDesiguais,
    ColunaAusente(usize),
    AlvoEntrePreditores(usize),
    PreditoresDemais(usize),
    SistemaSingular,
    AmbienteUnico,
}

impl std::fmt::Display for Erro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Erro::SemLinhas => write!(f, "ambiente sem linhas"),
            Erro::LinhasDesiguais => write!(f, "colunas com numero de linhas diferente"),
            Erro::ColunaAusente(j) => write!(f, "coluna {j} ausente"),
            Erro::AlvoEntrePreditores(j) => write!(f, "o alvo aparece como preditor: coluna {j}"),
            Erro::PreditoresDemais(n) => write!(f, "{n} candidatos: a enumeracao para em 16"),
            Erro::SistemaSingular => write!(f, "sistema singular: preditores colineares"),
            Erro::AmbienteUnico => write!(f, "invariância precisa de dois ambientes ou mais"),
        }
    }
}

impl std::error::Error for Erro {}

/// Um ambiente: o mesmo conjunto de colunas medido em condicao diferente.
#[derive(Debug, Clone, PartialEq)]
pub struct Ambiente {
    pub nome: String,
    pub colunas: Vec<Vec<f64>>,
}

impl Ambiente {
    pub fn novo(nome: &str, colunas: Vec<Vec<f64>>) -> Result<Self, Erro> {
        let linhas = colunas.first().map(Vec::len).ok_or(Erro::SemLinhas)?;
        if linhas == 0 {
            return Err(Erro::SemLinhas);
        }
        if colunas.iter().any(|c| c.len() != linhas) {
            return Err(Erro::LinhasDesiguais);
        }
        Ok(Ambiente {
            nome: nome.to_string(),
            colunas,
        })
    }

    pub fn linhas(&self) -> usize {
        self.colunas.first().map(Vec::len).unwrap_or(0)
    }

    pub fn coluna(&self, j: usize) -> Result<&Vec<f64>, Erro> {
        self.colunas.get(j).ok_or(Erro::ColunaAusente(j))
    }

    /// Junta as linhas de `por_grupo` em medias: a unidade deixa de ser a linha e passa a ser o grupo.
    pub fn media_por_grupo(&self, por_grupo: usize) -> Result<Ambiente, Erro> {
        if por_grupo == 0 {
            return Err(Erro::LinhasDesiguais);
        }
        let mut colunas = Vec::with_capacity(self.colunas.len());
        for coluna in &self.colunas {
            let mut medias = Vec::with_capacity(coluna.len() / por_grupo);
            for bloco in coluna.chunks(por_grupo) {
                if bloco.len() < por_grupo {
                    break;
                }
                medias.push(bloco.iter().sum::<f64>() / por_grupo as f64);
            }
            colunas.push(medias);
        }
        if colunas.first().map(Vec::len).unwrap_or(0) == 0 {
            return Err(Erro::SemLinhas);
        }
        Ambiente::novo(&format!("{}#media{}", self.nome, por_grupo), colunas)
    }
}

/// Minimos quadrados com intercepto. Devolve os coeficientes (intercepto primeiro) e o residuo.
pub fn regressao(alvo: &[f64], preditores: &[Vec<f64>]) -> Result<(Vec<f64>, Vec<f64>), Erro> {
    let n = alvo.len();
    if n == 0 {
        return Err(Erro::SemLinhas);
    }
    if preditores.iter().any(|p| p.len() != n) {
        return Err(Erro::LinhasDesiguais);
    }
    let p = preditores.len() + 1;
    let mut a = vec![vec![0.0_f64; p]; p];
    let mut b = vec![0.0_f64; p];
    // primeira coluna e o intercepto
    for i in 0..n {
        let mut linha = Vec::with_capacity(p);
        linha.push(1.0);
        for preditor in preditores {
            linha.push(preditor[i]);
        }
        for u in 0..p {
            b[u] += linha[u] * alvo[i];
            for v in 0..p {
                a[u][v] += linha[u] * linha[v];
            }
        }
    }
    let coef = resolver(a, b)?;
    let mut residuo = Vec::with_capacity(n);
    for i in 0..n {
        let mut ajuste = coef[0];
        for (u, preditor) in preditores.iter().enumerate() {
            ajuste += coef[u + 1] * preditor[i];
        }
        residuo.push(alvo[i] - ajuste);
    }
    Ok((coef, residuo))
}

fn resolver(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Result<Vec<f64>, Erro> {
    let n = b.len();
    for coluna in 0..n {
        let mut pivo = coluna;
        for linha in coluna + 1..n {
            if a[linha][coluna].abs() > a[pivo][coluna].abs() {
                pivo = linha;
            }
        }
        if a[pivo][coluna].abs() < 1e-12 {
            return Err(Erro::SistemaSingular);
        }
        a.swap(coluna, pivo);
        b.swap(coluna, pivo);
        let divisor = a[coluna][coluna];
        for v in coluna..n {
            a[coluna][v] /= divisor;
        }
        b[coluna] /= divisor;
        for linha in 0..n {
            if linha == coluna {
                continue;
            }
            let fator = a[linha][coluna];
            if fator == 0.0 {
                continue;
            }
            for v in coluna..n {
                a[linha][v] -= fator * a[coluna][v];
            }
            b[linha] -= fator * b[coluna];
        }
    }
    Ok(b)
}

/// Estatistica de Kolmogorov-Smirnov de duas amostras (distancia maxima entre as acumuladas).
pub fn ks(a: &[f64], b: &[f64]) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 1.0;
    }
    let mut ordenada_a: Vec<f64> = a.to_vec();
    let mut ordenada_b: Vec<f64> = b.to_vec();
    ordenada_a.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    ordenada_b.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    let (n, m) = (a.len() as f64, b.len() as f64);
    let (mut i, mut j, mut maior) = (0usize, 0usize, 0.0_f64);
    while i < a.len() || j < b.len() {
        let proximo = match (ordenada_a.get(i), ordenada_b.get(j)) {
            (Some(x), Some(y)) => x.min(*y),
            (Some(x), None) => *x,
            (None, Some(y)) => *y,
            (None, None) => break,
        };
        while i < a.len() && ordenada_a[i] <= proximo {
            i += 1;
        }
        while j < b.len() && ordenada_b[j] <= proximo {
            j += 1;
        }
        let distancia = (i as f64 / n - j as f64 / m).abs();
        if distancia > maior {
            maior = distancia;
        }
    }
    maior
}

/// Limiar assintotico de KS de duas amostras: `c(alfa) * sqrt((n+m)/(n*m))`.
pub fn limiar_ks(n: usize, m: usize, alfa: f64) -> f64 {
    if n == 0 || m == 0 {
        return 0.0;
    }
    let c = (-0.5 * (alfa / 2.0).ln()).sqrt();
    c * (((n + m) as f64) / ((n * m) as f64)).sqrt()
}

fn residuos_do_ambiente(
    ambiente: &Ambiente,
    alvo: usize,
    preditores: &[usize],
) -> Result<Vec<f64>, Erro> {
    if preditores.contains(&alvo) {
        return Err(Erro::AlvoEntrePreditores(alvo));
    }
    let colunas: Vec<Vec<f64>> = preditores
        .iter()
        .map(|j| ambiente.coluna(*j).cloned())
        .collect::<Result<_, _>>()?;
    let (_, residuo) = regressao(ambiente.coluna(alvo)?, &colunas)?;
    Ok(residuo)
}

/// O conjunto e aceito quando o residuo nao muda de ambiente para ambiente, ao nivel `alfa`.
pub fn conjunto_aceito(
    ambientes: &[Ambiente],
    alvo: usize,
    preditores: &[usize],
    alfa: f64,
) -> Result<bool, Erro> {
    if ambientes.len() < 2 {
        return Err(Erro::AmbienteUnico);
    }
    if preditores.len() > 16 {
        return Err(Erro::PreditoresDemais(preditores.len()));
    }
    let residuos: Vec<Vec<f64>> = ambientes
        .iter()
        .map(|ambiente| residuos_do_ambiente(ambiente, alvo, preditores))
        .collect::<Result<_, _>>()?;
    for i in 0..residuos.len() {
        for j in i + 1..residuos.len() {
            let limiar = limiar_ks(residuos[i].len(), residuos[j].len(), alfa);
            if ks(&residuos[i], &residuos[j]) > limiar {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

/// Todos os subconjuntos de `candidatos` aceitos, do menor para o maior.
pub fn familia_aceita(
    ambientes: &[Ambiente],
    alvo: usize,
    candidatos: &[usize],
    alfa: f64,
) -> Result<Vec<Vec<usize>>, Erro> {
    if candidatos.len() > 16 {
        return Err(Erro::PreditoresDemais(candidatos.len()));
    }
    let mut aceitos = Vec::new();
    for mascara in 0u32..(1u32 << candidatos.len()) {
        let conjunto: Vec<usize> = (0..candidatos.len())
            .filter(|k| mascara & (1 << k) != 0)
            .map(|k| candidatos[k])
            .collect();
        if conjunto_aceito(ambientes, alvo, &conjunto, alfa)? {
            aceitos.push(conjunto);
        }
    }
    aceitos.sort_by_key(|c| (c.len(), c.clone()));
    Ok(aceitos)
}

/// A area que sobrevive: a interseccao dos conjuntos aceitos. Familia vazia da area vazia — e, se o
/// conjunto vazio for aceito, tambem nao ha area a defender: nenhum preditor e necessario.
pub fn area(aceitos: &[Vec<usize>]) -> Vec<usize> {
    let primeiro = match aceitos.first() {
        Some(c) => c,
        None => return Vec::new(),
    };
    let mut area: Vec<usize> = primeiro.clone();
    for conjunto in &aceitos[1..] {
        area.retain(|j| conjunto.contains(j));
    }
    area
}

/// Quantos subconjuntos um conjunto de `n` candidatos tem: `2^n`.
pub fn contar_conjuntos(n: usize) -> u64 {
    1u64 << n
}

/// Normal padrao por Box-Muller, para os mundos sinteticos dos testes.
pub fn normal(rng: &mut Rng) -> f64 {
    let u1 = ((rng.proximo() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
    let u2 = ((rng.proximo() >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
    (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
}

#[cfg(test)]
mod testes {
    use super::*;

    /// Mundo sintetico com a marginal mudando de ambiente para ambiente: X1 tem escala 1 no
    /// ambiente A e 3 no B, o ruido do alvo tem escala propria por ambiente e X2 carrega o rotulo.
    fn mundo(semente: u64, por_ambiente: usize, ruido_a: f64, ruido_b: f64) -> Vec<Ambiente> {
        let mut rng = Rng::novo(semente);
        let mut ambientes = Vec::new();
        for (indice, ambiente) in ["A", "B"].iter().enumerate() {
            let e = indice as f64;
            let escala = if indice == 0 { 1.0 } else { 3.0 };
            let ruido = if indice == 0 { ruido_a } else { ruido_b };
            let mut x1 = Vec::new();
            let mut x2 = Vec::new();
            let mut y = Vec::new();
            for _ in 0..por_ambiente {
                let a = escala * normal(&mut rng);
                x1.push(a);
                x2.push(0.9 * e + normal(&mut rng));
                y.push(2.0 * a + ruido * normal(&mut rng));
            }
            ambientes.push(Ambiente::novo(ambiente, vec![y, x1, x2]).expect("ambiente sintetico"));
        }
        ambientes
    }

    #[test]
    fn preditor_invariante_e_aceito_e_confundido_e_recusado() {
        let ambientes = mundo(2026, 120, 1.0, 1.0);
        assert!(conjunto_aceito(&ambientes, 0, &[1], 0.05).expect("testa"));
        assert!(!conjunto_aceito(&ambientes, 0, &[2], 0.05).expect("testa"));
        let familia = familia_aceita(&ambientes, 0, &[1, 2], 0.05).expect("familia");
        assert!(
            familia.contains(&vec![1]),
            "o invariante entra: {familia:?}"
        );
        assert!(
            !familia.contains(&vec![2]),
            "o confundido fica fora: {familia:?}"
        );
        assert_eq!(area(&familia), vec![1]);
    }

    #[test]
    fn area_vazia_quando_nada_e_invariante() {
        // o ruido do alvo muda de escala entre ambientes: nem o conjunto vazio, nem {X1}, nem {X2}.
        let ambientes = mundo(7, 80, 1.0, 4.0);
        let familia = familia_aceita(&ambientes, 0, &[1, 2], 0.05).expect("familia");
        assert!(familia.is_empty(), "nada e invariante: {familia:?}");
        assert!(area(&familia).is_empty());
    }

    #[test]
    fn a_unidade_de_grupo_encolhe_a_familia() {
        let fina = mundo(11, 240, 1.0, 1.0);
        let grossa: Vec<Ambiente> = fina
            .iter()
            .map(|a| a.media_por_grupo(20).expect("media por grupo"))
            .collect();
        let familia_fina = familia_aceita(&fina, 0, &[1, 2], 0.05).expect("familia fina");
        let familia_grossa = familia_aceita(&grossa, 0, &[1, 2], 0.05).expect("familia grossa");
        // O poder do teste anda com o n, e o n tem de ser contado na unidade independente: com 12
        // pontos por ambiente o limiar de KS e frouxo e a familia aceita cresce.
        assert_eq!(grossa[0].linhas(), 12);
        assert!(limiar_ks(12, 12, 0.05) > limiar_ks(240, 240, 0.05));
        assert!(
            familia_grossa.len() >= familia_fina.len(),
            "grao grosso aceita mais: {} contra {}",
            familia_grossa.len(),
            familia_fina.len()
        );
        assert_eq!(area(&familia_fina), vec![1]);
    }

    #[test]
    fn ks_conhece_o_proprio_limiar() {
        assert_eq!(ks(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0]), 0.0);
        let longe = ks(&[0.0, 0.1, 0.2], &[5.0, 5.1, 5.2]);
        assert!(longe > 0.9, "amostras separadas dao ks alto: {longe}");
        let mut rng = Rng::novo(3);
        let a: Vec<f64> = (0..400).map(|_| normal(&mut rng)).collect();
        let b: Vec<f64> = (0..400).map(|_| normal(&mut rng) + 1.2).collect();
        let estatistica = ks(&a, &b);
        assert!(estatistica > limiar_ks(400, 400, 0.05));
    }

    #[test]
    fn limiar_cai_com_o_tamanho_da_amostra() {
        assert!(limiar_ks(50, 50, 0.05) > limiar_ks(500, 500, 0.05));
        assert!(limiar_ks(500, 500, 0.05) > limiar_ks(5000, 5000, 0.05));
    }

    #[test]
    fn preditores_colineares_reprovam() {
        let ambientes = [Ambiente::novo(
            "A",
            vec![
                vec![1.0, 2.0, 3.0],
                vec![1.0, 2.0, 3.0],
                vec![1.0, 2.0, 3.0],
            ],
        )
        .expect("ambiente")];
        match regressao(
            &[1.0, 2.0, 3.0],
            &[vec![1.0, 2.0, 3.0], vec![2.0, 4.0, 6.0]],
        ) {
            Err(Erro::SistemaSingular) => {}
            outro => panic!("esperava sistema singular, veio {outro:?}"),
        }
        assert_eq!(ambientes.len(), 1);
    }

    #[test]
    fn alvo_como_preditor_reprova() {
        let ambientes = mundo(1, 20, 1.0, 1.0);
        match conjunto_aceito(&ambientes, 1, &[1], 0.05) {
            Err(Erro::AlvoEntrePreditores(j)) => assert_eq!(j, 1),
            outro => panic!("esperava alvo entre preditores, veio {outro:?}"),
        }
    }

    #[test]
    fn contagem_de_conjuntos_bate_com_a_enumeracao() {
        let ambientes = mundo(5, 30, 1.0, 1.0);
        let familia = familia_aceita(&ambientes, 0, &[1, 2], 0.05).expect("familia");
        assert_eq!(contar_conjuntos(2), 4);
        assert!(familia.len() <= contar_conjuntos(2) as usize);
        assert_eq!(contar_conjuntos(10), 1024);
    }
}
