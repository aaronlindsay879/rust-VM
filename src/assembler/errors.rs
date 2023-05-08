#[derive(Debug, Clone)]
pub enum AssemblerError {
    NoSegmentDeclarationFound,
    StringConstantDeclaredWithoutLabel,
    SymbolAlreadyDeclared,
    UnknownDirectiveFound { directive: String },
    NonOpcodeInOpcodeField,
    InsufficientSections,
    ParseError { error: String },
    IncorrectOperand,
}
