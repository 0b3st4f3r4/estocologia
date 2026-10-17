//! Hipergrafo magnetico: o Laplaciano Hermiteano L^(q), o propagador unitario de Cayley e a
//! auditoria por cadeia de selos.
//!
//! A matriz de incidencia complexa B_q(v, h) = eta_{vh} * e^{i 2 pi q alfa_{vh}} carrega o peso
//! topologico em eta e a carga direcional em alfa. Com W_e os pesos das hiperarestas,
//!
//!   A^(q) = D_v^{-1/2} B_q W_e D_e^{-1} B_q^dagger D_v^{-1/2},   L^(q) = I - A^(q).
//!
//! Como B_q W_e D_e^{-1} B_q^dagger e Hermiteano, L^(q) tambem e, e o passo de Cayley
//! (I + i dt L / 2)^{-1} (I - i dt L / 2) preserva a norma l2 a menos do erro de ponto flutuante.
//! Euler explicito nao preserva: a norma cresce a cada salto.

use std::f64::consts::PI;

use crate::complexo::Complexo;
use crate::selo::{SEMENTE_CADEIA, Selo};

const TOLERANCIA_HERMITIANA: f64 = 1e-11;

#[derive(Debug, Clone, PartialEq)]
pub struct Incidencia {
    pub vertice: usize,
    pub peso: f64,
    pub alfa: f64,
}

impl Incidencia {
    pub fn nova(vertice: usize, peso: f64, alfa: f64) -> Self {
        Self {
            vertice,
            peso,
            alfa,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HiperarestaMag {
    pub id: String,
    pub peso: f64,
    pub incidencias: Vec<Incidencia>,
}

impl HiperarestaMag {
    pub fn nova(id: &str, peso: f64, incidencias: Vec<Incidencia>) -> Self {
        Self {
            id: id.to_string(),
            peso,
            incidencias,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HipergrafoMag {
    pub vertices: Vec<String>,
    pub hiperarestas: Vec<HiperarestaMag>,
    pub q: f64,
}

/// Diagnostico de um estado sobre o hipergrafo.
#[derive(Debug, Clone, PartialEq)]
pub struct Propagacao {
    pub estado: Vec<Complexo>,
    pub norma2_inicial: f64,
    pub norma2: f64,
    /// <Psi, L Psi>, a energia de gradiente do estado.
    pub frustracao: f64,
    /// |(L Psi)_v| por vertice.
    pub tensao: Vec<f64>,
    /// Soma das contribuicoes de chegada por vertice, |A_uv| |psi_u| com u != v.
    pub entradas: Vec<f64>,
    /// Coerencia das chegadas por vertice; 0 significa chegadas que se anulam.
    pub coerencia_local: Vec<f64>,
    /// Vertices com chegada relevante e coerencia abaixo do limiar critico.
    pub dissonantes: Vec<usize>,
}

/// Um salto auditado da propagacao.
#[derive(Debug, Clone, PartialEq)]
pub struct Salto {
    pub indice: usize,
    pub estado: Vec<Complexo>,
    pub norma2: f64,
    pub frustracao: f64,
    pub dissonantes: Vec<usize>,
    pub selo_estado: u64,
    pub selo_cadeia: u64,
}

impl HipergrafoMag {
    pub fn novo(vertices: Vec<&str>, hiperarestas: Vec<HiperarestaMag>, q: f64) -> Self {
        Self {
            vertices: vertices.into_iter().map(str::to_string).collect(),
            hiperarestas,
            q,
        }
    }

    /// Problemas estruturais: indice fora da faixa, incidencia repetida, valor nao finito.
    pub fn valida(&self) -> Vec<String> {
        let mut problemas = Vec::new();
        if !self.q.is_finite() {
            problemas.push("q nao finito".to_string());
        }
        for (h, aresta) in self.hiperarestas.iter().enumerate() {
            if !aresta.peso.is_finite() || aresta.peso < 0.0 {
                problemas.push(format!("peso invalido na hiperaresta {h}"));
            }
            if aresta.incidencias.len() < 2 {
                problemas.push(format!("hiperaresta {h} com menos de duas incidencias"));
            }
            let mut vistos = Vec::new();
            for inc in &aresta.incidencias {
                if inc.vertice >= self.vertices.len() {
                    problemas.push(format!(
                        "hiperaresta {h} cita vertice inexistente {}",
                        inc.vertice
                    ));
                }
                if vistos.contains(&inc.vertice) {
                    problemas.push(format!("hiperaresta {h} repete o vertice {}", inc.vertice));
                }
                vistos.push(inc.vertice);
                if !inc.peso.is_finite() || !inc.alfa.is_finite() {
                    problemas.push(format!("incidencia nao finita na hiperaresta {h}"));
                }
            }
        }
        problemas
    }

    /// Constroi L^(q) normalizado.
    #[allow(clippy::needless_range_loop)]
    pub fn laplaciano(&self) -> Vec<Vec<Complexo>> {
        let n = self.vertices.len();
        let m = self.hiperarestas.len();
        let mut b_q = vec![vec![Complexo::ZERO; m]; n];
        let mut grau_v = vec![0.0; n];
        let mut grau_e = vec![0.0; m];

        for (h, aresta) in self.hiperarestas.iter().enumerate() {
            for inc in &aresta.incidencias {
                let theta = 2.0 * PI * self.q * inc.alfa;
                let entrada = Complexo::polar(inc.peso, theta);
                b_q[inc.vertice][h] = entrada;
                let m2 = entrada.modulo2();
                grau_e[h] += m2;
                grau_v[inc.vertice] += aresta.peso * m2;
            }
        }

        let inv_raiz_v: Vec<f64> = grau_v
            .iter()
            .map(|&d| if d > 1e-15 { 1.0 / d.sqrt() } else { 0.0 })
            .collect();
        let inv_e: Vec<f64> = grau_e
            .iter()
            .map(|&d| if d > 1e-15 { 1.0 / d } else { 0.0 })
            .collect();

        let mut lap = vec![vec![Complexo::ZERO; n]; n];
        for u in 0..n {
            for v in 0..n {
                let mut soma = Complexo::ZERO;
                for h in 0..m {
                    let escala = self.hiperarestas[h].peso * inv_e[h];
                    soma = soma + (b_q[u][h] * b_q[v][h].conj()) * escala;
                }
                let a = soma * (inv_raiz_v[u] * inv_raiz_v[v]);
                lap[u][v] = if u == v {
                    Complexo::UM - a
                } else {
                    Complexo::ZERO - a
                };
            }
        }
        lap
    }

    /// Maior desvio de Hermiticidade de L^(q); zero para o operador exato.
    #[allow(clippy::needless_range_loop)]
    pub fn assimetria_hermitiana(&self, lap: &[Vec<Complexo>]) -> f64 {
        let n = lap.len();
        let mut maior: f64 = 0.0;
        for u in 0..n {
            for v in u..n {
                let diff = (lap[u][v] - lap[v][u].conj()).modulo();
                if diff > maior {
                    maior = diff;
                }
            }
        }
        maior
    }

    pub fn confere_hermitiano(&self, lap: &[Vec<Complexo>]) -> Result<f64, String> {
        let maior = self.assimetria_hermitiana(lap);
        if maior > TOLERANCIA_HERMITIANA {
            return Err(format!(
                "L^(q) nao Hermiteano: maior desvio {maior:.3e} acima de {TOLERANCIA_HERMITIANA:.1e}"
            ));
        }
        Ok(maior)
    }

    /// Selo canonico da topologia: q, pesos das hiperarestas, pesos e angulos magneticos.
    pub fn selo_topologia(&self) -> u64 {
        let mut selo = Selo::novo();
        selo.escreve_f64(self.q);
        for aresta in &self.hiperarestas {
            selo.escreve_texto(&aresta.id);
            selo.escreve_f64(aresta.peso);
            for inc in &aresta.incidencias {
                selo.escreve_u64(inc.vertice as u64);
                selo.escreve_f64(inc.peso);
                selo.escreve_f64(inc.alfa);
            }
        }
        selo.fecha()
    }

    /// Diagnostico de um estado qualquer sobre a topologia atual.
    pub fn diagnostico(&self, estado: &[Complexo], norma2_inicial: f64) -> Propagacao {
        let lap = self.laplaciano();
        self.diagnostico_com(&lap, estado, norma2_inicial)
    }

    #[allow(clippy::needless_range_loop)]
    fn diagnostico_com(
        &self,
        lap: &[Vec<Complexo>],
        estado: &[Complexo],
        norma2_inicial: f64,
    ) -> Propagacao {
        let n = estado.len();
        let mut lpsi = vec![Complexo::ZERO; n];
        for u in 0..n {
            let mut soma = Complexo::ZERO;
            for v in 0..n {
                soma = soma + lap[u][v] * estado[v];
            }
            lpsi[u] = soma;
        }

        let mut frustracao = 0.0;
        let mut tensao = vec![0.0; n];
        let mut entradas = vec![0.0; n];
        let mut coerencia_local = vec![0.0; n];
        let mut dissonantes = Vec::new();

        for u in 0..n {
            frustracao += (estado[u].conj() * lpsi[u]).re;
            tensao[u] = lpsi[u].modulo();

            let mut chegada = Complexo::ZERO;
            let mut soma_chegadas = 0.0;
            for v in 0..n {
                if v == u {
                    continue;
                }
                chegada = chegada + lap[u][v] * estado[v];
                soma_chegadas += lap[u][v].modulo() * estado[v].modulo();
            }
            entradas[u] = soma_chegadas;
            if soma_chegadas > 1e-12 {
                coerencia_local[u] = (chegada.modulo() / soma_chegadas).clamp(0.0, 1.0);
            }
            // Chegada relevante que se anula: energia entrando e nenhuma saindo.
            if soma_chegadas > 0.10 && coerencia_local[u] < 0.35 {
                dissonantes.push(u);
            }
        }

        Propagacao {
            estado: estado.to_vec(),
            norma2_inicial,
            norma2: estado.iter().map(|c| c.modulo2()).sum(),
            frustracao: frustracao.max(0.0),
            tensao,
            entradas,
            coerencia_local,
            dissonantes,
        }
    }

    /// Propaga `saltos` passos de Cayley e devolve o diagnostico final.
    pub fn propaga(
        &self,
        inicial: &[Complexo],
        saltos: usize,
        dt: f64,
    ) -> Result<Propagacao, String> {
        let n = self.vertices.len();
        if inicial.len() != n {
            return Err(format!(
                "estado com {} posicoes para {n} vertices",
                inicial.len()
            ));
        }
        if !dt.is_finite() || dt <= 0.0 {
            return Err(format!("dt invalido: {dt}"));
        }
        let lap = self.laplaciano();
        self.confere_hermitiano(&lap)?;
        let norma2_inicial: f64 = inicial.iter().map(|c| c.modulo2()).sum();
        let mut psi = inicial.to_vec();
        for _ in 0..saltos {
            psi = passo_cayley(&lap, &psi, dt)?;
        }
        Ok(self.diagnostico_com(&lap, &psi, norma2_inicial))
    }

    /// Propaga registrando um salto auditado por passo, com selo de estado e cadeia encadeada.
    pub fn propaga_auditado(
        &self,
        inicial: &[Complexo],
        saltos: usize,
        dt: f64,
    ) -> Result<Vec<Salto>, String> {
        let n = self.vertices.len();
        if inicial.len() != n {
            return Err(format!(
                "estado com {} posicoes para {n} vertices",
                inicial.len()
            ));
        }
        if !dt.is_finite() || dt <= 0.0 {
            return Err(format!("dt invalido: {dt}"));
        }
        let lap = self.laplaciano();
        self.confere_hermitiano(&lap)?;

        let mut psi = inicial.to_vec();
        let mut historico = Vec::with_capacity(saltos + 1);
        let mut anterior = SEMENTE_CADEIA;

        for passo in 0..=saltos {
            if passo > 0 {
                psi = passo_cayley(&lap, &psi, dt)?;
            }
            let frustracao = frustracao_de(&lap, &psi);
            let estado_selo = selo_do_estado(&psi);

            let mut selo = Selo::novo();
            selo.escreve_u64(anterior);
            selo.escreve_u64(passo as u64);
            selo.escreve_u64(estado_selo);
            selo.escreve_f64(frustracao);
            let selo_cadeia = selo.fecha();
            anterior = selo_cadeia;

            let diag = self.diagnostico_com(&lap, &psi, 0.0);
            historico.push(Salto {
                indice: passo,
                estado: psi.clone(),
                norma2: diag.norma2,
                frustracao,
                dissonantes: diag.dissonantes,
                selo_estado: estado_selo,
                selo_cadeia,
            });
        }
        Ok(historico)
    }
}

/// Selo canonico de um estado fasorial.
pub fn selo_do_estado(psi: &[Complexo]) -> u64 {
    let mut selo = Selo::novo();
    for c in psi {
        selo.escreve_f64(c.re);
        selo.escreve_f64(c.im);
    }
    selo.fecha()
}

/// Energia de gradiente <Psi, L Psi>.
pub fn frustracao_de(lap: &[Vec<Complexo>], psi: &[Complexo]) -> f64 {
    let n = psi.len();
    let mut total = 0.0;
    for u in 0..n {
        let mut soma = Complexo::ZERO;
        for v in 0..n {
            soma = soma + lap[u][v] * psi[v];
        }
        total += (psi[u].conj() * soma).re;
    }
    total.max(0.0)
}

/// Um passo de Cayley: (I + i dt L / 2) psi' = (I - i dt L / 2) psi.
pub fn passo_cayley(
    lap: &[Vec<Complexo>],
    psi: &[Complexo],
    dt: f64,
) -> Result<Vec<Complexo>, String> {
    let n = psi.len();
    let meio = Complexo::novo(0.0, 0.5 * dt);
    let mut a = vec![vec![Complexo::ZERO; n]; n];
    let mut b = vec![Complexo::ZERO; n];

    for u in 0..n {
        let mut soma = Complexo::ZERO;
        for v in 0..n {
            let identidade = if u == v { Complexo::UM } else { Complexo::ZERO };
            a[u][v] = identidade + meio * lap[u][v];
            soma = soma + (identidade - meio * lap[u][v]) * psi[v];
        }
        b[u] = soma;
    }
    resolve_linear(a, b)
}

/// Eliminacao gaussiana complexa com pivotamento parcial.
#[allow(clippy::needless_range_loop)]
pub fn resolve_linear(
    mut a: Vec<Vec<Complexo>>,
    mut b: Vec<Complexo>,
) -> Result<Vec<Complexo>, String> {
    let n = b.len();
    for k in 0..n {
        let mut pivo = k;
        let mut maior = a[k][k].modulo();
        for i in (k + 1)..n {
            let m = a[i][k].modulo();
            if m > maior {
                maior = m;
                pivo = i;
            }
        }
        if maior < 1e-14 {
            return Err(format!("sistema singular no passo {k}"));
        }
        a.swap(k, pivo);
        b.swap(k, pivo);

        let p = a[k][k];
        for i in (k + 1)..n {
            let fator = match a[i][k].div(p) {
                Some(f) => f,
                None => return Err(format!("pivo degenerado no passo {k}")),
            };
            b[i] = b[i] - fator * b[k];
            for j in k..n {
                a[i][j] = a[i][j] - fator * a[k][j];
            }
        }
    }

    let mut x = vec![Complexo::ZERO; n];
    for i in (0..n).rev() {
        let mut soma = b[i];
        for j in (i + 1)..n {
            soma = soma - a[i][j] * x[j];
        }
        match soma.div(a[i][i]) {
            Some(v) => x[i] = v,
            None => return Err(format!("diagonal nula no passo {i}")),
        }
    }
    Ok(x)
}

/// Recalcula a cadeia de selos e acusa adulteracao de estado, salto ou frustracao.
pub fn verifica_cadeia(saltos: &[Salto]) -> Result<(), String> {
    let mut esperado = SEMENTE_CADEIA;
    for salto in saltos {
        let medido = selo_do_estado(&salto.estado);
        if medido != salto.selo_estado {
            return Err(format!(
                "salto {}: selo de estado diverge (declarado 0x{:016x}, medido 0x{:016x})",
                salto.indice, salto.selo_estado, medido
            ));
        }
        let mut selo = Selo::novo();
        selo.escreve_u64(esperado);
        selo.escreve_u64(salto.indice as u64);
        selo.escreve_u64(medido);
        selo.escreve_f64(salto.frustracao);
        let cadeia = selo.fecha();
        if cadeia != salto.selo_cadeia {
            return Err(format!(
                "salto {}: cadeia quebrada (declarado 0x{:016x}, medido 0x{:016x})",
                salto.indice, salto.selo_cadeia, cadeia
            ));
        }
        esperado = cadeia;
    }
    Ok(())
}

/// Fase de chegada de um vertice ao alvo, atraves de uma incidencia: usada pela plasticidade.
pub(crate) fn fase_de_chegada(
    ambiente: &[Complexo],
    hiperaresta: &HiperarestaMag,
    alvo: usize,
    q: f64,
) -> Option<(Complexo, f64, f64)> {
    let dois_pi_q = 2.0 * PI * q;
    let mut contexto = Complexo::ZERO;
    let mut alfa_alvo = None;
    let mut peso_alvo = None;
    for inc in &hiperaresta.incidencias {
        if inc.vertice == alvo {
            alfa_alvo = Some(inc.alfa);
            peso_alvo = Some(inc.peso);
        } else {
            // B_q(u, h)^* psi_u = eta e^{-i 2 pi q alfa} psi_u
            let transporte = Complexo::polar(inc.peso, -dois_pi_q * inc.alfa);
            contexto = contexto + transporte * ambiente[inc.vertice];
        }
    }
    let alfa = alfa_alvo?;
    let _ = peso_alvo?;
    let chegada = contexto.fase() + dois_pi_q * alfa;
    Some((contexto, chegada, contexto.modulo()))
}

#[cfg(test)]
pub(crate) mod testes {
    use super::*;

    pub(crate) fn grafo_construtivo(destrutivo: bool) -> HipergrafoMag {
        HipergrafoMag::novo(
            vec!["V0_MUNDO", "V1_TESE", "V2_ANTITESE", "V3_AGENTE"],
            vec![
                HiperarestaMag::nova(
                    "H0_DIFUSAO",
                    1.0,
                    vec![
                        Incidencia::nova(0, 1.0, 0.0),
                        Incidencia::nova(1, 1.0, 0.0),
                        Incidencia::nova(2, 1.0, 0.0),
                    ],
                ),
                HiperarestaMag::nova(
                    "H1_VIA_TESE",
                    1.0,
                    vec![Incidencia::nova(1, 1.0, 0.0), Incidencia::nova(3, 1.0, 0.0)],
                ),
                HiperarestaMag::nova(
                    "H2_VIA_ANTITESE",
                    1.0,
                    vec![
                        Incidencia::nova(2, 1.0, 0.0),
                        Incidencia::nova(3, 1.0, if destrutivo { 1.0 } else { 0.0 }),
                    ],
                ),
            ],
            0.5,
        )
    }

    pub(crate) fn pulso() -> Vec<Complexo> {
        vec![
            Complexo::polar(1.0, 0.0),
            Complexo::ZERO,
            Complexo::ZERO,
            Complexo::ZERO,
        ]
    }

    #[test]
    fn laplaciano_e_hermitiano() {
        let g = grafo_construtivo(false);
        let lap = g.laplaciano();
        assert!(g.assimetria_hermitiana(&lap) < 1e-14);
        assert!(g.confere_hermitiano(&lap).is_ok());
    }

    #[test]
    fn cayley_preserva_a_norma() {
        let g = grafo_construtivo(false);
        let p = g.propaga(&pulso(), 6, 0.65).expect("propaga");
        assert!((p.norma2 - 1.0).abs() < 1e-10, "norma {}", p.norma2);
        assert!((p.norma2_inicial - 1.0).abs() < 1e-12);
    }

    #[test]
    fn antiffase_no_alvo_cancela_a_chegada() {
        let construtivo = grafo_construtivo(false)
            .propaga(&pulso(), 3, 0.65)
            .expect("propaga");
        let destrutivo = grafo_construtivo(true)
            .propaga(&pulso(), 3, 0.65)
            .expect("propaga");
        let a_construtivo = construtivo.estado[3].modulo();
        let a_destrutivo = destrutivo.estado[3].modulo();
        assert!(
            a_construtivo > 0.15,
            "amplitude construtiva {}",
            a_construtivo
        );
        assert!(a_destrutivo < 1e-9, "amplitude destrutiva {}", a_destrutivo);
        assert!(destrutivo.coerencia_local[3] < 0.05);
        assert!(destrutivo.dissonantes.contains(&3));
        assert!(!construtivo.dissonantes.contains(&3));
    }

    #[test]
    fn cadeia_detecta_adulteracao() {
        let g = grafo_construtivo(false);
        let saltos = g.propaga_auditado(&pulso(), 3, 0.65).expect("auditado");
        verifica_cadeia(&saltos).expect("cadeia limpa");
        let mut corrompido = saltos.clone();
        corrompido[2].estado[3].im += 0.001;
        assert!(verifica_cadeia(&corrompido).is_err());
    }

    #[test]
    fn mesma_entrada_mesmo_selo() {
        let g = grafo_construtivo(false);
        let a = g.propaga_auditado(&pulso(), 4, 0.65).expect("a");
        let b = g.propaga_auditado(&pulso(), 4, 0.65).expect("b");
        assert_eq!(
            a.last().map(|s| s.selo_cadeia),
            b.last().map(|s| s.selo_cadeia)
        );
    }

    #[test]
    fn estado_com_tamanho_errado_e_recusado() {
        let g = grafo_construtivo(false);
        assert!(g.propaga(&[Complexo::UM], 1, 0.5).is_err());
        assert!(g.propaga(&pulso(), 1, 0.0).is_err());
    }

    #[test]
    fn validacao_pega_vertice_inexistente() {
        let g = HipergrafoMag::novo(
            vec!["A"],
            vec![HiperarestaMag::nova(
                "H",
                1.0,
                vec![Incidencia::nova(0, 1.0, 0.0), Incidencia::nova(7, 1.0, 0.0)],
            )],
            0.5,
        );
        assert!(!g.valida().is_empty());
    }
}
