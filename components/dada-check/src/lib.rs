#![allow(incomplete_features)]
#![feature(trait_upcasting)]

mod check;

#[salsa::jar(Db)]
pub struct Jar(check::check_filename);

pub trait Db:
    salsa::DbWithJar<Jar> + dada_ir::Db + dada_lex::Db + dada_parse::Db + dada_manifest::Db
{
}

impl<T> Db for T where
    T: salsa::DbWithJar<Jar> + dada_ir::Db + dada_lex::Db + dada_parse::Db + dada_manifest::Db
{
}

pub use check::check_filename;
