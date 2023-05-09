#[derive(thiserror::Error, Debug, Clone)]
pub enum AssemblerError {
    #[error("instruction/directive not in a segment")]
    NoSegmentDeclarationFound,
    #[error("symbol already declared")]
    SymbolAlreadyDeclared,
    #[error("failed to parse: {error}")]
    ParseError { error: String },
    #[error("incorrect operand for instruction/directive")]
    IncorrectOperand,
}
