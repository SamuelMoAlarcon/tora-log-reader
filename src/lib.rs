//! Núcleo do ToraLogReader: leitura, cache, índice, codificações e gerador de logs.
//!
//! Não depende da interface gráfica. As decisões de projeto estão em `Helpers/Objetivo.md`.

#![forbid(unsafe_code)]

pub mod config;
pub mod display;
pub mod encoding;
pub mod filter;
pub mod index;
pub mod loggen;
pub mod reader;
pub mod watch;
