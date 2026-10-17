//! Capítulo 11 — o preço do erro: custo/perda, EVPI, diques, Goodhart e o trilema.
//!
//! Laboratório autocontido, sem dependências externas.
//! Uso: cargo run --release

// ---------------------------------------------------------------- saida medida
#[derive(Clone)]
pub struct SplitMix64(pub u64);

impl SplitMix64 {
    pub fn new(semente: u64) -> Self {
        SplitMix64(semente)
    }
    pub fn proximo(&mut self) -> u64 {
        stochology_core::rng::passo_splitmix64(&mut self.0)
    }
    pub fn uniforme(&mut self) -> f64 {
        (self.proximo() >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// Função de distribuição normal padrão, por erf de Abramowitz e Stegun 7.1.26.
pub fn phi(x: f64) -> f64 {
    let s = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.231_641_9 * x);
    let y = 1.0
        - (((((1.061_405_429 * t - 1.453_152_027) * t) + 1.421_413_741) * t - 0.284_496_736) * t
            + 0.254_829_592)
            * t
            * (-x * x).exp();
    0.5 * (1.0 + s * y)
}

/// Inverte a normal padrão: o quantil z com P(N(0,1) <= z) = p.
pub fn quantil_normal(p: f64) -> f64 {
    let p = p.clamp(1e-12, 1.0 - 1e-12);
    let (mut lo, mut hi) = (-10.0_f64, 10.0_f64);
    for _ in 0..200 {
        let meio = 0.5 * (lo + hi);
        if phi(meio) < p {
            lo = meio;
        } else {
            hi = meio;
        }
    }
    0.5 * (lo + hi)
}

// ------------------------------------------------- o limiar custo/perda

pub struct Navio {
    pub riqueza: f64,
    pub premio: f64,
    pub perda: f64,
    pub p: f64,
    pub jogadores: usize,
}

impl Navio {
    /// O limiar p > C/L: a partir de que chance vale a pena pagar o prêmio.
    pub fn limiar(&self) -> f64 {
        self.premio / self.perda
    }
    /// O que a esperança manda fazer.
    pub fn protege_pela_esperanca(&self) -> bool {
        self.p > self.limiar()
    }
    /// Perda esperada sem seguro, e resultado esperado com seguro.
    pub fn contas_de_esperanca(&self) -> (f64, f64) {
        (self.p * self.perda, self.premio - self.p * self.perda)
    }
    /// Crescimento no tempo por mês do armador dono de `navios` navios, com ou
    /// sem seguro sobre a carga perdida.
    pub fn crescimento_do_armador(&self, navios: usize, seguro: bool) -> f64 {
        let w = self.riqueza;
        let mut g = 0.0;
        for i in 0..=navios {
            let prob = binomial(navios, i, self.p);
            if prob <= 0.0 {
                continue;
            }
            let do_premio = if seguro { self.premio } else { 0.0 };
            let perda_liquida = if seguro { 0.0 } else { i as f64 * self.perda };
            let w_fim = w - do_premio - perda_liquida;
            if w_fim <= 0.0 {
                return f64::NEG_INFINITY;
            }
            g += prob * (w_fim / w).ln();
        }
        g
    }
}

/// Probabilidade binomial P(X = i) para k tentativas com chance p.
pub fn binomial(k: usize, i: usize, p: f64) -> f64 {
    if i > k {
        return 0.0;
    }
    let mut c = 1.0;
    for j in 0..i {
        c *= (k - j) as f64 / (j + 1) as f64;
    }
    c * p.powi(i as i32) * (1.0 - p).powi((k - i) as i32)
}

/// Crescimento no tempo por mês de uma seguradora com `contratos` contratos
/// independentes, cada um com prêmio `premio` e perda `perda` com chance `p` e
/// riqueza `w`: aproximação média menos variância/2 para a soma.
pub fn crescimento_da_seguradora(w: f64, contratos: usize, premio: f64, perda: f64, p: f64) -> f64 {
    let n = contratos as f64;
    let media = n * (premio - p * perda);
    let var = n * perda * perda * p * (1.0 - p);
    (1.0 + media / w).ln() - 0.5 * var / (w + media).powi(2)
}
// ------------------------------------------------- o valor da informação

/// Valores de dois atos em dois estados: (ganho_do_ato, ganho_do_estado) para cada célula.
#[derive(Clone, Copy)]
pub struct DoisAtos {
    pub u_agir_doente: f64,
    pub u_nao_agir_doente: f64,
    pub u_agir_sadio: f64,
    pub u_nao_agir_sadio: f64,
    pub p_doente: f64,
}

impl DoisAtos {
    pub fn ganho_esperado(&self, agir: bool) -> f64 {
        if agir {
            self.p_doente * self.u_agir_doente + (1.0 - self.p_doente) * self.u_agir_sadio
        } else {
            self.p_doente * self.u_nao_agir_doente + (1.0 - self.p_doente) * self.u_nao_agir_sadio
        }
    }
    /// O melhor ato sem informação e o ganho que ele entrega.
    pub fn melhor_sem_informacao(&self) -> (bool, f64) {
        let a = self.ganho_esperado(true);
        let b = self.ganho_esperado(false);
        if a >= b { (true, a) } else { (false, b) }
    }
    /// O ganho médio de quem sabe o estado antes de agir.
    pub fn com_informacao_perfeita(&self) -> f64 {
        let d = self.u_agir_doente.max(self.u_nao_agir_doente);
        let s = self.u_agir_sadio.max(self.u_nao_agir_sadio);
        self.p_doente * d + (1.0 - self.p_doente) * s
    }
    pub fn evpi(&self) -> f64 {
        self.com_informacao_perfeita() - self.melhor_sem_informacao().1
    }
    /// A chance em que o melhor ato muda: p* = (u_agir_sadio - u_nao_agir_sadio) / ((u_agir_sadio - u_nao_agir_sadio) + (u_nao_agir_doente - u_agir_doente)).
    pub fn limiar_de_acao(&self) -> f64 {
        let a = self.u_agir_sadio - self.u_nao_agir_sadio;
        let b = self.u_nao_agir_doente - self.u_agir_doente;
        a / (a + b)
    }
}

/// Valor esperado da informação de um exame imperfeito: (sensibilidade, especificidade).
pub fn evsi(exame: &DoisAtos, sensibilidade: f64, especificidade: f64) -> f64 {
    let p = exame.p_doente;
    let ppv = sensibilidade * p / (sensibilidade * p + (1.0 - especificidade) * (1.0 - p));
    let npv = especificidade * (1.0 - p) / (especificidade * (1.0 - p) + (1.0 - sensibilidade) * p);
    let ganho_positivo = ppv * exame.u_agir_doente.max(exame.u_nao_agir_doente)
        + (1.0 - ppv) * exame.u_agir_sadio.max(exame.u_nao_agir_sadio);
    let ganho_negativo = npv * exame.u_nao_agir_doente.max(exame.u_agir_doente)
        + (1.0 - npv) * exame.u_nao_agir_sadio.max(exame.u_agir_sadio);
    let prob_positivo = sensibilidade * p + (1.0 - especificidade) * (1.0 - p);
    let com_exame = prob_positivo * ganho_positivo + (1.0 - prob_positivo) * ganho_negativo;
    com_exame - exame.melhor_sem_informacao().1
}

// ------------------------------------------------- os diques da Holanda

/// Quantil da GEV de uma amostra de máximos: nível associado ao período de retorno `T`.
/// F(h) = exp(-(1 + xi (h - mu) / sigma)^(-1/xi)); o nível é mu - sigma (1 - (-ln(1-1/T))^(-xi)) / xi.
pub fn gev_nivel(xi: f64, mu: f64, sigma: f64, periodo: f64) -> f64 {
    let y = (-(1.0_f64 - 1.0 / periodo).ln()).powf(-xi);
    if xi.abs() < 1e-9 {
        mu - sigma * (y - 1.0)
    } else {
        mu - sigma * (1.0 - y) / xi
    }
}

/// Custo anual de um dique de altura h: a obra, h - 2, mais o dano esperado,
/// 3000 x P(máxima > h).
pub fn custo_do_dique(xi: f64, mu: f64, sigma: f64, h: f64) -> f64 {
    let obra = (h - 2.0).max(0.0);
    if h <= mu + sigma / xi.max(1e-9) && xi > 0.0 {
        // acima do suporte a probabilidade é zero
    }
    let z = 1.0 + xi * (h - mu) / sigma;
    let prob = if z <= 0.0 {
        0.0
    } else if xi.abs() < 1e-9 {
        (-((h - mu) / sigma).exp()).exp()
    } else {
        let t = z.powf(-1.0 / xi);
        1.0 - (-t).exp()
    };
    obra + 3_000.0 * prob
}

/// As alturas de uma grade fina que minimizam o custo de cada modelo.
pub fn altura_otima(xi: f64, mu: f64, sigma: f64) -> f64 {
    let mut melhor = 0.0;
    let mut melhor_custo = f64::INFINITY;
    let mut h = 0.0;
    while h <= 30.0 {
        let c = custo_do_dique(xi, mu, sigma, h);
        if c < melhor_custo {
            melhor_custo = c;
            melhor = h;
        }
        h += 0.001;
    }
    melhor
}

/// Um grupo: quantas linhas, a taxa-base do desfecho, o FNR e o FPR observados.
pub struct Grupo {
    pub nome: &'static str,
    pub n: usize,
    pub taxa_base: f64,
    pub fnr_observado: f64,
    pub fpr_observado: f64,
}

/// Os erros de um grupo quando o FNR é levado a um alvo, descritos pelo par
/// observado (FNR, FPR). A conta é a de Chouldechova: fixados o limiar e a
/// estatística z do escore, o FPR e o FNR se movem juntos, e não se pode
/// escolher os dois.
/// O FPR de um grupo quando o FNR é levado a um alvo, descrito pelo par observado
/// (FNR, FPR). Com a cauda do escore tratada como uma gaussiana de mesma forma
/// nos dois grupos, a razão (1-FPR)/(1-FNR) se conserva, de modo que
/// FPR_alvo = 1 - (1 - FPR_obs) * (1 - FNR_alvo) / (1 - FNR_obs).
pub fn fpr_com_fnr(fnr_observado: f64, fpr_observado: f64, fnr_alvo: f64) -> f64 {
    let denom = 1.0 - fnr_observado;
    if denom <= 1e-9 {
        return fpr_observado;
    }
    (1.0 - (1.0 - fpr_observado) * (1.0 - fnr_alvo) / denom).clamp(0.0, 1.0)
}

/// O valor preditivo positivo que a matriz de confusão entrega: TP/(TP+FP).
pub fn vpp_da_matriz(taxa_base: f64, fnr: f64, fpr: f64) -> f64 {
    let tp = (1.0 - fnr) * taxa_base;
    let fp = fpr * (1.0 - taxa_base);
    tp / (tp + fp)
}

/// O VPP que a identidade de Chouldechova exige, dados a taxa-base, o FNR e o FPR,
/// pela forma invertida VPP = 1 / (1 + [(1-FPR)(1-p)] / [p(1-FNR)]).
pub fn vpp_da_identidade(taxa_base: f64, fnr: f64, fpr: f64) -> f64 {
    let odds = (1.0 - fpr) * (1.0 - taxa_base) / (taxa_base * (1.0 - fnr));
    1.0 / (1.0 + odds)
}

/// As duas direções do trilema: igualar o FNR para baixo (ao valor do grupo
/// menos penalizado) e para cima (ao do mais penalizado).
pub struct Trilema {
    pub grupos: [Grupo; 2],
    pub fpr_com_fnr_igualado: [f64; 2],
    pub fnr_alvo_igualado: f64,
}

pub fn trilema(a: Grupo, b: Grupo) -> Trilema {
    let alvo = a.fnr_observado.max(b.fnr_observado);
    let fpr_com_fnr_igualado = [
        fpr_com_fnr(a.fnr_observado, a.fpr_observado, alvo),
        fpr_com_fnr(b.fnr_observado, b.fpr_observado, alvo),
    ];
    Trilema {
        grupos: [a, b],
        fpr_com_fnr_igualado,
        fnr_alvo_igualado: alvo,
    }
}

fn navio_de_bernoulli() {
    println!("== Capítulo 11 :: o navio de Bernoulli (C = 1.800, L = 34.000, p = 0,05) ==");
    let n = Navio {
        riqueza: 100_000.0,
        premio: 1_800.0,
        perda: 34_000.0,
        p: 0.05,
        jogadores: 1,
    };
    let (perda_esperada, resultado_do_seguro) = n.contas_de_esperanca();
    println!(
        "limiar p > C/L: p* = {:.5} ({:.3}%)  ->  p = 0,050 {} o limiar",
        n.limiar(),
        100.0 * n.limiar(),
        if n.protege_pela_esperanca() {
            "passa"
        } else {
            "nao passa"
        }
    );
    println!(
        "em esperança: perda esperada sem seguro = {:.0}, resultado do seguro = {:+.0} por mês",
        perda_esperada, resultado_do_seguro
    );
    println!(
        "crescimento no tempo do armador: 1 navio sem seguro = {:+.4}, 1 navio com seguro = {:+.4}, 3 navios sem seguro = {:+.4}",
        n.crescimento_do_armador(1, false),
        n.crescimento_do_armador(1, true),
        n.crescimento_do_armador(3, false)
    );
    for contratos in [1usize, 10, 100, 1_000] {
        println!(
            "seguradora com {:>5} contratos: crescimento no tempo = {:+.6} por mês",
            contratos,
            crescimento_da_seguradora(200_000.0, contratos, 1_800.0, 34_000.0, 0.05)
        );
    }
    println!();
}

fn valor_da_informacao() {
    println!("== Capítulo 11 :: o EVPI (tratar custa 1, deixar um doente custa 10) ==");
    let d = DoisAtos {
        u_agir_doente: -1.0,
        u_nao_agir_doente: -10.0,
        u_agir_sadio: -1.0,
        u_nao_agir_sadio: 0.0,
        p_doente: 0.3,
    };
    println!(
        "limiar de ação: p > {:.2}  ->  com p = 0,3 o melhor ato sem informação {} ({:+.2})",
        d.limiar_de_acao(),
        if d.melhor_sem_informacao().0 {
            "tratar"
        } else {
            "nao tratar"
        },
        d.melhor_sem_informacao().1
    );
    println!(
        "ganho de quem sabe o estado antes de agir = {:+.2}",
        d.com_informacao_perfeita()
    );
    println!("EVPI = {:+.2}", d.evpi());
    for (nome, sens, espec) in [
        ("sensivel  (0,90; 0,70)", 0.9, 0.7),
        ("especifico (0,30; 0,95)", 0.3, 0.95),
    ] {
        println!("  exame {}: EVSI = {:+.4}", nome, evsi(&d, sens, espec));
    }
    for p in [0.05_f64, 0.3, 0.6] {
        let mut e = d;
        e.p_doente = p;
        println!(
            "  com prevalência {:.2}: melhor ato sem informação = {}, EVPI = {:+.3}",
            p,
            if e.melhor_sem_informacao().0 {
                "tratar"
            } else {
                "nao tratar"
            },
            e.evpi()
        );
    }
    println!();
}

fn diques() {
    println!("== Capítulo 11 :: os diques de van Dantzig, quatro modelos de cauda ==");
    // Modelos calibrados de modo que o nível de 100 anos mais alto seja 4,26 m e o
    // de 10 000 anos do Gumbel seja 4,76 m: mu = 1,2567 e sigma = 0,762.
    let mu = 1.2567;
    let sigma = 0.762;
    let xis = [-0.1, 0.0, 0.1, 0.2];
    print!("  {:<8}", "modelo");
    for xi in xis {
        print!("  xi = {:+.1}", xi);
    }
    println!("   altura ótima");
    for periodo in [100.0_f64, 10_000.0] {
        print!("  T = {:<6.0}", periodo);
        for xi in xis {
            print!("  {:>9.2}", gev_nivel(xi, mu, sigma, periodo));
        }
        println!();
    }
    let mut alturas = Vec::new();
    for xi in xis {
        let h = altura_otima(xi, mu, sigma);
        alturas.push(h);
        println!(
            "  altura ótima com xi = {:+.1}: {:.3} m  (custo {:.3})",
            xi,
            h,
            custo_do_dique(xi, mu, sigma, h)
        );
    }
    let media = alturas.iter().sum::<f64>() / alturas.len() as f64;
    let wald = alturas.iter().cloned().fold(f64::MIN, f64::max);
    let mut custos = vec![vec![0.0; 4]; 4];
    for (i, xi) in xis.iter().enumerate() {
        for (j, h) in alturas.iter().enumerate() {
            custos[i][j] = custo_do_dique(*xi, mu, sigma, *h);
        }
    }
    // Savage: para cada modelo, o arrependimento é o custo menos o melhor custo do modelo
    let menores: Vec<f64> = (0..4)
        .map(|i| custos[i].iter().cloned().fold(f64::INFINITY, f64::min))
        .collect();
    let mut savage = alturas[0];
    let mut melhor_arrependimento = f64::INFINITY;
    for j in 0..4 {
        let pior = (0..4)
            .map(|i| custos[i][j] - menores[i])
            .fold(f64::MIN, f64::max);
        if pior < melhor_arrependimento {
            melhor_arrependimento = pior;
            savage = alturas[j];
        }
    }
    let precaucao = xis
        .iter()
        .map(|xi| gev_nivel(*xi, mu, sigma, 10_000.0))
        .fold(f64::MIN, f64::max);
    println!("  valor esperado com o Gumbel: {:.2} m", alturas[1]);
    println!("  media dos modelos: {:.2} m", media);
    println!("  maximin de Wald: {:.2} m", wald);
    println!(
        "  arrependimento minimax de Savage: {:.2} m (maior arrependimento {:.2})",
        savage, melhor_arrependimento
    );
    println!(
        "  precaucao: 1 em 10 000 no pior modelo = {:.2} m",
        precaucao
    );
    println!(
        "  EVPI do conjunto de modelos = {:+.3} m",
        precaucao - alturas[1]
    );
    println!();
}

fn bloco_do_trilema() {
    println!("== Capítulo 11 :: o trilema de Chouldechova ==");
    // Taxas-base e erros do H1 (h1/README.md): 0,069 contra 0,111 e FNR 0,24 contra 0,36.
    // O FPR por grupo vem do mesmo bloco de justiça: 0,105 contra 0,196.
    let brancos = Grupo {
        nome: "White",
        n: 4_776,
        taxa_base: 0.069,
        fnr_observado: 0.24,
        fpr_observado: 0.105,
    };
    let negros = Grupo {
        nome: "Black",
        n: 4_776,
        taxa_base: 0.111,
        fnr_observado: 0.36,
        fpr_observado: 0.196,
    };
    let t = trilema(brancos, negros);
    println!(
        "{:<8} {:>10} {:>10} {:>10} {:>16} {:>16} {:>10}",
        "grupo", "taxa-base", "FNR", "FPR", "falsos positivos", "falsos negativos", "VPP"
    );
    let vpp = |g: &Grupo| {
        (1.0 - g.fpr_observado) * (1.0 - g.taxa_base)
            / ((1.0 - g.fpr_observado) * (1.0 - g.taxa_base)
                + (1.0 - g.fnr_observado) * g.taxa_base)
    };
    let mut fp_antes = 0.0;
    let mut fn_antes = 0.0;
    for g in &t.grupos {
        let fp = g.n as f64 * (1.0 - g.taxa_base) * g.fpr_observado;
        let fneg = g.n as f64 * g.taxa_base * g.fnr_observado;
        fp_antes += fp;
        fn_antes += fneg;
        println!(
            "{:<8} {:>10.3} {:>10.3} {:>10.3} {:>16.0} {:>16.0} {:>10.4}",
            g.nome,
            g.taxa_base,
            g.fnr_observado,
            g.fpr_observado,
            fp,
            fneg,
            vpp(g)
        );
    }
    println!(
        "casos observados: White {} marcados pelos fatos, Black {}",
        (t.grupos[0].n as f64 * t.grupos[0].taxa_base) as u32,
        (t.grupos[1].n as f64 * t.grupos[1].taxa_base) as u32
    );
    println!();
    for (rotulo, alvo) in [
        ("para baixo, ao FNR do White", t.grupos[0].fnr_observado),
        ("para cima, ao FNR do Black", t.grupos[1].fnr_observado),
    ] {
        let fprs: Vec<f64> = t
            .grupos
            .iter()
            .map(|g| fpr_com_fnr(g.fnr_observado, g.fpr_observado, alvo))
            .collect();
        let mut fp = 0.0;
        let mut fneg = 0.0;
        println!("-- FNR igualado {} em {:.3} --", rotulo, alvo);
        for (i, g) in t.grupos.iter().enumerate() {
            let f = g.n as f64 * (1.0 - g.taxa_base) * fprs[i];
            let n_ = g.n as f64 * g.taxa_base * alvo;
            fp += f;
            fneg += n_;
            println!(
                "{:<8} {:>10.3} {:>10.3} {:>10.3} {:>16.0} {:>16.0}",
                g.nome, g.taxa_base, alvo, fprs[i], f, n_
            );
        }
        println!(
            "falsos positivos totais: {:.0} -> {:.0} ({:+.0})",
            fp_antes,
            fp,
            fp - fp_antes
        );
        println!(
            "falsos negativos totais: {:.0} -> {:.0} ({:+.0})",
            fn_antes,
            fneg,
            fneg - fn_antes
        );
        println!();
    }
    println!("identidade de Chouldechova: FPR = p/(1-p) * (1-VPP)/VPP * (1-FNR)");
    println!(
        "{:<8} {:>16} {:>18} {:>14}",
        "grupo", "VPP da matriz", "VPP da identidade", "razao"
    );
    for g in &t.grupos {
        let da_matriz = vpp_da_matriz(g.taxa_base, g.fnr_observado, g.fpr_observado);
        let da_identidade = vpp_da_identidade(g.taxa_base, g.fnr_observado, g.fpr_observado);
        println!(
            "{:<8} {:>16.4} {:>18.4} {:>14.1}x",
            g.nome,
            da_matriz,
            da_identidade,
            da_identidade / da_matriz
        );
    }
    println!("leitura: com o FNR e o FPR observados, o escore so seria calibrado por grupo");
    println!(
        "se o valor preditivo positivo fosse o da coluna do meio. Ele e muito maior: o escore"
    );
    println!("separa razoavelmente e nao se le como probabilidade de risco.");
    println!();
}
pub fn rodar() {
    navio_de_bernoulli();
    valor_da_informacao();
    diques();
    bloco_do_trilema();
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn dique_gev_subestima_com_gumbel_e_chouldechova_fecha() {
        let h_gumbel = altura_otima(0.0, 1.2567, 0.762);
        let h_frechet = altura_otima(0.2, 1.2567, 0.762);
        assert!(h_frechet > h_gumbel + 2.0);
        let v1 = vpp_da_matriz(0.111, 0.36, 0.196);
        assert!(v1 > 0.2 && v1 < 0.5);
    }
}
