use navi_core::models::symbol::{SymbolBuilder, SymbolKind};
use uuid::Uuid;

#[test]
fn create_symbol_with_generated_id() {
    let file_id = Uuid::new_v4();

    let symbol = SymbolBuilder::default()
        .set_file_id(file_id)
        .set_symbol_kind(SymbolKind::Function)
        .build();

    assert_eq!(symbol.get_file_id(), file_id);
}

#[test]
fn generate_unique_symbol_ids() {
    let file_id = Uuid::new_v4();

    let symbol_1 = SymbolBuilder::default()
        .set_file_id(file_id)
        .set_symbol_kind(SymbolKind::Function)
        .build();

    let symbol_2 = SymbolBuilder::default()
        .set_file_id(file_id)
        .set_symbol_kind(SymbolKind::Function)
        .build();

    assert_ne!(symbol_1.get_symbol_id(), symbol_2.get_symbol_id());
}

#[test]
#[should_panic(expected = "file_id is required!")]
fn require_file_id() {
    SymbolBuilder::default()
        .set_symbol_kind(SymbolKind::Function)
        .build();
}

#[test]
#[should_panic(expected = "symbol_kind is required!")]
fn require_symbol_kind() {
    let file_id = Uuid::new_v4();

    SymbolBuilder::default().set_file_id(file_id).build();
}

#[test]
fn create_symbol_with_specific_kind() {
    let file_id = Uuid::new_v4();

    let symbol = SymbolBuilder::default()
        .set_file_id(file_id)
        .set_symbol_kind(SymbolKind::Struct)
        .build();

    assert!(matches!(symbol.get_symbol_kind(), SymbolKind::Struct));
}
