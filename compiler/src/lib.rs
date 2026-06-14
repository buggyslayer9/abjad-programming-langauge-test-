//! Abjad Programming Language Compiler
//! 
//! This is the official compiler for the Abjad programming language,
//! an Arabic-first, high-performance systems programming language.

pub mod ast;
pub mod borrow_checker;
pub mod build_system;
pub mod cli;
pub mod codegen;
pub mod debugger;
pub mod doc_generator;
pub mod error;
pub mod formatter;
pub mod heap_allocation;
pub mod linker;
pub mod lexer;
pub mod linter;
pub mod memory_layout;
pub mod memory_safety;
pub mod object_file;
pub mod package_manager;
pub mod parser;
pub mod profiler;
pub mod stack_allocation;
pub mod token;
pub mod type_checker;

pub use error::{AbjadError, Result};
