//! Hipergrafo tipado G = (V, E_tree, E_graph, H, Phi) e as cinco lentes.

pub mod lentes;

use stochology_core::{Camada, TipoIncerteza};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct No {
    pub id: String,
    pub camada: Camada,
    pub incerteza: TipoIncerteza,
    pub definicao: String,
    pub artefato: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArestaTree {
    pub de: String,
    pub para: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TipoAresta {
    Dedutiva,
    PonteMatematica,
    Causal,
    Patologia,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArestaGraph {
    pub de: String,
    pub para: String,
    pub tipo: TipoAresta,
}

/// As cinco lentes operacionais.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lente {
    Fluxonomia,
    Crononomia,
    Escalonomia,
    Morfonomia,
    Crisonomia,
}

impl Lente {
    pub const TODAS: [Lente; 5] = [
        Lente::Fluxonomia,
        Lente::Crononomia,
        Lente::Escalonomia,
        Lente::Morfonomia,
        Lente::Crisonomia,
    ];

    pub fn nome(self) -> &'static str {
        match self {
            Lente::Fluxonomia => "Fluxonomia",
            Lente::Crononomia => "Crononomia",
            Lente::Escalonomia => "Escalonomia",
            Lente::Morfonomia => "Morfonomia",
            Lente::Crisonomia => "Crisonomia",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hiperaresta {
    pub id: String,
    pub lente: Lente,
    pub membros: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Grafo {
    pub nos: Vec<No>,
    pub arestas_tree: Vec<ArestaTree>,
    pub arestas_graph: Vec<ArestaGraph>,
    pub hiperarestas: Vec<Hiperaresta>,
}

impl Grafo {
    /// Valida estrutura: ids únicos, pontas existentes, árvore sem ciclo,
    /// hiperarestas com pelo menos dois membros.
    pub fn validar(&self) -> Vec<String> {
        let mut problemas = Vec::new();
        let mut vistos: Vec<&str> = Vec::new();
        for no in &self.nos {
            if no.id.trim().is_empty() || no.definicao.trim().is_empty() {
                problemas.push(format!("no incompleto: {}", no.id));
            }
            if vistos.contains(&no.id.as_str()) {
                problemas.push(format!("no duplicado: {}", no.id));
            }
            vistos.push(no.id.as_str());
        }

        let existe = |id: &str| self.nos.iter().any(|n| n.id == id);

        for a in &self.arestas_tree {
            if !existe(&a.de) || !existe(&a.para) {
                problemas.push(format!(
                    "aresta tree para no ausente: {} -> {}",
                    a.de, a.para
                ));
            }
        }
        for a in &self.arestas_graph {
            if !existe(&a.de) || !existe(&a.para) {
                problemas.push(format!(
                    "aresta graph para no ausente: {} -> {}",
                    a.de, a.para
                ));
            }
        }
        for h in &self.hiperarestas {
            if h.membros.len() < 2 {
                problemas.push(format!("hiperaresta com menos de dois membros: {}", h.id));
            }
            for m in &h.membros {
                if !existe(m) {
                    problemas.push(format!("hiperaresta {} cita no ausente: {}", h.id, m));
                }
            }
        }

        if let Some(ciclo) = self.ciclo_em_tree() {
            problemas.push(format!("ciclo em E_tree: {:?}", ciclo));
        }

        problemas
    }

    /// Ordenação topológica dos pré-requisitos; detecta ciclo dedutivo.
    pub fn ordem_topologica(&self) -> Result<Vec<String>, Vec<String>> {
        let mut ordem = Vec::new();
        let mut visitando: Vec<String> = Vec::new();
        for no in &self.nos {
            if !self.dfs_ok(&no.id, &mut ordem, &mut visitando) {
                return Err(visitando);
            }
        }
        Ok(ordem)
    }

    fn dfs_ok(&self, id: &str, ordem: &mut Vec<String>, visitando: &mut Vec<String>) -> bool {
        if ordem.iter().any(|x| x == id) {
            return true;
        }
        if visitando.iter().any(|x| x == id) {
            return false;
        }
        visitando.push(id.to_string());
        for a in &self.arestas_tree {
            if a.de == id && !self.dfs_ok(&a.para, ordem, visitando) {
                return false;
            }
        }
        visitando.retain(|x| x != id);
        ordem.push(id.to_string());
        true
    }

    fn ciclo_em_tree(&self) -> Option<Vec<String>> {
        self.ordem_topologica().err()
    }

    /// Cobertura: todo nó precisa estar em ao menos uma hiperaresta (lente).
    pub fn nos_sem_lente(&self) -> Vec<String> {
        self.nos
            .iter()
            .filter(|n| !self.hiperarestas.iter().any(|h| h.membros.contains(&n.id)))
            .map(|n| n.id.clone())
            .collect()
    }
}

#[cfg(test)]
mod testes {
    use super::{ArestaTree, Grafo, Hiperaresta, Lente, No};
    use stochology_core::{Camada, TipoIncerteza};

    fn no(id: &str) -> No {
        No {
            id: id.to_string(),
            camada: Camada::Modelo,
            incerteza: TipoIncerteza::Epistemica,
            definicao: format!("definicao curta de {}", id),
            artefato: Some(format!("artefato/{}", id)),
        }
    }

    fn grafo_ok() -> Grafo {
        Grafo {
            nos: vec![no("jensen"), no("fila"), no("espera")],
            arestas_tree: vec![ArestaTree {
                de: "jensen".to_string(),
                para: "espera".to_string(),
            }],
            arestas_graph: vec![],
            hiperarestas: vec![Hiperaresta {
                id: "H1".to_string(),
                lente: Lente::Escalonomia,
                membros: vec!["fila".to_string(), "espera".to_string()],
            }],
        }
    }

    #[test]
    fn grafo_completo_valida() {
        assert!(grafo_ok().validar().is_empty());
    }

    #[test]
    fn aresta_para_no_ausente_reprova() {
        let mut g = grafo_ok();
        g.arestas_tree.push(ArestaTree {
            de: "fila".to_string(),
            para: "no-fantasma".to_string(),
        });
        assert!(g.validar().iter().any(|p| p.contains("no ausente")));
    }

    #[test]
    fn ciclo_dedutivo_reprova() {
        let mut g = grafo_ok();
        g.arestas_tree.push(ArestaTree {
            de: "espera".to_string(),
            para: "jensen".to_string(),
        });
        assert!(g.validar().iter().any(|p| p.contains("ciclo")));
    }

    #[test]
    fn no_sem_lente_e_apontado() {
        let g = grafo_ok();
        assert_eq!(g.nos_sem_lente(), vec!["jensen".to_string()]);
    }
}
