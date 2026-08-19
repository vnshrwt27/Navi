use uuid::Uuid;

type RepositoryId = Uuid;
type WorkspaceId = Uuid;

#[derive(Debug)]
pub struct Repository {
    repository_id: RepositoryId,
    workspace_id: WorkspaceId,
}

impl Repository {
    pub fn get_workspace_id(&self) -> WorkspaceId {
        self.workspace_id
    }
    pub fn get_repository_id(&self) -> RepositoryId {
        self.repository_id
    }
}

#[derive(Default)]
pub struct RepositoryBuilder {
    repository_id: Option<RepositoryId>,
    workspace_id: Option<WorkspaceId>,
}

impl RepositoryBuilder {
    pub fn set_repository_id(mut self, repository_id: RepositoryId) -> Self {
        self.repository_id = Some(repository_id);
        self
    }
    pub fn set_workspace_id(mut self, workspace_id: WorkspaceId) -> Self {
        self.workspace_id = Some(workspace_id);
        self
    }
    pub fn build(self) -> Repository {
        Repository {
            repository_id: self.repository_id.unwrap_or_else(Uuid::new_v4),
            workspace_id: self.workspace_id.expect("workspace_id is required"),
        }
    }
}
