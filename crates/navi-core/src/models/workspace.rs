use uuid::Uuid;

#[allow(dead_code)]
type WorkspaceId = Uuid;

#[derive(Debug)]
#[allow(dead_code)]
struct Workspace {
    workspace_id: WorkspaceId,
}
