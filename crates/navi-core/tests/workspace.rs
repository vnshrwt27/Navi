use navi_core::models::workspace::WorkspaceBuilder;
use uuid::Uuid;

#[test]
fn build_generates_workspace_id_when_not_provided() {
    let workspace = WorkspaceBuilder::default().build();

    assert_ne!(workspace.get_id(), Uuid::nil());
}

#[test]
fn build_uses_provided_workspace_id() {
    let id = Uuid::new_v4();

    let workspace = WorkspaceBuilder::default()
        .set_workspace_id(id)
        .build();

    assert_eq!(workspace.get_id(), id);
}

#[test]
fn set_workspace_abc_sets_workspace_id() {
    let id = Uuid::new_v4();

    let workspace = WorkspaceBuilder::default()
        .set_workspace_abc(id)
        .build();

    assert_eq!(workspace.get_id(), id);
}
