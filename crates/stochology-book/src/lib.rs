//! Laboratórios do livro didático em 21 capítulos.
//!
//! Regra do livro: todo número citado no texto sai deste crate, com semente
//! fixa, e pode ser reproduzido pelo comando ~labs --cap N~.

pub mod banco;
pub mod cap01;
pub mod fila;
pub mod genetica;
pub mod escuta;

/// A cadeia do H3 vive no crate do H3; aqui fica só a ponte para o laboratório.
pub use stochology_h3::cadeia;
pub mod cap02;
pub mod cap03;
pub mod cap04;
pub mod cap05;
pub mod cap06;
pub mod cap07;
pub mod cap08;
pub mod cap09;
pub mod cap10;
pub mod cap11;
pub mod cap12;
pub mod cap13;
pub mod cap14;
pub mod cap15;
pub mod cap16;
pub mod cap17;
pub mod cap18;
pub mod cap19;
pub mod cap20;
pub mod cap21;
