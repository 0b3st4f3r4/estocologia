//! Motor ondulatorio da Estocologia: hipergrafo magnetico, plasticidade de fase e espiculas.
//!
//! Tres camadas, na ordem em que o Sistema 1 e o Sistema 2 se revezam:
//!
//! 1. **Superposicao local** (`fasor`): o Phi de uma hiperaresta avaliado como fasor. A energia
//!    tem um termo incoerente e um termo cruzado de interferencia; a coerencia C(h) decide entre
//!    propagacao direta (Sistema 1), batimento (Crononomia) e conflito (Sistema 2).
//! 2. **Propagacao multi-salto** (`magnetico`): Laplaciano magnetico Hermiteano L^(q) com a carga
//!    direcional em alfa e o propagador unitario de Cayley. Norma preservada, cadeia de selos
//!    auditavel, deteccao de chegadas que se anulam.
//! 3. **Aprendizado** (`plasticidade` e `espicula`): Kuramoto-Oja consolida o veredito do Sistema 2
//!    com portao de ganho e reversao; o barramento de espiculas de 40 bytes aprende a seta causal
//!    entre as cinco lentes por STDP fasorial e reconhece palavras polifronicas.
//!
//! Divergencias declaradas em relacao ao rascunho que originou este crate:
//!
//! - a frustracao <Psi, L Psi> nao serve de gatilho unico: na antifase simetrica ela vale zero
//!   exatamente quando o conflito existe. O gatilho e a coerencia das chegadas por vertice, e a
//!   frustracao fica como diagnostico;
//! - o portao da plasticidade cobra ganho de ressonancia no alvo, nao queda monotona de frustracao.
//!
//! Nada aqui usa dependencia externa; o estado e deterministico e o `onda --check` derruba o
//! relatorio quando qualquer um dos seis portoes falha.

pub mod complexo;
pub mod espicula;
pub mod fasor;
pub mod magnetico;
pub mod plasticidade;
pub mod portao;
pub mod selo;
