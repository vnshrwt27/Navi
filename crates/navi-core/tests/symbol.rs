use navi_core::models::symbol::SymbolBuilder;
use uuid::Uuid;

#[test]
fn create_symbols_with_generated_id() {
    let file_id = Uuid::new_v4();
    let symbol = SymbolBuilder::default().set_file_id(file_id).build();
    assert_eq!(symbol.get_file_id(), file_id);
}

#[test]
fn generate_unique_symbol_ids() {
    let file_id = Uuid::new_v4();

    let symbol_1 = SymbolBuilder::default().set_file_id(file_id).build();

    let symbol_2 = SymbolBuilder::default().set_file_id(file_id).build();
    assert_ne!(symbol_1.get_symbol_id(), symbol_2.get_symbol_id());
}

#[test]
#[should_panic(expected = "file_id is required!")]
fn require_file_id() {
    SymbolBuilder::default().build();
}
