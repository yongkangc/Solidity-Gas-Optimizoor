use crate::Lexer;

pub struct Parser<'ast> {
    arena: &'ast Arena,

    /// Lexer will produce tokens from the source
    lexer: Lexer<NulTermStr<'ast>>,

    /// Errors occurred during parsing
    errors: Vec<Error>,

    /// AST under construction
    body: SourceUnitList<'ast>,
}

/// Parse the Solidity source from `&str` and produce an Abstract Syntax Tree for it.
pub fn parse<'src, 'ast>(source: &'src str) -> Result<Program<'ast>, Vec<Error>> {
    let arena = Arena::new();

    let (body, errors) = {
        let mut parser = Parser::new(source, &arena);

        parser.parse();

        (parser.body.into_unsafe(), parser.errors)
    };

    match errors.len() {
        0 => Ok(Program::new(body, arena)),
        _ => Err(errors),
    }
}
