use navi_core::models::file::FileBuilder;
use uuid::Uuid;

#[test]
fn creates_file_with_generated_id() {
    let directory_id = Uuid::new_v4();

    let file = FileBuilder::default()
        .set_directory_id(directory_id)
        .build();
    assert_eq!(file.get_directory_id(), directory_id);
}
#[test]
fn preserves_supplied_file_id() {
    let file_id = Uuid::new_v4();
    let directory_id = Uuid::new_v4();

    let file = FileBuilder::default()
        .set_file_id(file_id)
        .set_directory_id(directory_id)
        .build();

    assert_eq!(file.get_directory_id(), directory_id);
    assert_eq!(file.get_file_id(), file_id);
}

#[test]
fn generates_unique_file_ids() {
    let directory_id = Uuid::new_v4();

    let file_1 = FileBuilder::default()
        .set_directory_id(directory_id)
        .build();

    let file_2 = FileBuilder::default()
        .set_directory_id(directory_id)
        .build();

    assert_ne!(file_1.get_file_id(), file_2.get_file_id());
}

#[test]
#[should_panic(expected = "directory_id is required")]
fn required_directory_id() {
    FileBuilder::default().build();
}
