use dada_ir::code::syntax::Expr;
use dada_ir::filename::Filename;
use dada_ir::span::{FileSpan, Span};
use dada_lex::prelude::*;
use salsa::DebugWithDb;
use dada_ir::token::Token;
use dada_parse::prelude::*;
use dada_collections::Map;
use dada_execute::interpreter::Interpreter;
use dada_execute::kernel::Kernel;

pub struct Evaluator<'me> {
    db: &'me mut dada_db::Db,
    kernel: &'me dyn Kernel,
    filename: Filename,
    items: Map<ItemName, ItemText>,
    item_indexes: Vec<ItemName>,
}

type ItemName = String;
type ItemText = String;

impl<'me> Evaluator<'me> {
    pub fn new(db: &'me mut dada_db::Db, kernel: &'me dyn Kernel) -> Evaluator<'me> {

        let filename = Filename::from(db, "<repl-input>");
        let initial_source = String::new();

        db.update_file(filename, initial_source);

        Evaluator {
            db,
            kernel,
            filename,
            items: Map::default(),
            item_indexes: Vec::default(),
        }
    }

    pub fn add_items(&mut self, items: Vec<(ItemName, ItemText)>) -> eyre::Result<()> {
        for (name, text) in items {
            if !self.items.contains_key(&name) {
                self.item_indexes.push(name.clone());
                self.items.insert(name, text);
            } else {
                self.items.insert(name.clone(), text);
            }
        }

        let module_source = self.create_items_source();
        self.db.update_file(self.filename, module_source);

        let diagnostics = self.db.diagnostics(self.filename);
        for diagnostic in diagnostics {
            dada_error_format::print_diagnostic(self.db, &diagnostic)?;
        }

        Ok(())
    }

    pub async fn eval_expr(&mut self, text: String) -> eyre::Result<()> {
        let expr_fn = format!("async fn __repl_expr() {{\n    {}\n}}", text);

        let mut module_source = self.create_items_source();
        module_source.push_str(&expr_fn);
        self.db.update_file(self.filename, module_source);

        let diagnostics = self.db.diagnostics(self.filename);
        for diagnostic in diagnostics {
            dada_error_format::print_diagnostic(self.db, &diagnostic)?;
        }

        let repl_fn = self.db.function_named(self.filename, "__repl_expr").expect("repl fn");
        let res = dada_execute::interpret(repl_fn, self.db, self.kernel, vec![]).await;

        match res {
            Ok(()) => Ok(()),
            Err(e) => {
                eprintln!("{}", e);
                Ok(())
            }
        }
    }

    fn create_items_source(&self) -> String {
        let mut source = String::new();
        for name in &self.item_indexes {
            let item = self.items.get(name).expect("source");
            source.push_str(&item);
            source.push_str("\n\n");
        }

        source
    }
}
