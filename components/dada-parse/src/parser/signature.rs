use crate::parser::Parser;

use dada_ir::{
    code::syntax::{
        op::Op, AsyncKeyword, AsyncKeywordData, AtomicKeyword, AtomicKeywordData, EffectKeyword,
        FnDecl, FnDeclData, GenericParameter, LocalVariableDecl, LocalVariableDeclData, ReturnTy,
        ReturnTyData,
    },
    kw::Keyword,
};

use super::{CodeParser, OrReportError, ParseList, SpanFallover};

impl CodeParser<'_, '_> {
    pub(crate) fn parse_fn(&mut self) -> Option<FnDecl> {
        let (kw_span, _) = self.eat(Keyword::Fn)?;
        Some(self.add(FnDeclData::Fn, kw_span))
    }

    pub(crate) fn parse_class(&mut self) -> Option<FnDecl> {
        let (kw_span, _) = self.eat(Keyword::Class)?;
        Some(self.add(FnDeclData::Class, kw_span))
    }

    pub(crate) fn parse_effect(&mut self) -> Option<EffectKeyword> {
        if let Some(k) = self.parse_atomic() {
            Some(EffectKeyword::Atomic(k))
        } else if let Some(k) = self.parse_async() {
            Some(EffectKeyword::Async(k))
        } else {
            None
        }
    }

    pub(crate) fn parse_atomic(&mut self) -> Option<AtomicKeyword> {
        let (kw_span, _) = self.eat(Keyword::Atomic)?;
        Some(self.add(AtomicKeywordData, kw_span))
    }

    pub(crate) fn parse_async(&mut self) -> Option<AsyncKeyword> {
        let (kw_span, _) = self.eat(Keyword::Async)?;
        Some(self.add(AsyncKeywordData, kw_span))
    }

    /// Parses an (optional) return type declaration from a function.
    pub(crate) fn parse_return_type(&mut self) -> Option<ReturnTy> {
        let right_arrow = self.eat_op(Op::RightArrow)?;
        let ty = self.parse_ty();
        Some(self.add(ReturnTyData { ty }, self.span_consumed_since(right_arrow)))
    }

    /// Parses an optional list of generic parameters delimited by `[]`.
    pub(crate) fn parse_generic_parameters(&mut self) -> Vec<GenericParameter> {
        let Some((_, generic_tokens)) = self.delimited('[') else {
            return vec![];
        };
        let mut subparser = Parser::new(self.db, generic_tokens);
        let mut subcodeparser = CodeParser {
            parser: &mut subparser,
            tables: self.tables,
            spans: self.spans,
        };
        subcodeparser.parse_only_generic_parameters()
    }

    fn parse_only_generic_parameters(&mut self) -> Vec<GenericParameter> {
        let params = self.parse_list(true, CodeParser::parse_generic_parameter);
        self.emit_error_if_more_tokens("extra tokens after generic parameters");
        params
    }

    fn parse_generic_parameter(&mut self) -> Option<GenericParameter> {
        if let Some(name) = self.parse_name() {
            Some(GenericParameter::Type(name))
        } else if let Some(perm) = self.parse_perm() {
            let name = self
                .parse_name()
                .or_report_error(self, || "expected name after permission")?;
            Some(GenericParameter::Permission(perm, name))
        } else {
            None
        }
    }

    /// Parses a list of parameters delimited by `()`.
    pub(crate) fn parse_parameter_list(&mut self) -> Option<Vec<LocalVariableDecl>> {
        let (_, parameter_tokens) = self.delimited('(')?;
        let mut subparser = Parser::new(self.db, parameter_tokens);
        let mut subcodeparser = CodeParser {
            parser: &mut subparser,
            tables: self.tables,
            spans: self.spans,
        };
        Some(subcodeparser.parse_only_parameters())
    }

    fn parse_only_parameters(&mut self) -> Vec<LocalVariableDecl> {
        let p = self.parse_list(true, CodeParser::parse_parameter);
        self.emit_error_if_more_tokens("extra tokens after parameters");
        p
    }

    fn parse_parameter(&mut self) -> Option<LocalVariableDecl> {
        // Parse an optional "atomic" keyword.
        let atomic = self.parse_atomic();

        // Parse the name: if there is no name, then return None, but if we saw the atomic
        // keyword, we can report an error.
        let Some(name) = self.parse_name() else {
            if let Some(atomic) = atomic {
                let atomic_span = self.spans[atomic];
                self.error_at_current_token("expected parameter name after `atomic`")
                    .secondary_label(atomic_span, "`atomic` specified here")
                    .emit(self.db);
            }

            return None;
        };

        let ty = self.parse_colon_ty();

        let span = self.span_consumed_since_parsing(atomic.or_parsing(name));
        let decl = self.add(LocalVariableDeclData { atomic, name, ty }, span);

        Some(decl)
    }
}
