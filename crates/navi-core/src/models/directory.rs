use uuid::Uuid;

use crate::models::repository::RepositoryId;

pub type DirectoryId = Uuid;

#[derive(Debug)]
pub struct Directory {
    directory_id: DirectoryId,
    repository_id: RepositoryId,
}

impl Directory {
    pub fn get_directory_id(&self) -> DirectoryId {
        self.directory_id
    }

    pub fn get_repository_id(&self) -> RepositoryId {
        self.repository_id
    }
}

#[derive(Default)]
pub struct DirectoryBuilder {
    directory_id: Option<DirectoryId>,
    repository_id: Option<RepositoryId>,
}

impl DirectoryBuilder {
    pub fn set_directory_id(mut self, directory_id: DirectoryId) -> Self {
        self.directory_id = Some(directory_id);
        self
    }

    pub fn set_repository_id(mut self, repository_id: RepositoryId) -> Self {
        self.repository_id = Some(repository_id);
        self
    }
    pub fn build(self) -> Directory {
        Directory {
            directory_id: self.directory_id.unwrap_or_else(Uuid::new_v4),
            repository_id: self.repository_id.expect("repository_id is required"),
        }
    }
}
