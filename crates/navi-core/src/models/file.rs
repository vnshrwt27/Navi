use uuid::Uuid;

pub type FileId = Uuid;
pub type RepositoryId = Uuid;

#[derive(Debug)]
pub struct File {
    file_id: FileId,
    repository_id: RepositoryId,
}

impl File {
    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }

    pub fn get_repository_id(&self) -> RepositoryId {
        self.repository_id
    }
}

#[derive(Default)]
pub struct FileBuilder {
    file_id: Option<FileId>,
    repository_id: Option<RepositoryId>,
}

impl FileBuilder {
    pub fn set_file_id(mut self, file_id: FileId) -> Self {
        self.file_id = Some(file_id);
        self
    }

    pub fn set_repository_id(mut self, repository_id: RepositoryId) -> Self {
        self.repository_id = Some(repository_id);
        self
    }
    pub fn build(self) -> File {
        File {
            file_id: self.file_id.unwrap_or_else(Uuid::new_v4),
            repository_id: self.repository_id.expect("repository_id is required"),
        }
    }
}
