use crate::grammar_trait::GrammarTrait;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum GrammarItem {}

impl Display for GrammarItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO!")
    }
}

#[derive(Debug, Default)]
pub struct Grammar {
    pub item_stack: Vec<GrammarItem>,
}

impl GrammarTrait for Grammar {}
