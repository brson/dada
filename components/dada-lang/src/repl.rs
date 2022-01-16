#![allow(unused)]

use dada_repl::read::{Reader, Command, Step};
use dada_repl::eval::{Evaluator};
use dada_repl::kernel::Kernel;

#[derive(structopt::StructOpt)]
pub struct Options {}

impl Options {
    // todo: spawn_blocking
    pub async fn main(&self, _crate_options: &crate::Options) -> eyre::Result<()> {
        let mut reader = Reader::new();

        'reset: loop {
            let mut db = dada_db::Db::default();
            let kernel = Kernel::new();
            let mut evaluator = Evaluator::new(&mut db, &kernel);

            loop {
                let next = reader.step();

                match next {
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                    Ok(Step::EvalExpr(text)) => {
                        evaluator.eval_expr(text).await?;
                    }
                    Ok(Step::AddItems(items)) => {
                        evaluator.add_items(items)?;
                    }
                    Ok(Step::ExecCommand(Command::Reset)) => {
                        continue 'reset;
                    }
                    Ok(Step::ExecCommand(Command::Exit)) => {
                        break 'reset;
                    }
                    Ok(Step::ExecCommand(Command::Interrupt)) => {
                        eprintln!("interrupted");
                    }
                    Ok(Step::ExecCommand(Command::SkipStep)) => {
                        // pass
                    }
                }
            }
        }

        Ok(())
    }
}
