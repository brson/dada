//! To generate:
//!
//!     (cd components/dada-reference-grammar/src/ && parol -f grammar.par -e grammar-exp.par -p parser.rs -a grammar_trait.rs -t Grammar -m grammar)
//!
//! or
//!
//!     cargo xtask grammar build

#![allow(unused)]

#[macro_use]
extern crate lazy_static;

mod parser;
mod grammar;
mod grammar_trait;

use miette::Diagnostic;
use std::path::Path;

pub fn try_accept(filename: &Path, text: &str) -> eyre::Result<()> {
    let mut grammar = grammar::Grammar::default();
    let r = parser::parse(text, filename, &mut grammar);
    // FIXME: there must be a better way to get an eyre type here
    let r = r.map_err(|e| {
        eyre::Report::msg(e)
    });
                      
    r?;
    Ok(())
}
