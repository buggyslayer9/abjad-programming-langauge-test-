//! Abjad Programming Language Compiler
//! 
//! This is the official compiler for the Abjad programming language,
//! an Arabic-first, high-performance systems programming language.

pub mod cli;
pub mod error;
pub mod lexer;
pub mod token;

pub use error::{AbjadError, Result};
