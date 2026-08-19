use uuid::Uuid;

#[allow(dead_code)]
type WorkspaceId = Uuid;

#[derive(Debug)]
pub struct Workspace {
    workspace_id: WorkspaceId,
}

impl Workspace {
    pub fn get_id(&self) -> WorkspaceId {
        self.workspace_id
    }
}

#[derive(Default)]
pub struct WorkspaceBuilder {
    workspace_id: Option<WorkspaceId>,
}

impl WorkspaceBuilder {
    pub fn set_workspace_id(mut self, workspace_id: WorkspaceId) -> Self {
        self.workspace_id = Some(workspace_id);
        self
    }

    pub fn set_workspace_abc(mut self, workspace_id: WorkspaceId) -> Self {
        self.workspace_id = Some(workspace_id);
        self
    }
    pub fn build(self) -> Workspace {
        Workspace {
            workspace_id: self.workspace_id.unwrap_or_else(Uuid::new_v4),
        }
    }
}
