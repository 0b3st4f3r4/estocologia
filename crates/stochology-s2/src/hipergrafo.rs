//! O mundo do Sistema 2 sobre o hipergrafo: a busca chega onde nenhuma lente cobre.
//!
//! O objetivo da deliberacao sobre o hipergrafo é declarado e medido: **encontrar o no que nenhuma das
//! cinco lentes cobre**, andando pelas arestas de `E_tree` e `E_graph`. O valor de um no é
//! `1 - cobertura/5`, e a cobertura conta **lentes distintas**, nao hiperarestas repetidas.
//!
//! Declarado e nao implementado: a sintese de codigo que o PLAN.md §4 pede na mesma frase (F1); e a
//! ponderacao por tipo de aresta (hoje `Causal` e `Patologia` pesam igual a `Dedutiva`).

use std::collections::BTreeMap;
use stochology_core::{Camada, TipoIncerteza};
use stochology_hypergraph::{ArestaGraph, ArestaTree, Grafo, Hiperaresta, Lente, No, TipoAresta};

use crate::lats::{Lats, Mundo};

/// O separador entre o tipo da aresta e o destino, na acao declarada.
const SEPARADOR: char = '|';

pub fn acao(tipo: &str, destino: &str) -> String {
    format!("{tipo}{SEPARADOR}{destino}")
}

pub fn destino_da_acao(acao: &str) -> &str {
    acao.split_once(SEPARADOR).map(|(_, d)| d).unwrap_or(acao)
}

fn nome_do_tipo(tipo: TipoAresta) -> &'static str {
    match tipo {
        TipoAresta::Dedutiva => "dedutiva",
        TipoAresta::PonteMatematica => "ponte",
        TipoAresta::Causal => "causal",
        TipoAresta::Patologia => "patologia",
    }
}

/// O hipergrafo como mundo: estados sao nos, acoes sao arestas, valor é cobertura de lente.
#[derive(Debug, Clone)]
pub struct MundoHipergrafo<'a> {
    grafo: &'a Grafo,
    lentes: BTreeMap<String, Vec<Lente>>,
}

impl<'a> MundoHipergrafo<'a> {
    pub fn novo(grafo: &'a Grafo) -> Self {
        let mut lentes: BTreeMap<String, Vec<Lente>> =
            grafo.nos.iter().map(|n| (n.id.clone(), Vec::new())).collect();
        for hiperaresta in &grafo.hiperarestas {
            for membro in &hiperaresta.membros {
                if let Some(lista) = lentes.get_mut(membro) {
                    if !lista.contains(&hiperaresta.lente) {
                        lista.push(hiperaresta.lente);
                    }
                }
            }
        }
        MundoHipergrafo { grafo, lentes }
    }

    pub fn grafo(&self) -> &Grafo {
        self.grafo
    }

    /// Lentes distintas que cobrem o no.
    pub fn lentes_do_no(&self, id: &str) -> Vec<Lente> {
        self.lentes.get(id).cloned().unwrap_or_default()
    }

    /// Quantas lentes cobrem o no; `None` para no que nao existe no grafo.
    pub fn cobertura(&self, id: &str) -> Option<usize> {
        self.lentes.get(id).map(Vec::len)
    }

    /// Nos que nenhuma lente cobre, na ordem declarada do grafo.
    pub fn nos_descobertos(&self) -> Vec<&str> {
        self.grafo
            .nos
            .iter()
            .filter(|n| self.lentes_of(&n.id) == 0)
            .map(|n| n.id.as_str())
            .collect()
    }

    fn lentes_of(&self, id: &str) -> usize {
        self.cobertura(id).unwrap_or(0)
    }

    /// Valor do estado: quanto do no nenhuma lente cobre. No ausente do grafo vale zero.
    pub fn valor(&self, id: &str) -> f64 {
        match self.cobertura(id) {
            Some(quantas) => 1.0 - quantas as f64 / Lente::TODAS.len() as f64,
            None => 0.0,
        }
    }

    /// A busca passo a passo: repete o LATS a partir do estado escolhido, ate `passos` vezes.
    pub fn busca(&self, raiz: &str, lats: &Lats, passos: usize) -> Option<Trilha> {
        let mut atual = raiz.to_string();
        let mut caminho = vec![raiz.to_string()];
        let mut valor = self.valor(raiz);
        for _ in 0..passos.max(1) {
            if self.terminal(&atual) {
                break;
            }
            let decisao = lats.deliberar(self, &atual)?;
            atual = destino_da_acao(&decisao.acao).to_string();
            valor = self.valor(&atual);
            caminho.push(atual.clone());
        }
        Some(Trilha { caminho, valor, sem_lente: self.cobertura(&atual) == Some(0) })
    }
}

/// O resultado de uma busca: o caminho, o valor do no de chegada e se ele ficou sem lente.
#[derive(Debug, Clone, PartialEq)]
pub struct Trilha {
    pub caminho: Vec<String>,
    pub valor: f64,
    pub sem_lente: bool,
}

impl Mundo for MundoHipergrafo<'_> {
    fn acoes(&self, estado: &str) -> Vec<String> {
        let mut saida = Vec::new();
        for aresta in &self.grafo.arestas_tree {
            if aresta.de == estado {
                saida.push(acao("tree", &aresta.para));
            }
        }
        for aresta in &self.grafo.arestas_graph {
            if aresta.de == estado {
                saida.push(acao(nome_do_tipo(aresta.tipo), &aresta.para));
            }
        }
        saida
    }

    fn passo(&self, _estado: &str, acao: &str) -> String {
        destino_da_acao(acao).to_string()
    }

    fn avaliar(&self, estado: &str) -> f64 {
        self.valor(estado)
    }

    fn terminal(&self, estado: &str) -> bool {
        match self.cobertura(estado) {
            Some(0) => true,
            None => true,
            Some(_) => self.acoes(estado).is_empty(),
        }
    }
}

/// O grafo declarado da amostra: os horizontes, o eixo R e a frente que nenhuma lente cobre.
///
/// E um grafo declarado, nao medido: os nos sao as frentes do programa, as arestas de `E_tree` sao o que
/// depende do que, e as hiperarestas dizem qual lente cobre qual frente. O no descoberto e
/// `f1.sintese-de-codigo` — a frente que o `PLAN.md` §4 declara e que este crate nao implementa. A busca
/// do Sistema 2 chega nela pelo eixo R, e nao por atalho.
pub fn grafo_de_amostra() -> Grafo {
    let no = |id: &str, camada: Camada, incerteza: TipoIncerteza, definicao: &str| No {
        id: id.to_string(),
        camada,
        incerteza,
        definicao: definicao.to_string(),
        artefato: Some(format!("crates/stochology-{id}")),
    };
    let aresta = |de: &str, para: &str| ArestaTree { de: de.to_string(), para: para.to_string() };
    let grafo = |de: &str, para: &str, tipo: TipoAresta| ArestaGraph {
        de: de.to_string(),
        para: para.to_string(),
        tipo,
    };
    let hiper = |id: &str, lente: Lente, membros: &[&str]| Hiperaresta {
        id: id.to_string(),
        lente,
        membros: membros.iter().map(|m| (*m).to_string()).collect(),
    };
    Grafo {
        nos: vec![
            no("raiz.estocologia", Camada::Modelo, TipoIncerteza::Epistemica, "o programa"),
            no("h0.fila", Camada::Mundo, TipoIncerteza::Aleatoria, "a fila e a central de chamadas"),
            no("h1.bolsa", Camada::Mundo, TipoIncerteza::Aleatoria, "bolsa, emprego e fogo"),
            no("h2.invariancia", Camada::Modelo, TipoIncerteza::Epistemica, "as rodadas de invariância"),
            no("h3.malha", Camada::Mundo, TipoIncerteza::Epistemica, "a malha contínua"),
            no("r.resolucao", Camada::Modelo, TipoIncerteza::Epistemica, "a resolução da pergunta"),
            no("s1.triagem", Camada::Agente, TipoIncerteza::Aleatoria, "a triagem determinística"),
            no(
                "f1.sintese-de-codigo",
                Camada::Agente,
                TipoIncerteza::Knightiana,
                "a síntese profunda de código, declarada e não implementada",
            ),
        ],
        arestas_tree: vec![
            aresta("raiz.estocologia", "h0.fila"),
            aresta("raiz.estocologia", "h1.bolsa"),
            aresta("raiz.estocologia", "h2.invariancia"),
            aresta("raiz.estocologia", "h3.malha"),
            aresta("raiz.estocologia", "r.resolucao"),
            aresta("r.resolucao", "f1.sintese-de-codigo"),
        ],
        arestas_graph: vec![
            grafo("h3.malha", "r.resolucao", TipoAresta::Causal),
            grafo("s1.triagem", "h0.fila", TipoAresta::Patologia),
        ],
        hiperarestas: vec![
            hiper("lente.fluxonomia", Lente::Fluxonomia, &["h0.fila", "h1.bolsa"]),
            hiper("lente.escalonomia", Lente::Escalonomia, &["h0.fila", "h1.bolsa", "h2.invariancia"]),
            hiper("lente.crononomia", Lente::Crononomia, &["h3.malha", "r.resolucao"]),
            hiper(
                "lente.morfonomia",
                Lente::Morfonomia,
                &["raiz.estocologia", "h2.invariancia", "r.resolucao"],
            ),
            hiper("lente.crisonomia", Lente::Crisonomia, &["s1.triagem", "h3.malha"]),
        ],
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    fn no(id: &str) -> No {
        No {
            id: id.to_string(),
            camada: Camada::Modelo,
            incerteza: TipoIncerteza::Epistemica,
            definicao: format!("no {id}"),
            artefato: None,
        }
    }

    fn grafo_de_teste() -> Grafo {
        Grafo {
            nos: vec![no("raiz"), no("coberto"), no("meio"), no("descoberto")],
            arestas_tree: vec![
                ArestaTree { de: "raiz".into(), para: "coberto".into() },
                ArestaTree { de: "raiz".into(), para: "meio".into() },
                ArestaTree { de: "meio".into(), para: "descoberto".into() },
            ],
            arestas_graph: vec![ArestaGraph {
                de: "meio".into(),
                para: "descoberto".into(),
                tipo: TipoAresta::Causal,
            }],
            hiperarestas: vec![
                Hiperaresta {
                    id: "h1".into(),
                    lente: Lente::Fluxonomia,
                    membros: vec!["raiz".into(), "coberto".into()],
                },
                Hiperaresta {
                    id: "h2".into(),
                    lente: Lente::Crononomia,
                    membros: vec!["raiz".into(), "meio".into()],
                },
                Hiperaresta {
                    id: "h3".into(),
                    lente: Lente::Escalonomia,
                    membros: vec!["meio".into()],
                },
                Hiperaresta {
                    id: "h4".into(),
                    lente: Lente::Morfonomia,
                    membros: vec!["meio".into()],
                },
            ],
        }
    }

    #[test]
    fn a_cobertura_conta_lentes_distintas() {
        let grafo = grafo_de_teste();
        let mundo = MundoHipergrafo::novo(&grafo);
        assert_eq!(mundo.cobertura("raiz"), Some(2));
        assert_eq!(mundo.cobertura("meio"), Some(3));
        assert_eq!(mundo.cobertura("descoberto"), Some(0));
        assert_eq!(mundo.cobertura("ausente"), None);
        assert_eq!(mundo.lentes_do_no("meio").len(), 3);
        assert_eq!(mundo.nos_descobertos(), vec!["descoberto"]);
        assert_eq!(mundo.valor("descoberto"), 1.0);
        assert_eq!(mundo.valor("raiz"), 0.6);
    }

    #[test]
    fn a_deliberacao_acha_o_no_sem_lente_onde_o_ganancioso_para_antes() {
        let grafo = grafo_de_teste();
        let mundo = MundoHipergrafo::novo(&grafo);
        // o ganancioso olha so o vizinho: "coberto" vale 0,8 e "meio" vale 0,4
        let guloso = crate::lats::ganancioso(&mundo, "raiz").expect("decide");
        assert_eq!(destino_da_acao(&guloso.acao), "coberto");
        assert_eq!(guloso.valor, 0.8);
        // o LATS olha dois passos: de "meio" so ha uma saida, e ela vale 1,0
        let lats = Lats { profundidade: 2, orcamento: 40, exploracao: 1.0, semente: 2026 };
        let busca = mundo.busca("raiz", &lats, 3).expect("busca");
        assert_eq!(busca.caminho, vec!["raiz", "meio", "descoberto"]);
        assert!(busca.sem_lente);
        assert_eq!(busca.valor, 1.0);
    }

    #[test]
    fn a_acao_declara_o_tipo_da_aresta() {
        let grafo = grafo_de_teste();
        let mundo = MundoHipergrafo::novo(&grafo);
        let acoes = mundo.acoes("raiz");
        assert!(acoes.contains(&"tree|coberto".to_string()));
        assert!(acoes.contains(&"tree|meio".to_string()));
        let de_meio = mundo.acoes("meio");
        assert!(de_meio.contains(&"tree|descoberto".to_string()));
        assert!(de_meio.contains(&"causal|descoberto".to_string()));
        assert_eq!(destino_da_acao(&de_meio[0]), "descoberto");
        assert!(mundo.acoes("descoberto").is_empty());
        assert!(mundo.terminal("descoberto"));
    }

    #[test]
    fn a_busca_e_deterministica_no_grafo_de_teste() {
        let grafo = grafo_de_teste();
        let mundo = MundoHipergrafo::novo(&grafo);
        let lats = Lats::default();
        assert_eq!(mundo.busca("raiz", &lats, 3), mundo.busca("raiz", &lats, 3));
    }

    #[test]
    fn o_grafo_da_amostra_e_valido_e_tem_um_no_descoberto() {
        let grafo = grafo_de_amostra();
        assert!(grafo.validar().is_empty(), "o grafo declarado tem de ser valido: {:?}", grafo.validar());
        assert!(grafo.ordem_topologica().is_ok());
        assert_eq!(grafo.nos.len(), 8);
        assert_eq!(grafo.hiperarestas.len(), Lente::TODAS.len());
        let mundo = MundoHipergrafo::novo(&grafo);
        assert_eq!(mundo.nos_descobertos(), vec!["f1.sintese-de-codigo"]);
        assert_eq!(mundo.lentes_do_no("raiz.estocologia"), vec![Lente::Morfonomia]);
    }

    #[test]
    fn a_busca_no_grafo_da_amostra_chega_na_frente_sem_lente() {
        let grafo = grafo_de_amostra();
        let mundo = MundoHipergrafo::novo(&grafo);
        let lats = Lats { profundidade: 2, orcamento: 40, exploracao: 1.0, semente: 2026 };
        // o ganancioso empata nas cinco frentes de valor 0,6 e fica na primeira
        let guloso = crate::lats::ganancioso(&mundo, "raiz.estocologia").expect("decide");
        assert_eq!(destino_da_acao(&guloso.acao), "h0.fila");
        assert!((guloso.valor - 0.6).abs() < 1e-12);
        // o LATS ve que de r.resolucao se chega a frente que nenhuma lente cobre
        let busca = mundo.busca("raiz.estocologia", &lats, 3).expect("busca");
        assert_eq!(busca.caminho, vec!["raiz.estocologia", "r.resolucao", "f1.sintese-de-codigo"]);
        assert!(busca.sem_lente);
        assert_eq!(busca.valor, 1.0);
    }

    #[test]
    fn no_ausente_do_grafo_nao_vale_nada() {
        let grafo = grafo_de_teste();
        let mundo = MundoHipergrafo::novo(&grafo);
        assert_eq!(mundo.valor("nao-existe"), 0.0);
        assert!(mundo.terminal("nao-existe"));
        assert!(mundo.lentes_do_no("nao-existe").is_empty());
    }
}
