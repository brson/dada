use dada_ir::{diagnostic::Diagnostic, item::Item, word::Word};

#[salsa::db(
    dada_check::Jar,
    dada_ir::Jar,
    dada_lex::Jar,
    dada_manifest::Jar,
    dada_parse::Jar
)]
#[derive(Default)]
pub struct Db {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Db {
    fn salsa_runtime(&self) -> &salsa::Runtime {
        self.storage.runtime()
    }
}

impl salsa::ParallelDatabase for Db {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(Db {
            storage: self.storage.snapshot(),
        })
    }
}

impl Db {
    pub fn update_file(&mut self, filename: Word, source_text: String) {
        dada_manifest::source_text::set(self, filename, source_text)
    }

    /// Checks `filename` for compilation errors and returns all relevant diagnostics.
    pub fn diagnostics(&self, filename: Word) -> Vec<Diagnostic> {
        dada_check::check_filename::accumulated::<dada_ir::diagnostic::Diagnostics>(self, filename)
    }

    /// Parses `filename` and returns a lits of the items within.
    pub fn items(&self, filename: Word) -> Vec<Item> {
        dada_parse::parse_file(self, filename).clone()
    }
}
