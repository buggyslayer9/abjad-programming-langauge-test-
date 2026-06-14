//! Abjad Programming Language Compiler
//! 
//! This is the official compiler for the Abjad programming language,
//! an Arabic-first, high-performance systems programming language.

pub mod ast;
pub mod cli;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod type_checker;

pub use error::{AbjadError, Result};
