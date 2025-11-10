//! # LOLCODE Compiler Library
//! 
//! A library for compiling LOLCODE-inspired markdown to HTML.
//! 
//! This library provides a complete compilation pipeline with four stages:
//! 
//! 1. **Lexical Analysis** - Tokenization of source code
//! 2. **Syntax Analysis** - Building an Abstract Syntax Tree
//! 3. **Semantic Analysis** - Variable scope and usage validation
//! 4. **Code Generation** - HTML output generation

pub mod token;
pub mod lexer;
pub mod parser;
pub mod semantic;