use oxc_allocator::Allocator;
use oxc_diagnostics::{OxcDiagnostic, Result};

use crate::{ast, span::SpanFactory};

pub struct AstroParser<'a> {
    allocator: &'a Allocator,
    source_text: &'a str,
    options: ParserOptions,
    span_factory: SpanFactory,
}

impl<'a> AstroParser<'a> {
    pub fn new(allocator: &'a Allocator, source_text: &'a str, options: ParserOptions) -> Self {
        Self {
            allocator,
            source_text,
            options,
            span_factory: SpanFactory::new(options.span_offset),
        }
    }

    pub fn parse(self) -> Result<ast::AstroProgram<'a>> {
        Ok(ast::AstroProgram {
            span: self.span_factory.create(0, self.source_text.len()),
            frontmatter: None,
        })
    }
}
