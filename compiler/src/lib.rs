//! Abjad Programming Language Compiler
//! 
//! This is the official compiler for the Abjad programming language,
//! an Arabic-first, high-performance systems programming language.

pub mod ast;
pub mod borrow_checker;
pub mod cli;
pub mod codegen;
pub mod error;
pub mod heap_allocation;
pub mod linker;
pub mod lexer;
pub mod memory_layout;
pub mod memory_safety;
pub mod object_file;
pub mod parser;
pub mod stack_allocation;
pub mod token;
pub mod type_checker;

pub use error::{AbjadError, Result};
