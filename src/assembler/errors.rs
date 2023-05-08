#[derive(Debug, Clone)]
pub enum AssemblerError {
    NoSegmentDeclarationFound,
    SymbolAlreadyDeclared,
    ParseError { error: String },
    IncorrectOperand,
}
