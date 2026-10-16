//! Núcleo da Estocologia: camadas L1-L3, invariantes Phi1-Phi4,
//! banca adversarial A1-A8 e orquestrador M.
//!
//! Todo artefato é determinístico: nenhuma função usa relógio, entropia do
//! sistema ou estado global. As sementes são explícitas em cada ficha.

pub mod agentes;
pub mod camada;
pub mod erlang;
pub mod funil;
pub mod invariantes;
pub mod orquestrador;
pub mod resolucao;
pub mod rng;
pub mod s1;

pub use agentes::{Aprovacao, AgenteA, Severidade, SEVERIDADE_MINIMA, aprovado, estressar};
pub use camada::{Camada, Ficha, TipoIncerteza};
pub use invariantes::{Falha, Portao, avaliar, relatorio};
pub use orquestrador::OrquestradorM;
pub use resolucao::{PerdaDeResolucao, Resolucao, medir as medir_perda_de_resolucao};
pub use rng::Rng;
