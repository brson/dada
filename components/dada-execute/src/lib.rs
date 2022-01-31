#![feature(trait_upcasting)]
#![feature(try_blocks)]
#![allow(incomplete_features)]

#[salsa::jar(Db)]
pub struct Jar(ext::class_field_names);

pub trait Db:
    salsa::DbWithJar<Jar> + dada_ir::Db + dada_parse::Db + dada_brew::Db + dada_error_format::Db
{
}

impl<T> Db for T where
    T: salsa::DbWithJar<Jar> + dada_ir::Db + dada_parse::Db + dada_brew::Db + dada_error_format::Db
{
}

#[macro_use]
mod macros;

mod data;
mod error;
mod execute;
mod ext;
pub mod heap_graph;
pub mod interpreter;
mod intrinsic;
pub mod kernel;
mod moment;
mod permission;
mod poll_once;
mod thunk;
mod value;

pub use execute::interpret;
pub use execute::StackFrame;
