use dada_ir::code::syntax::{Expr, ExprData};
use dada_ir::filename::Filename;
use dada_ir::span::{FileSpan, Span};
use dada_lex::prelude::*;
use salsa::DebugWithDb;
use dada_ir::token::Token;
use dada_parse::prelude::*;
use dada_ir::in_ir_db::InIrDbExt;
use dada_ir::item::Item;
use dada_ir::function::Function;
use dada_ir::class::Class;
use dada_ir::Db;

pub enum Step {
    EvalExpr(String),
    AddItems(Vec<(String, String)>),
    ExecCommand(Command),
}

pub enum Command {
    Exit,
    Interrupt,
    SkipStep, // whitespace input
    Reset,
}

pub struct Reader {
    rl: rustyline::Editor<()>,
}

impl Reader {
    pub fn new() -> Reader {
        let rl = rustyline::Editor::new();

        Reader { rl }
    }

    pub fn step(&mut self) -> eyre::Result<Step> {
        let next_line = self.rl.readline(">>> ");

        match next_line {
            Ok(line) => {
                self.rl.add_history_entry(&line);
                let thing = self.try_parse_thing(line)?;
                match thing {
                    ParsedThing::Whitespace => Ok(Step::ExecCommand(Command::SkipStep)),
                    ParsedThing::ReplCommand(command) => Ok(Step::ExecCommand(command)),
                    ParsedThing::OpenTokenTree(text) => {
                        self.read_multiline(text)
                    }
                    ParsedThing::Expr(text) => Ok(Step::EvalExpr(text)),
                    ParsedThing::Items(items) => Ok(Step::AddItems(items)),
                }
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                Ok(Step::ExecCommand(Command::Exit))
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                Ok(Step::ExecCommand(Command::Interrupt))
            }
            Err(e) => Err(e.into()),
        }
    }

    fn read_multiline(&mut self, mut text: String) -> eyre::Result<Step> {
        loop {
            let next_line = self.rl.readline("... ");

            return match next_line {
                Ok(next_line) => {
                    text.push_str("\n");
                    text.push_str(&next_line);
                    let thing = self.try_parse_thing(text)?;
                    match thing {
                        ParsedThing::Whitespace => unreachable!(),
                        ParsedThing::ReplCommand(_) => unreachable!(),
                        ParsedThing::OpenTokenTree(same_text) => {
                            text = same_text;
                            continue;
                        }
                        ParsedThing::Expr(text) => Ok(Step::EvalExpr(text)),
                        ParsedThing::Items(text) => Ok(Step::AddItems(text)),
                    }
                }
                Err(rustyline::error::ReadlineError::Eof) => {
                    Ok(Step::ExecCommand(Command::Exit))
                }
                Err(rustyline::error::ReadlineError::Interrupted) => {
                    Ok(Step::ExecCommand(Command::Interrupt))
                }
                Err(e) => Err(e.into()),
            };
        }
    }

    fn try_parse_thing(&mut self, text: String) -> eyre::Result<ParsedThing> {
        let input_type = determine_input_type(&text)?;
        match input_type {
            InputType::Whitespace => Ok(ParsedThing::Whitespace),
            InputType::ReplCommand => parse_repl_command(&text).map(ParsedThing::ReplCommand),
            InputType::OpenTokenTree => Ok(ParsedThing::OpenTokenTree(text)),
            InputType::Expr => Ok(ParsedThing::Expr(text)),
            InputType::Items => parse_items(&text).map(ParsedThing::Items),
            InputType::Unknown => Err(eyre::eyre!("unrecognized input type")),
        }
    }
}

enum ParsedThing {
    Whitespace,
    ReplCommand(Command),
    OpenTokenTree(String),
    Expr(String),
    Items(Vec<(String, String)>),
}

enum InputType {
    Whitespace,
    ReplCommand,
    OpenTokenTree,
    Expr,
    Items,
    Unknown,
}

fn determine_input_type(text: &str) -> eyre::Result<InputType> {
    if is_whitespace(text) {
        Ok(InputType::Whitespace)
    } else if is_repl_command(text) {
        Ok(InputType::ReplCommand)
    } else if is_open_token_tree(text) {
        Ok(InputType::OpenTokenTree)
    } else if is_expr(text) {
        Ok(InputType::Expr)
    } else if is_items(text) {
        Ok(InputType::Items)
    } else {
        Ok(InputType::Unknown)
    }
}

fn is_whitespace(text: &str) -> bool {
    text.chars().all(char::is_whitespace)
}

fn is_repl_command(text: &str) -> bool {
    text.trim().starts_with(":")
}

fn is_open_token_tree(text: &str) -> bool {
    let mut db = dada_db::Db::default();
    let filename = Filename::from(&db, "<repl-input>");
    db.update_file(filename, text.into());

    let tt = dada_lex::lex_file(&db, filename);

    let mut tokens = tt.tokens(&db).iter();
    while let Some(token) = tokens.next() {
        if let Token::Delimiter(opening) = token {
            loop {
                let next = tokens.next();
                match next {
                    Some(Token::Delimiter(closing)) => {
                        if *closing == dada_lex::closing_delimiter(*opening) {
                            break;
                        }
                    }
                    None => {
                        return true;
                    }
                    _ => {
                        // pass
                    }
                }
            }
        }
    }

    false
}

fn is_expr(text: &str) -> bool {
    let mut db = dada_db::Db::default();
    let filename = Filename::from(&db, "<repl-input>");
    db.update_file(filename, text.into());

    let tt = dada_lex::lex_file(&db, filename);
    let expr_tree = dada_parse::code_parser::parse_repl_expr(&db, tt);

    if let Some(expr_tree) = expr_tree {
        let expr_tree_data = expr_tree.data(&db);
        let root_expr = expr_tree_data.root_expr;
        let root_expr_data = &expr_tree_data.tables[root_expr];

        match root_expr_data {
            ExprData::Var(decl, rhs) => {
                println!("{:#?}", decl.debug(&expr_tree.in_ir_db(&db)));
                let local_decl = &expr_tree_data.tables[*decl];
                let name = local_decl.name.data(&db).string.clone();
                println!("{}", name);
                todo!()
            }
            _ => {
            }
        }

        true
    } else {
        false
    }
}

fn is_items(text: &str) -> bool {
    let mut db = dada_db::Db::default();
    let filename = Filename::from(&db, "<repl-input>");
    db.update_file(filename, text.into());

    let items = filename.items(&db);

    items.len() > 0
}

fn parse_repl_command(text: &str) -> eyre::Result<Command> {
    match text {
        ":exit" => Ok(Command::Exit),
        ":skip" => Ok(Command::SkipStep),
        ":reset" => Ok(Command::Reset),
        _=> Err(eyre::eyre!("unknown repl command `{}`", text)),
    }
}

fn parse_items(text: &str) -> eyre::Result<Vec<(String, String)>> {
    let mut db = dada_db::Db::default();
    let filename = Filename::from(&db, "<repl-input>");
    db.update_file(filename, text.into());

    let mut items = vec![];

    for item in filename.items(&db) {
        let name = item.name(&db);
        let name = name.as_str(&db).to_string();
        let (start, end) = match item {
            Item::Function(function) => {
                let start = function.effect_span(&db).start;
                let end = function.code(&db)
                    .body_tokens.span(&db)
                    .end;

                let start = usize::from(start);
                let end = usize::from(end);

                // The body tokens don't include the final closing brace
                let end = end + 1;

                (start, end)
            }
            Item::Class(class) => {
                let start = class.name_span(&db).start;
                let end = class.field_tokens(&db).span(&db).end;
                    
                let start = usize::from(start);
                let end = usize::from(end);

                // The field tokens don't include the final closing brace
                let end = end + 1;

                (start, end)
            }
        };

        let item_text = text[start..end].to_string();
        items.push((name, item_text));
    }

    Ok(items)
}
