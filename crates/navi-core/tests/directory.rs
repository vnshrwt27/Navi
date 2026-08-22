use navi_core::models::directory::DirectoryBuilder;
use uuid::Uuid;

#[test]
fn creates_directory_with_generated_id() {
    let repository_id = Uuid::new_v4();

    let directory = DirectoryBuilder::default()
        .set_repository_id(repository_id)
        .build();
    assert_eq!(directory.get_repository_id(), repository_id);
}
#[test]
fn preserves_supplied_file_id() {
    let repository_id = Uuid::new_v4();
    let directory_id = Uuid::new_v4();

    let directory = DirectoryBuilder::default()
        .set_repository_id(repository_id)
        .set_directory_id(directory_id)
        .build();

    assert_eq!(directory.get_repository_id(), repository_id);
    assert_eq!(directory.get_directory_id(), directory_id);
}

#[test]
fn generates_unique_directory_ids() {
    let repository_id = Uuid::new_v4();

    let directory_1 = DirectoryBuilder::default()
        .set_repository_id(repository_id)
        .build();

    let directory_2 = DirectoryBuilder::default()
        .set_repository_id(repository_id)
        .build();

    assert_ne!(
        directory_1.get_directory_id(),
        directory_2.get_directory_id()
    );
}

#[test]
#[should_panic(expected = "repository_id is required")]
fn required_repository_id() {
    DirectoryBuilder::default().build();
}
