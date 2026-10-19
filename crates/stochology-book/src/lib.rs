//! Laboratories of the textbook in 21 chapters.
//!
//! Book rule: every number cited in the text comes out of this crate, with a fixed seed,
//! and can be reproduced by the command ~labs --chapter N~.

pub mod banco;
pub mod ch01;
pub mod ch02;
pub mod ch03;
pub mod ch04;
pub mod ch05;
pub mod ch06;
pub mod ch07;
pub mod ch08;
pub mod ch09;
pub mod ch10;
pub mod ch11;
pub mod ch12;
pub mod ch13;
pub mod ch14;
pub mod ch15;
pub mod ch16;
pub mod ch17;
pub mod ch18;
pub mod ch19;
pub mod ch20;
pub mod ch21;
pub mod costura;
pub mod ensaio;
pub mod eprocess;
pub mod escuta;
pub mod fila;
pub mod genetica;
pub mod ponte;
pub mod segunda;
pub mod streams;

/// The Jev-LLM bridge lives in the System 2 crate; here stays only the bridge to the laboratory.
pub use stochology_s2::bridge as jev;

/// The H3 chain lives in the H3 crate; here stays only the bridge to the laboratory.
pub use stochology_horizons::h3::chain;
