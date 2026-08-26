use crate::models::file::FileId;
use uuid::Uuid;

pub type SymbolId = Uuid;

// Expand the enum later
#[derive(Debug, Copy, Clone)]
pub enum SymbolKind {
    Function,
    Method,
    Struct,
    Enum,
    Trait,
    Class,
    Interface,
    Variable,
    Constant,
    Module,
    Type,
}
#[derive(Debug)]
pub struct Symbol {
    symbol_id: SymbolId,
    file_id: FileId,
    symbol_kind: SymbolKind,
}

impl Symbol {
    pub fn get_symbol_id(&self) -> SymbolId {
        self.symbol_id
    }

    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }

    pub fn get_symbol_kind(&self) -> SymbolKind {
        self.symbol_kind
    }
}

#[derive(Default)]
pub struct SymbolBuilder {
    symbol_id: Option<SymbolId>,
    file_id: Option<FileId>,
    symbol_kind: Option<SymbolKind>,
}

impl SymbolBuilder {
    pub fn set_symbol_id(mut self, symbol_id: SymbolId) -> Self {
        self.symbol_id = Some(symbol_id);
        self
    }
    pub fn set_file_id(mut self, file_id: FileId) -> Self {
        self.file_id = Some(file_id);
        self
    }
    pub fn set_symbol_kind(mut self, symbol_kind: SymbolKind) -> Self {
        self.symbol_kind = Some(symbol_kind);
        self
    }

    pub fn build(self) -> Symbol {
        Symbol {
            symbol_id: self.symbol_id.unwrap_or_else(Uuid::new_v4),
            file_id: self.file_id.expect("file_id is required!"),
            symbol_kind: self.symbol_kind.expect("symbol_kind is required!"),
        }
    }
}
