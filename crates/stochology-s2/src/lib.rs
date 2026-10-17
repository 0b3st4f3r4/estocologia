//! stochology-s2 — o Sistema 2 deliberativo: busca em arvore, estado de crenca, inferencia e estresse.
//!
//! O Sistema 1 (triagem deterministica, O(1)) vive em `stochology-core/src/s1.rs` e fecha a familia de
//! erro de uma ficha. O Sistema 2 e o modo lento que o PLAN.md §4 e o Capitulo 13 declaram: **LATS** (busca
//! em arvore com estimativa) mais o **motor de estado de crenca** sobre o hipergrafo, e mais as frentes que
//! exigem deliberacao — inferencia pesada (MCMC) e estresse adversarial.
//!
//! # O que este crate faz, por arquivo
//!
//! - `lats`: o mundo declarado (`Mundo`), a busca com UCB e o ganancioso que serve de nulo.
//! - `crenca`: o motor de estado de crenca, com piso e teto do contrato em toda probabilidade.
//! - `hipergrafo`: o mundo do Sistema 2 sobre o hipergrafo de verdade — o objetivo e o no que **nenhuma
//!   lente cobre**, medido aresta a aresta.
//! - `inferencia`: Metropolis de passo aleatorio, com o alvo declarado pelo chamador.
//! - `estresse`: a banca A1-A8 atacada na ordem que a crenca aprendeu, contra a ordem declarada A1 a A8.
//! - `planejamento`: a resolucao da pergunta (eixo R) escolhida pelo instrumento, nunca por palpite.
//!
//! # Declarado e nao implementado
//!
//! - **Sintese profunda de codigo (F1).** O `PLAN.md` §4 pede que o Sistema 2 sintetize codigo. Este crate
//!   nao gera programa: o que existe e a busca sobre o hipergrafo, a inferencia e o estresse. A frente
//!   continua declarada, e nao ha meio-caminho honesto entre a busca em arvore e um gerador de codigo.
//! - **Ordem de ataque com custo assimetrico.** O estresse deliberativo conta ataques, nao o preco de cada
//!   ataque; a ponderacao por custo por faixa segue declarada.
//! - **R2 e R3 no planejamento.** O instrumento mede o par agregado/fino (R0 e R1); as resolucoes mais finas
//!   ficam declaradas como fora do alcance medido, e nao sao estimadas por extrapolacao.

pub mod crenca;
pub mod estresse;
pub mod hipergrafo;
pub mod inferencia;
pub mod lats;
pub mod planejamento;
pub mod ponte;
pub mod runner;

pub use crenca::{EstadoCrenca, PISO, TETO};
pub use estresse::{
    Atuacao, CrencaDaBanca, ORDEM_DECLARADA, aprender, estressar_deliberativo, estressar_em_ordem,
};
pub use hipergrafo::{MundoHipergrafo, Trilha, destino_da_acao, grafo_de_amostra};
pub use inferencia::{Cadeia, Metropolis, log_beta, log_normal};
pub use lats::{Decisao, Lats, Mundo, ganancioso};
pub use planejamento::{Avaliacao, DecisaoDeResolucao, avaliar as avaliar_resolucoes, decidir};
pub use runner::{RunnerLocal, brier, hash_declarado};
