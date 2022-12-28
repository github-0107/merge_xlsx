use std::{fs, path::PathBuf, ffi::OsStr};

#[derive(Debug)]
pub struct FileList(Vec<PathBuf>);

impl FileList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn list(&mut self, target: PathBuf, filter: &str) -> std::io::Result<()> {
        let read_dir = fs::read_dir(target)?;
        for entry in read_dir {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_dir() {
                self.list(entry.path(), filter)?;
                continue;
            }
            if entry.path().extension() == Some(OsStr::new(filter)) {
                self.0.push(entry.path());
            }
        }
        Ok(())
    }

    pub fn get_files(&self) -> &[PathBuf] {
        &self.0
    }
}

#[cfg(test)]
mod tests {

    use std::path::Path;

    use crate::FileList;

    #[test]
    fn test_list() {
        let mut file_list = FileList::new();
        file_list.list(Path::new("/home/arch-0107/文档").to_path_buf(), "xlsx").unwrap();
        println!("{:?}", file_list);
    }
}
