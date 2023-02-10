pub mod cmd;

use std::{
    fs::{self, DirEntry, FileType},
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use xshell::{cmd, Shell};

pub use crate::cmd::*;

pub const COMMANDS: &[(&str, fn(&Shell) -> anyhow::Result<()>)] = &[
    ("amend", amend::run),   // git commit --amend --no-edit
    ("commit", commit::run), // git commit
    ("cron", cron::run),     // cron reminders
    ("gg", gg::run),         // gitui
    ("go", go::run),         // go to
];

pub fn build_link() -> anyhow::Result<()> {
    println!("Welcome to rssh link setup");

    let sh = Shell::new()?;
    let bin = Path::new("/home/lloyd/bin");
    // let src = &"./target/release/rssh";
    let src = &"/home/lloyd/.cargo/bin/rssh";

    for &(cmd, _) in COMMANDS {
        let dst: PathBuf = bin.join(cmd);
        sh.remove_path(&dst).unwrap();

        let _ = cmd!(sh, "git rm {dst} -f").ignore_stderr().quiet().run();
        sh.hard_link(src, &dst).map_err(|err| anyhow!("{:#?}", err)).unwrap();
        println!("=> Hardlinked: src: {} to dst: {:#?}", &src, &dst);
    }

    let home: PathBuf = "/home/lloyd/".into();
    let config_home: PathBuf = home.join("config/home");
    for abs_path in walkdir(config_home.clone()).unwrap() {
        let rel_path: &Path = abs_path.strip_prefix(&config_home).unwrap();
        let dest: PathBuf = home.join(rel_path);
        sh.remove_path(&dest).unwrap();
        sh.create_dir(dest.parent().unwrap()).unwrap();

        std::os::unix::fs::symlink(&abs_path, &dest).unwrap();
        println!("=> Symbolic linked: original: {:#?} to link: {:#?}", &abs_path, &dest);
    }

    Ok(())
}

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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Context;
    use xshell::{cmd, Shell};

    use super::*;

    #[test]
    fn link_up() {
        let sh = Shell::new().unwrap();
        let bin = std::path::Path::new("/home/lloyd/bin");
        cmd!(sh, "cargo build --release").run().unwrap();

        for &(cmd, _) in COMMANDS {
            let dst: PathBuf = bin.join(cmd);
            sh.remove_path(&dst).unwrap();
            let _ = cmd!(sh, "git rm {dst} -f").ignore_stderr().quiet().run();
            sh.hard_link("./target/release/rssh", &dst).unwrap();
        }

        let home: PathBuf = "/home/lloyd/".into();
        let config_home: PathBuf = home.join("config/home");
        for abs_path in walkdir(config_home.clone()).unwrap() {
            let rel_path: &Path = abs_path.strip_prefix(&config_home).unwrap();
            let dest: PathBuf = home.join(rel_path);
            sh.remove_path(&dest).unwrap(); // Removes the file or directory at the given path.
            sh.create_dir(dest.parent().unwrap()).unwrap(); // Creates the specified directory.

            // The `link` path will be a symbolic link pointing to the `original` path.
            std::os::unix::fs::symlink(abs_path, dest).unwrap();
        }
    }

    #[test]
    fn it_rssh_file_stem() {
        assert_eq!(
            "rssh",
            Path::new("rssh.rs")
                .file_stem()
                .context(anyhow!(
                    "Should extract the stem (non-extension) portion of [`self.file_name`]."
                ))
                .unwrap()
        );
    }
}
