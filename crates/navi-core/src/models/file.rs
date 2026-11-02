use uuid::Uuid;

use crate::models::directory::DirectoryId;

pub type FileId = Uuid;

#[derive(Debug)]
pub enum Language {
    Python,
    Rust,
    Javascript,
    Go,
    Java,
    C
}
#[derive(Debug)]
pub struct File {
    file_id: FileId,
    directory_id: DirectoryId,
}

impl File {
    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }

    pub fn get_directory_id(&self) -> DirectoryId {
        self.directory_id
    }
}

#[derive(Default)]
pub struct FileBuilder {
    file_id: Option<FileId>,
    directory_id: Option<DirectoryId>,
}

impl FileBuilder {
    pub fn set_file_id(mut self, file_id: FileId) -> Self {
        self.file_id = Some(file_id);
        self
    }

    pub fn set_directory_id(mut self, directory_id: DirectoryId) -> Self {
        self.directory_id = Some(directory_id);
        self
    }

    pub fn build(self) -> File {
        File {
            file_id: self.file_id.unwrap_or_else(Uuid::new_v4),
            directory_id: self.directory_id.expect("directory_id is required"),
        }
    }
}
