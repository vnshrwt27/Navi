use navi_core::models::file::FileBuilder;
use uuid::Uuid;

#[test]
fn creates_repository_with_generated_id() {
    let repository_id = Uuid::new_v4();

    let file = FileBuilder::default()
        .set_repository_id(repository_id)
        .build();
    assert_eq!(file.get_repository_id(), repository_id);
}
#[test]
fn preserves_supplied_file_id() {
    let file_id = Uuid::new_v4();
    let repository_id = Uuid::new_v4();

    let file = FileBuilder::default()
        .set_file_id(file_id)
        .set_repository_id(repository_id)
        .build();

    assert_eq!(file.get_repository_id(), repository_id);
    assert_eq!(file.get_file_id(), file_id);
}

#[test]
fn generates_unique_file_ids() {
    let repository_id = Uuid::new_v4();

    let file_1 = FileBuilder::default()
        .set_repository_id(repository_id)
        .build();

    let file_2 = FileBuilder::default()
        .set_repository_id(repository_id)
        .build();

    assert_ne!(file_1.get_file_id(), file_2.get_file_id());
}

#[test]
#[should_panic(expected = "repository_id is required")]
fn requires_repository_id() {
    FileBuilder::default().build();
}
