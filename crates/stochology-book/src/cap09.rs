//! Capítulo 9 — a escada de causa e efeito: confundidor contra colisor e um ICP mínimo.
//!
//! Laboratório autocontido, sem dependências externas: SplitMix64 embutido, o teste de
//! invariância pela razão de verossimilhança de uma F e o sorteio com semente fixa.
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
    /// Uniforme em [0,1).
    pub fn uniforme(&mut self) -> f64 {
        (self.proximo() >> 11) as f64 / (1u64 << 53) as f64
    }
    /// Normal padrão (Marsaglia, polar).
    pub fn normal(&mut self) -> f64 {
        loop {
            let u = 2.0 * self.uniforme() - 1.0;
            let v = 2.0 * self.uniforme() - 1.0;
            let s = u * u + v * v;
            if s > 0.0 && s < 1.0 {
                return u * (-2.0 * s.ln() / s).sqrt();
            }
        }
    }
}

// ---------------------------------------------------------------- estatística

fn ln_gama(x: f64) -> f64 {
    // Lanczos, g = 7, n = 9.
    const C: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    if x < 0.5 {
        std::f64::consts::PI.ln() - (std::f64::consts::PI * x).sin().ln() - ln_gama(1.0 - x)
    } else {
        let x = x - 1.0;
        let mut a = C[0];
        let t = x + 7.5;
        for (i, c) in C.iter().enumerate().skip(1) {
            a += c / (x + i as f64);
        }
        0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
    }
}

/// Beta incompleta regularizada I_x(a,b), pela fração continuada de Lentz.
pub fn beta_incompleta(x: f64, a: f64, b: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let frente = (ln_gama(a + b) - ln_gama(a) - ln_gama(b) + a * x.ln() + b * (1.0 - x).ln()).exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        frente * fracao(x, a, b) / a
    } else {
        1.0 - frente * fracao(1.0 - x, b, a) / b
    }
}

fn fracao(x: f64, a: f64, b: f64) -> f64 {
    let mut c = 1.0;
    let mut d = 1.0 - (a + b) * x / (a + 1.0);
    if d.abs() < 1e-30 {
        d = 1e-30;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..300 {
        let m = m as f64;
        let m2 = 2.0 * m;
        let mut num = m * (b - m) * x / ((a + m2 - 1.0) * (a + m2));
        d = 1.0 + num * d;
        if d.abs() < 1e-30 {
            d = 1e-30;
        }
        c = 1.0 + num / c;
        if c.abs() < 1e-30 {
            c = 1e-30;
        }
        d = 1.0 / d;
        h *= d * c;
        num = -(a + m) * (a + b + m) * x / ((a + m2) * (a + m2 + 1.0));
        d = 1.0 + num * d;
        if d.abs() < 1e-30 {
            d = 1e-30;
        }
        c = 1.0 + num / c;
        if c.abs() < 1e-30 {
            c = 1e-30;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < 1e-14 {
            break;
        }
    }
    h
}

/// P(F > f) para uma F com (d1, d2) graus de liberdade.
pub fn p_f_maior(f: f64, d1: f64, d2: f64) -> f64 {
    if f <= 0.0 {
        return 1.0;
    }
    let x = d2 / (d2 + d1 * f);
    beta_incompleta(x, d2 / 2.0, d1 / 2.0)
}
// ---------------------------------------------------------------- o mundo

/// Um ambiente (um sítio, um ano, uma escola): quantas linhas e como ele desloca a causa X.
pub struct Ambiente {
    pub nome: &'static str,
    pub n: usize,
    pub deslocamento: f64,
    /// Desvio-padrão do ruído do sintoma Z neste ambiente. É o que muda o
    /// significado de Z de um sítio para o outro.
    pub ruido_z: f64,
}

pub const TRES_SITIOS: [Ambiente; 3] = [
    Ambiente {
        nome: "sitio_A",
        n: 4000,
        deslocamento: -1.0,
        ruido_z: 0.5,
    },
    Ambiente {
        nome: "sitio_B",
        n: 4000,
        deslocamento: 0.0,
        ruido_z: 2.0,
    },
    Ambiente {
        nome: "sitio_C",
        n: 4000,
        deslocamento: 1.0,
        ruido_z: 3.0,
    },
];

/// Uma linha: a causa X, o sintoma Z (filho de X), o ruído W, o desfecho Y e o ambiente.
#[derive(Clone, Copy)]
pub struct Linha {
    pub ambiente: usize,
    pub x: f64,
    pub z: f64,
    pub w: f64,
    pub y: f64,
}

/// Gera o mundo. `instabilidade = 0` é o caso honesto:
/// Y = 2X + ruído (mecanismo igual em todo ambiente), X -> Z, W puro ruído.
/// Com `instabilidade = 1`, o ambiente passa a agir direto em Y e o mecanismo muda.
pub fn gerar(semente: u64, ambientes: &[Ambiente], instabilidade: f64) -> Vec<Linha> {
    let mut rng = SplitMix64::new(semente);
    let mut linhas = Vec::new();
    for (e, amb) in ambientes.iter().enumerate() {
        for _ in 0..amb.n {
            let x = amb.deslocamento + rng.normal();
            let z = 2.0 * x + amb.ruido_z * rng.normal();
            let w = rng.normal();
            // instabilidade = 0: o mecanismo X -> Y é o mesmo em todo sítio.
            // instabilidade > 0: o sítio também entra por X e o coeficiente cambia.
            let y = 2.0 * x + rng.normal() + instabilidade * 2.0 * x;
            linhas.push(Linha {
                ambiente: e,
                x,
                z,
                w,
                y,
            });
        }
    }
    linhas
}

// ---------------------------------------------------------------- o teste

/// Resultado do teste de invariância para um conjunto candidato.
pub struct Teste {
    pub nome: String,
    pub f: f64,
    pub p: f64,
    /// Inclinação comum ajustada (para conjuntos de uma variável).
    pub inclinacao_comum: f64,
    /// Inclinações por ambiente (para conjuntos de uma variável).
    pub inclinacoes_por_ambiente: Vec<f64>,
}

pub fn coluna(linhas: &[Linha], e: usize, nome: &str) -> Vec<f64> {
    linhas
        .iter()
        .filter(|l| l.ambiente == e)
        .map(|l| valor(l, nome))
        .collect()
}

fn medias(xs: &[f64], ys: &[f64]) -> (f64, f64, f64, f64) {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let mut sxx = 0.0;
    let mut sxy = 0.0;
    for i in 0..xs.len() {
        sxx += (xs[i] - mx) * (xs[i] - mx);
        sxy += (xs[i] - mx) * (ys[i] - my);
    }
    (mx, my, sxx, sxy)
}

/// Ajuste com inclinação comum e intercepto livre por ambiente: devolve (rss, inclinação comum).
pub fn ajuste_restrito(linhas: &[Linha], ambientes: &[Ambiente], nome: &str) -> (f64, f64) {
    let (mut sxx, mut sxy) = (0.0, 0.0);
    for (e, _) in ambientes.iter().enumerate() {
        let xs = coluna(linhas, e, nome);
        let ys = coluna(linhas, e, "y");
        let (_, _, a, b) = medias(&xs, &ys);
        sxx += a;
        sxy += b;
    }
    let beta = if sxx > 0.0 { sxy / sxx } else { 0.0 };
    let mut rss = 0.0;
    for (e, amb) in ambientes.iter().enumerate() {
        let xs = coluna(linhas, e, nome);
        let ys = coluna(linhas, e, "y");
        let (mx, my, _, _) = medias(&xs, &ys);
        let a0 = my - beta * mx;
        for i in 0..amb.n {
            let r = ys[i] - a0 - beta * xs[i];
            rss += r * r;
        }
    }
    (rss, beta)
}

/// Ajuste com inclinação e intercepto livres por ambiente: devolve (rss, inclinações).
pub fn ajuste_livre(linhas: &[Linha], ambientes: &[Ambiente], nome: &str) -> (f64, Vec<f64>) {
    let mut rss = 0.0;
    let mut incl = Vec::new();
    for (e, amb) in ambientes.iter().enumerate() {
        let xs = coluna(linhas, e, nome);
        let ys = coluna(linhas, e, "y");
        let (mx, my, sxx, sxy) = medias(&xs, &ys);
        let beta = if sxx > 0.0 { sxy / sxx } else { 0.0 };
        let a0 = my - beta * mx;
        for i in 0..amb.n {
            let r = ys[i] - a0 - beta * xs[i];
            rss += r * r;
        }
        incl.push(beta);
    }
    (rss, incl)
}

fn resolver(a: &mut [Vec<f64>], b: &mut [f64]) -> Vec<f64> {
    let n = b.len();
    for i in 0..n {
        let mut pivo = i;
        for k in i + 1..n {
            if a[k][i].abs() > a[pivo][i].abs() {
                pivo = k;
            }
        }
        a.swap(i, pivo);
        b.swap(i, pivo);
        for k in i + 1..n {
            let fator = a[k][i] / a[i][i];
            for l in i..n {
                a[k][l] -= fator * a[i][l];
            }
            b[k] -= fator * b[i];
        }
    }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut s = b[i];
        for l in i + 1..n {
            s -= a[i][l] * x[l];
        }
        x[i] = s / a[i][i];
    }
    x
}

/// O teste de invariância (ICP linear) para um conjunto candidato de variáveis.
/// Hipótese nula: P(Y | X_S, E = e) é a mesma em todo ambiente.
pub fn icp(linhas: &[Linha], ambientes: &[Ambiente], nomes: &[&str]) -> Teste {
    let n_total: usize = ambientes.iter().map(|a| a.n).sum();
    let e = ambientes.len();
    let p = nomes.len();
    let k_livre = e * (1 + p);
    let k_restrito = e + p;

    if p == 0 {
        // o conjunto vazio só tem interceptos: nunca rejeita por inclinação.
        let incl = vec![0.0; ambientes.len()];
        return Teste {
            nome: "(vazio)".into(),
            f: 0.0,
            p: 1.0,
            inclinacao_comum: 0.0,
            inclinacoes_por_ambiente: incl,
        };
    }

    // Restrito: intercepto livre por ambiente + inclinação comum a todos.
    // Equivale a descontar a média de cada ambiente antes de ajustar.
    let mut a = vec![vec![0.0; p]; p];
    let mut b = vec![0.0; p];
    let mut cols_por_ambiente: Vec<Vec<Vec<f64>>> = Vec::new();
    for (i, amb) in ambientes.iter().enumerate() {
        let cols: Vec<Vec<f64>> = nomes.iter().map(|nm| coluna(linhas, i, nm)).collect();
        let ys = coluna(linhas, i, "y");
        let (mx, my, _, _) = medias(&cols[0], &ys);
        let mxs: Vec<f64> = cols
            .iter()
            .map(|c| c.iter().sum::<f64>() / amb.n as f64)
            .collect();
        let _ = mx;
        for k in 0..p {
            for l in 0..p {
                let mut s = 0.0;
                for r in 0..amb.n {
                    s += (cols[k][r] - mxs[k]) * (cols[l][r] - mxs[l]);
                }
                a[k][l] += s;
            }
            let mut s = 0.0;
            for r in 0..amb.n {
                s += (cols[k][r] - mxs[k]) * (ys[r] - my);
            }
            b[k] += s;
        }
        cols_por_ambiente.push(cols);
    }
    let beta = resolver(&mut a, &mut b);
    let mut rss_restrito = 0.0;
    for (i, amb) in ambientes.iter().enumerate() {
        let cols = &cols_por_ambiente[i];
        let ys = coluna(linhas, i, "y");
        let mxs: Vec<f64> = cols
            .iter()
            .map(|c| c.iter().sum::<f64>() / amb.n as f64)
            .collect();
        let my = ys.iter().sum::<f64>() / amb.n as f64;
        let mut a0 = my;
        for k in 0..p {
            a0 -= beta[k] * mxs[k];
        }
        for r in 0..amb.n {
            let mut prev = a0;
            for k in 0..p {
                prev += beta[k] * cols[k][r];
            }
            rss_restrito += (ys[r] - prev) * (ys[r] - prev);
        }
    }

    // Livre: intercepto e inclinação por ambiente (sistema de (p+1) incógnitas).
    let mut rss_livre = 0.0;
    let mut incl_livre = Vec::new();
    for (i, amb) in ambientes.iter().enumerate() {
        let cols = &cols_por_ambiente[i];
        let ys = coluna(linhas, i, "y");
        let mut m = vec![vec![0.0; p + 1]; p + 1];
        let mut v = vec![0.0; p + 1];
        m[0][0] = amb.n as f64;
        v[0] = ys.iter().sum::<f64>();
        for k in 0..p {
            let sk: f64 = cols[k].iter().sum();
            m[0][k + 1] = sk;
            m[k + 1][0] = sk;
            v[k + 1] = cols[k]
                .iter()
                .zip(ys.iter())
                .map(|(x, y)| x * y)
                .sum::<f64>();
            for l in 0..p {
                m[k + 1][l + 1] = cols[k]
                    .iter()
                    .zip(cols[l].iter())
                    .map(|(x, z)| x * z)
                    .sum::<f64>();
            }
        }
        let sol = resolver(&mut m, &mut v);
        for r in 0..amb.n {
            let mut prev = sol[0];
            for k in 0..p {
                prev += sol[k + 1] * cols[k][r];
            }
            rss_livre += (ys[r] - prev) * (ys[r] - prev);
        }
        incl_livre.push(sol[1]);
    }

    let gl_num = (k_livre - k_restrito) as f64;
    let gl_den = (n_total - k_livre) as f64;
    let f = ((rss_restrito - rss_livre) / gl_num) / (rss_livre / gl_den);
    let pvalor = p_f_maior(f, gl_num, gl_den);
    Teste {
        nome: nomes.join("+"),
        f,
        p: pvalor,
        inclinacao_comum: beta[0],
        inclinacoes_por_ambiente: incl_livre,
    }
}

/// A interseção dos conjuntos aceitos pelo teste.
pub fn intersecao(aceitos: &[Vec<&str>]) -> Vec<String> {
    if aceitos.is_empty() {
        return Vec::new();
    }
    let mut atual: Vec<String> = aceitos[0].iter().map(|s| s.to_string()).collect();
    for conj in aceitos.iter().skip(1) {
        atual.retain(|v| conj.contains(&v.as_str()));
    }
    atual
}

pub fn valor(l: &Linha, nome: &str) -> f64 {
    match nome {
        "x" => l.x,
        "z" => l.z,
        "w" => l.w,
        "y" => l.y,
        _ => f64::NAN,
    }
}

/// Correlação de Pearson entre duas colunas, em todas as linhas.
pub fn correlacao(linhas: &[Linha], a: &str, b: &str) -> f64 {
    let xs: Vec<f64> = linhas.iter().map(|l| valor(l, a)).collect();
    let ys: Vec<f64> = linhas.iter().map(|l| valor(l, b)).collect();
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let (mut sxy, mut sxx, mut syy) = (0.0, 0.0, 0.0);
    for i in 0..xs.len() {
        sxy += (xs[i] - mx) * (ys[i] - my);
        sxx += (xs[i] - mx).powi(2);
        syy += (ys[i] - my).powi(2);
    }
    sxy / (sxx * syy).sqrt()
}

/// Inclinação da regressão simples de Y sobre `nome`, em todas as linhas juntas (a conta ingênua).
pub fn inclinacao_ingenua(linhas: &[Linha], nome: &str) -> f64 {
    let xs: Vec<f64> = linhas.iter().map(|l| valor(l, nome)).collect();
    let ys: Vec<f64> = linhas.iter().map(|l| l.y).collect();
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let (mut sxy, mut sxx) = (0.0, 0.0);
    for i in 0..xs.len() {
        sxy += (xs[i] - mx) * (ys[i] - my);
        sxx += (xs[i] - mx).powi(2);
    }
    sxy / sxx
}

/// Os três degraus de Pearl, classificados pela forma da pergunta.
pub fn degrau_de_pearl(pergunta: &str) -> &'static str {
    if pergunta.contains("do(") {
        return "2. intervenção";
    }
    if pergunta.starts_with("se ") && pergunta.contains('?') {
        return "2. intervenção";
    }
    if pergunta.starts_with("eu teria") || pergunta.contains("teria acontecido") {
        return "3. contrafactual";
    }
    "1. associação"
}

const PERGUNTAS: [&str; 3] = [
    "bairros com mais policia tem mais crimes",
    "se dobrarmos o policiamento, o crime cai?",
    "esse assalto teria acontecido com uma viatura na esquina?",
];

fn pedra_nos_rins() {
    println!("== Capítulo 9 :: o paradoxo de Simpson nas cirurgias de pedra nos rins ==");
    // Charig e colegas (1986): sucessos em 350 procedimentos abertos e 350 na tecnica nova.
    let (peq_ab, peq_nova) = (81.0, 234.0);
    let (grd_ab, grd_nova) = (192.0, 55.0);
    let (n_peq_ab, n_peq_nova) = (87.0, 270.0);
    let (n_grd_ab, n_grd_nova) = (263.0, 80.0);
    let ab = peq_ab + grd_ab;
    let nova = peq_nova + grd_nova;
    let nab = n_peq_ab + n_grd_ab;
    let nnova = n_peq_nova + n_grd_nova;
    let peq = n_peq_ab + n_peq_nova;
    let grd = n_grd_ab + n_grd_nova;
    println!(
        "pequenas: aberta {}/{} = {:.4}  nova {}/{} = {:.4}",
        peq_ab as u32,
        n_peq_ab as u32,
        peq_ab / n_peq_ab,
        peq_nova as u32,
        n_peq_nova as u32,
        peq_nova / n_peq_nova
    );
    println!(
        "grandes:  aberta {}/{} = {:.4}  nova {}/{} = {:.4}",
        grd_ab as u32,
        n_grd_ab as u32,
        grd_ab / n_grd_ab,
        grd_nova as u32,
        n_grd_nova as u32,
        grd_nova / n_grd_nova
    );
    println!(
        "cru:      aberta {}/{} = {:.4}  nova {}/{} = {:.4}",
        ab as u32,
        nab as u32,
        ab / nab,
        nova as u32,
        nnova as u32,
        nova / nnova
    );
    let w_peq = peq / (peq + grd);
    let w_grd = grd / (peq + grd);
    let ponderada_ab = w_peq * (peq_ab / n_peq_ab) + w_grd * (grd_ab / n_grd_ab);
    let ponderada_nova = w_peq * (peq_nova / n_peq_nova) + w_grd * (grd_nova / n_grd_nova);
    println!(
        "peso dos estratos na populacao inteira: pequenas {:.4}, grandes {:.4}",
        w_peq, w_grd
    );
    println!(
        "ponderado pela populacao inteira: aberta {:.4}  nova {:.4}  -> a vantagem inverte de novo",
        ponderada_ab, ponderada_nova
    );
    println!(
        "fracao de casos dificeis na aberta = {:.4}, na nova = {:.4}",
        n_grd_ab / nab,
        n_grd_nova / nnova
    );
    println!();
}

fn escalada_de_pearl() {
    println!("== Capítulo 9 :: os tres degraus de Pearl ==");
    for p in PERGUNTAS {
        println!("  [{:>16}] {}", degrau_de_pearl(p), p);
    }
    println!();
}

fn uma_rodada() {
    println!("== Capítulo 9 :: um ICP minimo, tres sitios, tres candidatos ==");
    let linhas = gerar(2026, &TRES_SITIOS, 0.0);
    println!(
        "linhas = {} (4000 por sitio), X -> Y com coeficiente 2, X -> Z (filho) com ruido diferente por sitio, W puro ruido",
        linhas.len()
    );
    println!(
        "correlacao ingenua com Y: corr(X,Y) = {:.4}  corr(Z,Y) = {:.4}  corr(W,Y) = {:.4}",
        correlacao(&linhas, "x", "y"),
        correlacao(&linhas, "z", "y"),
        correlacao(&linhas, "w", "y")
    );
    println!(
        "inclinacao ingenua (todos os sitios juntos): X = {:.4}  Z = {:.4}  W = {:.4}",
        inclinacao_ingenua(&linhas, "x"),
        inclinacao_ingenua(&linhas, "z"),
        inclinacao_ingenua(&linhas, "w")
    );
    let conjuntos: [&[&str]; 7] = [
        &["x"],
        &["z"],
        &["w"],
        &["x", "z"],
        &["x", "w"],
        &["z", "w"],
        &["x", "z", "w"],
    ];
    let mut aceitos: Vec<Vec<&str>> = Vec::new();
    for conj in conjuntos {
        let t = icp(&linhas, &TRES_SITIOS, conj);
        let incl: Vec<String> = t
            .inclinacoes_por_ambiente
            .iter()
            .map(|b| format!("{:.3}", b))
            .collect();
        println!(
            "  candidato {{{:>5}}}  F = {:>7.3}  p = {:.3e}  comum = {:.3}  por sitio = [{}]  {}",
            t.nome,
            t.f,
            t.p,
            t.inclinacao_comum,
            incl.join(", "),
            if t.p > 0.05 { "ACEITO" } else { "rejeitado" }
        );
        if t.p > 0.05 {
            aceitos.push(conj.to_vec());
        }
    }
    let t = icp(&linhas, &TRES_SITIOS, &[]);
    println!(
        "  candidato {{vazio}}  F = {:.3}  p = {:.1}  (so interceptos)",
        t.f, t.p
    );
    println!(
        "conjuntos aceitos: {} -> intersecao S_hat = {{{}}}",
        aceitos.len(),
        intersecao(&aceitos).join(", ")
    );
    println!();
}

fn muitas_rodadas(repeticoes: usize, instabilidade: f64, rotulo: &str) {
    let conjuntos: [&[&str]; 7] = [
        &["x"],
        &["z"],
        &["w"],
        &["x", "z"],
        &["x", "w"],
        &["z", "w"],
        &["x", "z", "w"],
    ];
    let mut rejeicoes = [0usize; 7];
    let mut soma_f = [0.0f64; 7];
    let mut aceito_x = 0usize;
    let mut inter_x = 0usize;
    let mut inter_z = 0usize;
    let mut inter_vazia = 0usize;
    let mut todos_aceitos = 0usize;
    let mut inter_vazia_por_poder = 0usize;
    let mut inter_w = 0usize;
    let mut inter_com_xsem_w = 0usize;
    let mut ing_x = 0.0;
    let mut ing_z = 0.0;
    for r in 0..repeticoes {
        let linhas = gerar(2026 + r as u64, &TRES_SITIOS, instabilidade);
        let mut aceitos: Vec<Vec<&str>> = Vec::new();
        for (i, conj) in conjuntos.iter().enumerate() {
            let t = icp(&linhas, &TRES_SITIOS, conj);
            soma_f[i] += t.f;
            if t.p <= 0.05 {
                rejeicoes[i] += 1;
            } else {
                aceitos.push(conj.to_vec());
            }
        }
        if icp(&linhas, &TRES_SITIOS, &["x"]).p > 0.05 {
            aceito_x += 1;
        }
        let inter = intersecao(&aceitos);
        if inter == vec!["x".to_string()] {
            inter_x += 1;
        }
        if inter == vec!["z".to_string()] {
            inter_z += 1;
        }
        if inter == vec!["w".to_string()] {
            inter_w += 1;
        }
        if inter.contains(&"x".to_string()) && !inter.contains(&"w".to_string()) {
            inter_com_xsem_w += 1;
        }
        if inter.is_empty() {
            inter_vazia += 1;
            if aceitos.len() == conjuntos.len() {
                todos_aceitos += 1;
            } else if !aceitos.iter().any(|c| c.contains(&"x")) {
                inter_vazia_por_poder += 1;
            }
        }
        ing_x += inclinacao_ingenua(&linhas, "x");
        ing_z += inclinacao_ingenua(&linhas, "z");
    }
    let n = repeticoes as f64;
    println!("== Capítulo 9 :: {} repetições, {}", repeticoes, rotulo);
    println!(
        "inclinação ingênua média (todos os sítios juntos): X = {:.4}  Z = {:.4}",
        ing_x / n,
        ing_z / n
    );
    for (i, conj) in conjuntos.iter().enumerate() {
        println!(
            "  {{{:>5}}}  F médio = {:>8.2}  taxa de rejeição = {:.3}",
            conj.join("+"),
            soma_f[i] / n,
            rejeicoes[i] as f64 / n
        );
    }
    println!("  {{x}} aceito em {:.3} das rodadas", aceito_x as f64 / n);
    println!(
        "  interseção = {{x}} em {:.3} das rodadas",
        inter_x as f64 / n
    );
    println!(
        "  interseção = {{z}} em {:.3} das rodadas",
        inter_z as f64 / n
    );
    println!(
        "  interseção = {{w}} em {:.3} das rodadas",
        inter_w as f64 / n
    );
    println!(
        "  interseção contém x e não contém w em {:.3} das rodadas",
        inter_com_xsem_w as f64 / n
    );
    let vazia_por_w =
        (inter_vazia as f64 / n) - (todos_aceitos as f64 / n) - (inter_vazia_por_poder as f64 / n);
    println!(
        "  interseção vazia em {:.3} (dos quais {:.3} porque só o ruído W sobrou, {:.3} porque a própria causa não sobrou, e {:.3} porque W entrou em alguns conjuntos e não em outros)",
        inter_vazia as f64 / n,
        inter_w as f64 / n,
        inter_vazia_por_poder as f64 / n,
        vazia_por_w
    );
    println!();
}

pub fn rodar() {
    pedra_nos_rins();
    escalada_de_pearl();
    uma_rodada();
    muitas_rodadas(200, 0.0, "mecanismo estavel: Y = 2X + ruido em todo sitio");
    muitas_rodadas(
        200,
        1.0,
        "mecanismo instavel: o coeficiente de X em Y muda de sitio para sitio",
    );
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn degraus_de_pearl_e_intersecao_icp_funcionam() {
        assert!(degrau_de_pearl("P(Y | X)").contains("associação"));
        assert!(degrau_de_pearl("P(Y | do(X))").contains("intervenção"));
        assert!(degrau_de_pearl("esse assalto teria acontecido?").contains("contrafactual"));
        assert_eq!(
            intersecao(&[vec!["x", "z"], vec!["x"]]),
            vec!["x".to_string()]
        );
    }
}
