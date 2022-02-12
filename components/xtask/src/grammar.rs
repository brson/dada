use crate::utils;
use eyre::WrapErr;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Grammar {
    /// Build the reference parser from the reference grammar.
    Build,
    /// Determine the maximum lookahead required by the reference grammar.
    Decidable,
}

static GRAMMAR_DIR: &str = "components/dada-reference-grammar/src";
static GRAMMAR_FILE: &str = "grammar.par";
static GRAMMAR_EXPANDED: &str = "grammar-exp.par";
static PARSER_SOURCE: &str = "parser.rs";
static GRAMMAR_TRAIT_SOURCE: &str = "grammar_trait.rs";
static GRAMMAR_TYPE: &str = "Grammar";
static GRAMMAR_MODULE: &str = "grammar";

impl Grammar {
    pub fn main(&self) -> eyre::Result<()> {
        xshell::Cmd::new("parol")
            .arg("--version")
            .run()
            .wrap_err_with(|| "`parol` not installed - run `cargo install parol`")?;

        let xtask_dir = utils::cargo_path("CARGO_MANIFEST_DIR")?;
        let manifest_dir = xtask_dir.parent().unwrap().parent().unwrap();
        let grammar_src_dir = manifest_dir.join(GRAMMAR_DIR);
        let _p = xshell::pushd(grammar_src_dir)?;
        match self {
            Grammar::Build => {
                // -e grammar-exp.par -p parser.rs -a grammar_trait.rs -t Grammar -m grammar
                xshell::Cmd::new("parol")
                    .arg("--file").arg(GRAMMAR_FILE)
                    .arg("--expanded").arg(GRAMMAR_EXPANDED)
                    .arg("--parser").arg(PARSER_SOURCE)
                    .arg("--actions").arg(GRAMMAR_TRAIT_SOURCE)
                    .arg("--user_type").arg(GRAMMAR_TYPE)
                    .arg("--module").arg(GRAMMAR_MODULE)
                    .run()?;
            }
            Grammar::Decidable => {
                xshell::Cmd::new("parol")
                    .arg("decidable")
                    .arg("--grammar-file").arg(GRAMMAR_FILE)
                    .run()?;
            }
        }

        Ok(())
    }
}
