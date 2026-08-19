use navi_core::models::repository::RepositoryBuilder;
use uuid::Uuid;

#[test]
fn creates_repository_with_generated_id() {
    let workspace_id = Uuid::new_v4();

    let repository = RepositoryBuilder::default()
        .set_workspace_id(workspace_id)
        .build();
    assert_eq!(repository.get_workspace_id(), workspace_id);
}
#[test]
fn preserves_supplied_repository_id() {
    let repository_id = Uuid::new_v4();
    let workspace_id = Uuid::new_v4();

    let repository = RepositoryBuilder::default()
        .set_repository_id(repository_id)
        .set_workspace_id(workspace_id)
        .build();

    assert_eq!(repository.get_repository_id(), repository_id);
    assert_eq!(repository.get_workspace_id(), workspace_id);
}

#[test]
fn generates_unique_repository_ids() {
    let workspace_id = Uuid::new_v4();

    let repository_1 = RepositoryBuilder::default()
        .set_workspace_id(workspace_id)
        .build();

    let repository_2 = RepositoryBuilder::default()
        .set_workspace_id(workspace_id)
        .build();

    assert_ne!(
        repository_1.get_repository_id(),
        repository_2.get_repository_id()
    );
}

#[test]
#[should_panic(expected = "workspace_id is required")]
fn requires_workspace_id() {
    RepositoryBuilder::default().build();
}
