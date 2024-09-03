use oxc_allocator::Allocator;
use oxc_diagnostics::Result;

use crate::{
    ast::{AstroElement, AstroFrontMatter, AstroProgram},
    options::ParserOptions,
    span::SpanFactory,
};

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

    /// Main function to parse the Astro source code
    pub fn parse(self) -> Result<AstroProgram<'a>> {
        // TODO: this is a placeholder implementation
        let frontmatter = self.parse_frontmatter()?;
        let _body = self.parse_body()?;

        Ok(AstroProgram {
            span: self.span_factory.create(0, self.source_text.len()),
            frontmatter: frontmatter.map(|fm| oxc_allocator::Box::new_in(fm, self.allocator)),
        })
    }

    /// Function to parse the frontmatter of an Astro document
    fn parse_frontmatter(&self) -> Result<Option<AstroFrontMatter<'a>>> {
        // TODO: Implementation for parsing the frontmatter (aka component script)
        Ok(None) // Return None if no frontmatter is found
    }

    /// Function to parse the main body of an Astro document
    fn parse_body(&self) -> Result<Vec<AstroElement<'a>>> {
        // TODO: Implementation for parsing the main body
        Ok(Vec::new()) // Returning an empty Vec as a placeholder
    }
}

/// Function to check if the source text is a valid Astro document.
fn parse_astro_literal(_source_text: &str) -> Result<()> {
    // TODO: Implementation of Astro document validation
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_valid_astro_literal() {
        for literal_text in [
            "<html><body><h1>Hello, Astro!</h1></body></html>",
            "<div><Component /></div>",
            "<AstroComponent></AstroComponent>",
        ] {
            parse_astro_literal(literal_text)
                .unwrap_or_else(|_| panic!("{literal_text} should be parsed"));
        }
    }

    #[test]
    fn parse_invalid_astro_literal() {
        for literal_text in [
            // TODO: Add invalid Astro literals
            "<html><body><h1>Hello, Astro!</body></html>", // Missing closing tag
            "<div><Component /</div>",                     // Invalid self-closing tag
        ] {
            assert!(parse_astro_literal(literal_text).is_err());
        }
    }
}
