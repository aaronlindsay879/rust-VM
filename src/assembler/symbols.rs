use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    /// Adds a symbol, returning a bool indicating if it's a new symbol with that name
    pub fn add_symbol(&mut self, name: &str, symbol: Symbol) -> bool {
        self.symbols.insert(name.to_string(), symbol).is_none()
    }

    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}

#[derive(Debug, PartialEq)]
pub struct Symbol {
    /// Offset from start of data section (in terms of bytes)
    pub offset: u32,
    symbol_type: SymbolType,
}

impl Symbol {
    pub fn new(offset: u32, symbol_type: SymbolType) -> Self {
        Self {
            offset,
            symbol_type,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SymbolType {
    Label,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table() {
        let mut sym = SymbolTable::default();
        let new_symbol = Symbol::new(12, SymbolType::Label);
        sym.add_symbol("test", new_symbol);
        assert_eq!(sym.symbols.len(), 1);

        let v = sym.get_symbol("test");
        assert!(v.is_some());

        let v = v.unwrap();
        assert_eq!(*v, Symbol::new(12, SymbolType::Label));

        let v = sym.get_symbol("does_not_exist");
        assert!(v.is_none());
    }
}
