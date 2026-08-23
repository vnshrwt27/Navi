use crate::models::file::FileId;
use uuid::Uuid;

pub type SymbolId = Uuid;

#[derive(Debug)]
pub struct Symbol {
    symbol_id: SymbolId,
    file_id: FileId,
}

impl Symbol {
    pub fn get_symbol_id(&self) -> SymbolId {
        self.symbol_id
    }

    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }
}

#[derive(Default)]
pub struct SymbolBuilder {
    symbol_id: Option<SymbolId>,
    file_id: Option<FileId>,
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

    pub fn build(self) -> Symbol {
        Symbol {
            symbol_id: self.symbol_id.unwrap_or_else(Uuid::new_v4),
            file_id: self.file_id.expect("file_id is required!"),
        }
    }
}
