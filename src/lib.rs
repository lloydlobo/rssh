pub mod cmd;

use std::{
    fs::{self, DirEntry, FileType},
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use xshell::{cmd, Shell};

pub use crate::cmd::*;

pub const COMMANDS: &[(&str, fn(&Shell) -> anyhow::Result<()>)] = &[
    // ("ammend", ammend::run),
    // ("autostart", autostart::run),
    ("commit", commit::run),
    ("go", go::run),
    // ("script", script::run),
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
        sh.hard_link(&src, &dst).map_err(|err| anyhow!("{:#?}", err)).unwrap();
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
