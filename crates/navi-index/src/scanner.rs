use std::path::{Path,PathBuf};
#[derive(Debug)]
pub struct ScannedFile {
    pub path : PathBuf,
    pub language: Language
}

#[derive(Debug, PartialEq, Eq)]
pub enum Language {
    Python,
 //   Rust,
//  Javascript,
    Unknown,
}

pub struct Scanner;

impl Scanner{
    pub fn scan(root: &Path)-> Vec<ScannedFile>{
        // scans the project root directory where the tool is initialized 
        
        let mut files = Vec::new();

        Self::walk(root, &mut files);
        files 
    }

    fn walk(path: &Path, files : &mut Vec<ScannedFile>){
        // walk the directory
        let Ok(entries) =  std::fs::read_dir(path) else {
            return ;
        };

         for entry in entries{
             let Ok(entry) = entry else {
                 // skip any files that give error 
                 continue;
             };

             let path = entry.path();

            // recursively scan sub directories 
            if path.is_dir(){
                Self::walk(&path, files);
                continue;
            }

            // skip everything that isn't a regular file 
            if !path.is_file(){
                continue;
            }

            let language = Self::detect_language(&path);
            files.push(ScannedFile{
                path,
                language});
        }
    }

    fn detect_language(path: &Path)-> Language{
        //detect Language by matching the file type to the language 
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("py") => Language::Python,
            // Some("rs") => Language::Rust,
            // Some("js") => Language::Javascript,
            _=> Language::Unknown,
        }

    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn python_file_detection() {
        // PYTHON test 
        let path = Path::new("main.py");
        assert_eq!(
                Scanner::detect_language(path),
                Language::Python
            );
}

    #[test]
    fn unknown_file_detection() {
        let path = Path::new("Readme.md");
        // UNKNOWN test
        assert_eq!(
            Scanner::detect_language(path),
            Language::Unknown
            );
    }
}
