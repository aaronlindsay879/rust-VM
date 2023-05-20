use crate::parser::directive::Directive;

#[derive(Debug, PartialEq)]
pub(super) enum AssemblerSection {
    Data,
    Code,
    Unknown,
}

impl Default for AssemblerSection {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<Directive> for AssemblerSection {
    fn from(value: Directive) -> Self {
        match value {
            Directive::Data => Self::Data,
            Directive::Code => Self::Code,
            _ => Self::Unknown,
        }
    }
}
