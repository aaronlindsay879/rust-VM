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

impl From<&str> for AssemblerSection {
    fn from(value: &str) -> Self {
        match value {
            "data" => Self::Data,
            "code" => Self::Code,
            _ => Self::Unknown,
        }
    }
}
