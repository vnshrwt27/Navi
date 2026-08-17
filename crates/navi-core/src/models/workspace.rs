use uuid::Uuid;

type WorkspaceId = Uuid;

#[derive(Debug)]
struct Workspace {
    workspace_id: WorkspaceId,
}
