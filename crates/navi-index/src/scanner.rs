use std::path::{Path,PathBuf};
#[derive(Debug)]
pub struct ScannedFile {
    pub path : PathBuf,
    pub language: Language
}

#[derive(Debug)]
pub enum Language {
    Python,
    Unknown,
}

pub struct Scanner;

impl Scanner{
    pub fn scan(root: &Path)-> Vec<ScannedFile>{
        //scans the root directory
        let mut files = Vec::new();

        Self::walk(root, &mut files);
        files 
    }

    fn walk(path: &Path, files : &mut Vec<ScannedFile>){
        // walk the directory
        let Ok(entries) =  std::fs::read_dir(path) else {
            return ;
        };
        // Add each file to the files list 
        // recusively call walk function for all directories

         for entry in entries.flatten(){
            let path = entry.path();
            
            if path.is_dir(){
                Self::walk(&path, files);
                continue;
            }
            if path.is_file(){
                continue;
            }

            let language = Self::detect_language(&path);
            files.push(ScannedFile{path,language});
        }
    }

    fn detect_language(path: &Path)-> Language{
        //detect Language by matchin the file type to the language 
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("py") => Language::Python,
            _=> Language::Unknown,
        }

    }
}
