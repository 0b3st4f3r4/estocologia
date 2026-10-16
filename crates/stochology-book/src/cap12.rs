//! Laboratorio do Capitulo 12 — o mapa do conhecimento.
//!
//! Mini-hipergrafo tipado G = (V, E_tree, E_graph, H, Phi): quatro provas de
//! estrutura e tres lentes medidas (Escalonomia por Hill, Crisonomia por EVPI,
//! Morfonomia por invariancia entre regimes).
//!
//! Semente fixa. Sem dependencias externas. SplitMix64 embutido.

fn splitmix64(estado: &mut u64) -> u64 {
    *estado = estado.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *estado;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn uniforme(estado: &mut u64, limite: u64) -> u64 {
    splitmix64(estado) % limite
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Camada {
    L1,
    L2,
    L3,
}

#[derive(Debug, Clone, PartialEq)]
struct No {
    id: String,
    camada: Camada,
    definicao: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TipoAresta {
    Dedutiva,
    PonteMatematica,
    Causal,
    Patologia,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lente {
    Fluxonomia,
    Crononomia,
    Escalonomia,
    Morfonomia,
    Crisonomia,
}

impl Lente {
    const TODAS: [Lente; 5] = [
        Lente::Fluxonomia,
        Lente::Crononomia,
        Lente::Escalonomia,
        Lente::Morfonomia,
        Lente::Crisonomia,
    ];

    fn nome(self) -> &'static str {
        match self {
            Lente::Fluxonomia => "Fluxonomia",
            Lente::Crononomia => "Crononomia",
            Lente::Escalonomia => "Escalonomia",
            Lente::Morfonomia => "Morfonomia",
            Lente::Crisonomia => "Crisonomia",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Hiperaresta {
    id: String,
    lente: Lente,
    membros: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct Aresta {
    de: String,
    para: String,
    tipo: TipoAresta,
    rotulo: String,
}

#[derive(Debug, Clone, Default)]
struct Grafo {
    nos: Vec<No>,
    tree: Vec<Aresta>,
    graph: Vec<Aresta>,
    hiperarestas: Vec<Hiperaresta>,
}

fn no(id: &str, camada: Camada, definicao: &str) -> No {
    No {
        id: id.to_string(),
        camada,
        definicao: definicao.to_string(),
    }
}

fn aresta(de: &str, para: &str, tipo: TipoAresta, rotulo: &str) -> Aresta {
    Aresta {
        de: de.to_string(),
        para: para.to_string(),
        tipo,
        rotulo: rotulo.to_string(),
    }
}

impl Grafo {
    /// Estrutura: ids unicos e completos, pontas existentes, arvore sem ciclo,
    /// hiperarestas com dois membros ou mais.
    fn validar(&self) -> Vec<String> {
        let mut p: Vec<String> = Vec::new();
        let mut vistos: Vec<&str> = Vec::new();
        for n in &self.nos {
            if n.id.trim().is_empty() || n.definicao.trim().is_empty() {
                p.push(format!("no incompleto: {}", n.id));
            }
            if vistos.contains(&n.id.as_str()) {
                p.push(format!("no duplicado: {}", n.id));
            }
            vistos.push(n.id.as_str());
        }
        let existe = |id: &str| self.nos.iter().any(|n| n.id == id);
        for a in self.tree.iter().chain(self.graph.iter()) {
            if !existe(&a.de) || !existe(&a.para) {
                p.push(format!("aresta para no ausente: {} -> {}", a.de, a.para));
            }
        }
        for h in &self.hiperarestas {
            if h.membros.len() < 2 {
                p.push(format!("hiperaresta com menos de dois membros: {}", h.id));
            }
            for m in &h.membros {
                if !existe(m) {
                    p.push(format!("hiperaresta {} cita no ausente: {}", h.id, m));
                }
            }
        }
        if let Some(ciclo) = self.ciclo_em_tree() {
            p.push(format!("ciclo em E_tree: {:?}", ciclo));
        }
        p
    }

    fn ordem_topologica(&self) -> Result<Vec<String>, Vec<String>> {
        let mut ordem: Vec<String> = Vec::new();
        let mut visitando: Vec<String> = Vec::new();
        for n in &self.nos {
            if !self.dfs(&n.id, &mut ordem, &mut visitando) {
                return Err(visitando);
            }
        }
        Ok(ordem)
    }

    fn dfs(&self, id: &str, ordem: &mut Vec<String>, visitando: &mut Vec<String>) -> bool {
        if ordem.iter().any(|x| x == id) {
            return true;
        }
        if visitando.iter().any(|x| x == id) {
            return false;
        }
        visitando.push(id.to_string());
        for a in &self.tree {
            if a.de == id && !self.dfs(&a.para, ordem, visitando) {
                return false;
            }
        }
        visitando.retain(|x| x != id);
        ordem.push(id.to_string());
        true
    }

    fn ciclo_em_tree(&self) -> Option<Vec<String>> {
        match self.ordem_topologica() {
            Ok(_) => None,
            Err(ciclo) => Some(ciclo),
        }
    }

    fn nos_sem_lente(&self) -> Vec<String> {
        self.nos
            .iter()
            .filter(|n| !self.hiperarestas.iter().any(|h| h.membros.contains(&n.id)))
            .map(|n| n.id.clone())
            .collect()
    }

    /// Profundidade dedutiva: comprimento do caminho mais longo ate uma folha.
    fn profundidades(&self) -> Vec<(String, usize)> {
        match self.ordem_topologica() {
            Ok(ordem) => ordem
                .iter()
                .map(|id| {
                    let mut d = 1usize;
                    let mut atual = id.clone();
                    loop {
                        match self.tree.iter().find(|a| a.de == atual) {
                            Some(a) => {
                                d += 1;
                                atual = a.para.clone();
                            }
                            None => break,
                        }
                    }
                    (id.clone(), d)
                })
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    fn por_camada(&self, c: Camada) -> usize {
        self.nos.iter().filter(|n| n.camada == c).count()
    }
}

// ---------------------------------------------------------------- lentes

/// Escalonomia: estimador de Hill sobre a cauda superior.
fn indice_hill(valores: &[f64], k: usize) -> f64 {
    let mut v: Vec<f64> = valores.iter().copied().filter(|x| *x > 0.0).collect();
    v.sort_by(|a, b| b.partial_cmp(a).unwrap());
    if v.len() <= k || k == 0 {
        return f64::NAN;
    }
    let corte = v[k];
    let soma: f64 = v[..k].iter().map(|x| (x / corte).ln()).sum();
    (soma / k as f64).recip()
}

fn cauda_selvagem(alfa: f64) -> bool {
    alfa < 2.0
}

/// Crisonomia: valor esperado da informacao perfeita, em unidades do pagamento.
fn evpi(pagamentos: &[f64], probabilidades: &[f64], melhor_sem_info: f64) -> f64 {
    let com: f64 = pagamentos
        .iter()
        .zip(probabilidades.iter())
        .map(|(w, p)| w * p)
        .sum();
    com - melhor_sem_info
}

fn inclinacao(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let num: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let den: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    num / den
}

fn invariante(inclinacoes: &[f64], tolerancia: f64) -> bool {
    let maior = inclinacoes.iter().cloned().fold(f64::MIN, f64::max);
    let menor = inclinacoes.iter().cloned().fold(f64::MAX, f64::min);
    (maior - menor).abs() <= tolerancia
}

// ------------------------------------------------------------- a bancada

fn grafo_valido() -> Grafo {
    Grafo {
        nos: vec![
            no(
                "L1_demanda",
                Camada::L1,
                "contagens de chamadas a cada 5 minutos da central real de 2003",
            ),
            no(
                "L2_erlang_c",
                Camada::L2,
                "SL(c,lambda) = 1 - C(c,lambda*AHT)*exp(-(c-lambda*AHT)t/AHT), estacionaria, sem abandono",
            ),
            no(
                "L3_gestor",
                Camada::L3,
                "escala decidida pelo Erlang C sobre a taxa media do historico",
            ),
        ],
        tree: vec![
            aresta("L2_erlang_c", "L3_gestor", TipoAresta::Dedutiva, "ficha que a decisao usa"),
            aresta("L1_demanda", "L2_erlang_c", TipoAresta::Dedutiva, "entrada do modelo"),
        ],
        graph: vec![
            aresta(
                "L2_erlang_c",
                "L1_demanda",
                TipoAresta::Patologia,
                "Falacia da Media: SL(E[L]) no lugar de E[SL(L)]",
            ),
            aresta(
                "L3_gestor",
                "L2_erlang_c",
                TipoAresta::Patologia,
                "decisao pelo dia medio, sem a cauda dos dias",
            ),
            aresta("L1_demanda", "L2_erlang_c", TipoAresta::Causal, "do(escala) sobre a demanda real"),
            aresta(
                "L2_erlang_c",
                "L3_gestor",
                TipoAresta::PonteMatematica,
                "Jensen: E[f(X)] >= f(E[X])",
            ),
        ],
        hiperarestas: vec![
            Hiperaresta {
                id: "H1".to_string(),
                lente: Lente::Crononomia,
                membros: vec!["L1_demanda".to_string(), "L2_erlang_c".to_string()],
            },
            Hiperaresta {
                id: "H2".to_string(),
                lente: Lente::Escalonomia,
                membros: vec!["L1_demanda".to_string(), "L3_gestor".to_string()],
            },
            Hiperaresta {
                id: "H3".to_string(),
                lente: Lente::Crisonomia,
                membros: vec![
                    "L1_demanda".to_string(),
                    "L2_erlang_c".to_string(),
                    "L3_gestor".to_string(),
                ],
            },
        ],
    }
}

/// Um regime: base propria, efeito do tratamento e chance de ser tratado.
struct Regime {
    p_tratado: u64,
    base: f64,
    efeito: f64,
    n: usize,
}

struct Medida {
    marginal: f64,
    fracao_tratada: f64,
    por_regime: Vec<f64>,
}

/// Cada regime recebe n unidades com a fracao de tratamento pedida e residuo
/// uniforme em [-2, 2]. A comparacao marginal mistura os regimes; a comparacao
/// dentro do regime isola o efeito.
fn medir(semente: u64, regimes: &[Regime]) -> Medida {
    let mut rng = semente;
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();
    let mut por_regime: Vec<f64> = Vec::new();
    for r in regimes {
        let n = r.n;
        let mut rx: Vec<f64> = Vec::new();
        let mut ry: Vec<f64> = Vec::new();
        for _ in 0..n {
            let t = if uniforme(&mut rng, 100) < r.p_tratado { 1.0 } else { 0.0 };
            let residuo = (uniforme(&mut rng, 4_001) as f64 - 2_000.0) / 1_000.0;
            let out = r.base + r.efeito * t + residuo;
            rx.push(t);
            ry.push(out);
            x.push(t);
            y.push(out);
        }
        por_regime.push(inclinacao(&rx, &ry));
    }
    Medida {
        marginal: inclinacao(&x, &y),
        fracao_tratada: x.iter().sum::<f64>() / x.len() as f64,
        por_regime,
    }
}

pub fn rodar() {
    println!("== Capitulo 12 :: o mapa do conhecimento ==");

    let g = grafo_valido();
    let ordem = g.ordem_topologica().unwrap_or_default();
    let prof = g.profundidades();
    let folhas = prof.iter().filter(|(_, d)| *d == 1).count();
    let raizes = prof.iter().filter(|(_, d)| *d > 1).count();
    println!(
        "grafo: nos={} (L1={} L2={} L3={}) E_tree={} E_graph={} hiperarestas={} validacao={:?}",
        g.nos.len(),
        g.por_camada(Camada::L1),
        g.por_camada(Camada::L2),
        g.por_camada(Camada::L3),
        g.tree.len(),
        g.graph.len(),
        g.hiperarestas.len(),
        g.validar()
    );
    println!(
        "ordem topologica = {:?} | folhas={} raizes={} | nos sem lente = {:?}",
        ordem, folhas, raizes, g.nos_sem_lente()
    );

    let mut g1 = g.clone();
    g1.tree.push(aresta("L2_erlang_c", "L2_no_fantasma", TipoAresta::Dedutiva, "ponte quebrada"));
    println!("prova 1 (aresta para no ausente): {:?}", g1.validar());

    let mut g2 = g.clone();
    g2.tree.push(aresta("L3_gestor", "L2_erlang_c", TipoAresta::Dedutiva, "volta"));
    println!("prova 2 (ciclo dedutivo): {:?}", g2.validar());

    let mut g3 = g.clone();
    g3.hiperarestas.push(Hiperaresta {
        id: "H4".to_string(),
        lente: Lente::Fluxonomia,
        membros: vec!["L1_demanda".to_string()],
    });
    println!("prova 3 (hiperaresta com um membro): {:?}", g3.validar());

    let mut g4 = g.clone();
    g4.nos.push(no("L2_orfao", Camada::L2, "no que ninguem olha"));
    println!(
        "prova 4 (no fora de toda lente): {:?} validacao={:?}",
        g4.nos_sem_lente(),
        g4.validar()
    );

    // Quais lentes o grafo aciona e quais ficam sem uso.
    let mut usadas: Vec<&str> = g.hiperarestas.iter().map(|h| h.lente.nome()).collect();
    usadas.sort();
    let nao_acionadas: Vec<&str> = Lente::TODAS
        .iter()
        .map(|l| l.nome())
        .filter(|n| !usadas.contains(n))
        .collect();
    println!(
        "lentes acionadas = {:?} | nao acionadas = {:?}",
        usadas, nao_acionadas
    );

    // Lente 1 — Escalonomia.
    let mut rng = 1909u64;
    let serie: Vec<f64> = (0..20_000)
        .map(|_| {
            let u = (splitmix64(&mut rng) as f64) / (u64::MAX as f64);
            (1.0 - u).powf(-1.0 / 1.81)
        })
        .collect();
    for k in [250usize, 500, 1_000, 2_000] {
        let a = indice_hill(&serie, k);
        println!(
            "Escalonomia: hill_alfa(k={}) = {:.4} cauda_selvagem={} valores_acima_de_1000={}",
            k,
            a,
            cauda_selvagem(a),
            serie.iter().filter(|x| **x > 1_000.0).count()
        );
    }

    // Lente 2 — Crisonomia.
    let p_ok: f64 = 0.70;
    let p_falha: f64 = 0.30;
    let lucro: f64 = 10.0;
    let perda: f64 = 4.0;
    let agir: f64 = lucro * p_ok - perda * p_falha;
    let evpi_do_sensor = evpi(&[lucro, -perda], &[p_ok, p_falha], agir);
    println!(
        "Crisonomia: EVPI = {:.4} | melhor ato sem informacao = {:.4} | valor com informacao perfeita = {:.4}",
        evpi_do_sensor,
        agir,
        lucro * p_ok + 0.0 * p_falha
    );
    // Sem a opcao de agir, a informacao passa a valer.
    let agir_metade: f64 = 0.5 * lucro - 0.5 * perda;
    let sem_terceira: f64 = (lucro * p_falha * 0.0) + (0.0 * p_falha).max(agir_metade * p_ok);
    println!(
        "Crisonomia: sem a opcao de agir (so apostar metade = {:.4} ou nao apostar), o melhor sem informacao e {:.4} e o EVPI sobe para {:.4}",
        agir_metade,
        sem_terceira.max(0.0),
        evpi(&[lucro, -perda], &[p_ok, p_falha], sem_terceira.max(0.0))
    );
    println!(
        "Crisonomia: sensor inutil (pagamento igual nos dois estados): EVPI = {:.4}",
        evpi(&[lucro, lucro], &[p_ok, p_falha], lucro)
    );

    // Lente 3 — Morfonomia.
    // Regimes 1, 2 e 3 com bases 100, 140 e 120 e efeito verdadeiro 10 nos tres.
    let regimes = [
        Regime { p_tratado: 25, base: 100.0, efeito: 10.0, n: 240 },
        Regime { p_tratado: 75, base: 140.0, efeito: 10.0, n: 240 },
        Regime { p_tratado: 50, base: 120.0, efeito: 10.0, n: 240 },
    ];
    let m = medir(2026, &regimes);
    let dois = |v: &[f64]| v.iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<f64>>();
    println!(
        "Morfonomia: regra 1 -> inclinacao marginal = {:.2} (fracao tratada = {:.3}) | por regime = {:?} | invariante(0,5) = {}",
        m.marginal,
        m.fracao_tratada,
        dois(&m.por_regime),
        invariante(&m.por_regime, 0.5)
    );
    // Mesmo efeito verdadeiro, alocacao trocada: o marginal muda de sinal.
    let regimes_trocados = [
        Regime { p_tratado: 75, base: 100.0, efeito: 10.0, n: 240 },
        Regime { p_tratado: 25, base: 140.0, efeito: 10.0, n: 240 },
        Regime { p_tratado: 50, base: 120.0, efeito: 10.0, n: 240 },
    ];
    let t = medir(2026, &regimes_trocados);
    println!(
        "Morfonomia: regra 1 com a alocacao trocada -> inclinacao marginal = {:.2} | por regime = {:?} | invariante(0,5) = {}",
        t.marginal,
        dois(&t.por_regime),
        invariante(&t.por_regime, 0.5)
    );
    // Efeito que quebra a invariancia: o regime do meio responde 40, nao 10.
    let regimes_quebrados = [
        Regime { p_tratado: 25, base: 100.0, efeito: 10.0, n: 240 },
        Regime { p_tratado: 75, base: 140.0, efeito: 40.0, n: 240 },
        Regime { p_tratado: 50, base: 120.0, efeito: 10.0, n: 240 },
    ];
    let q = medir(2026, &regimes_quebrados);
    println!(
        "Morfonomia: regra 2, regime do meio com efeito 40 -> inclinacao marginal = {:.2} | por regime = {:?} | invariante(0,5) = {} | passo dentro do regime = {:.2}",
        q.marginal,
        dois(&q.por_regime),
        invariante(&q.por_regime, 0.5),
        q.por_regime[1] - q.por_regime[0]
    );
}
