//! # Introduction
//! BFBFE IR is a custom intermediate representation (IR) designed to
//! efficiently represent Brainfuck while providing complex instructions for
//! equally complex patterns.
//!
//! BFBFE IR is designed to be optimized by a compiler, providing
//! the necessary semantics for performing optimizations regarding data
//! manipulation and traversal, math/arithmetic, I/O, conditionals/general
//! logic, and instruction blocks.
//!
//! # Usage
//!
//! ## Compiling Brainfuck to BFBFE IR
//! ```
//! // bfbfe-lang provides tooling for working with the Brainfuck language
//! extern crate bfbfe_lang;
//!
//! use bfbfe_lang::token::Token;
//!
//! let content = "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++.----.++++.-.";
//! let tokens: Vec<Token> = bfbfe_lang::lexer::tokenize_whole_program(&content.chars().collect<Vec<_>>());
//! let ir_block: IRBlock = bfbfe_lang::transformer::instructionize(&tokens);
//!
//! // Display BFBFE IR in visual form (without pretty printing because it'll be huge!)
//! println!("{:?}", &ir_block);
//! ```
//!
//! ## Optimizing BFBFE IR
//! ```
//! let mut ir_block: IRBlock = ...;
//! ir_block = ir_block.optimize();
//!
//! // Take a look at what's changed (pretty printed because it should be significantly smaller)
//! println!("{:#?}", &ir_block);
//! ```

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod block;
pub mod instruction;
pub mod optimization;
