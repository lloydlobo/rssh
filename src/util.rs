use std::{
    fs::{
        self,
        DirEntry,
        FileType,
    },
    path::PathBuf,
};

pub fn walkdir(path: PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    let mut res = Vec::new();
    let mut work = vec![path];

    while let Some(dir) = work.pop() {
        for entry in fs::read_dir(&dir)? {
            let entry: DirEntry = entry?;
            let file_type: FileType = entry.file_type()?;

            if file_type.is_file() {
                res.push(entry.path())
            } else if file_type.is_dir() {
                work.push(entry.path())
            }
        }
    }

    Ok(res)
}
