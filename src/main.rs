//! `rssh` or Rust Shell is a tool to implement maintainble bash aliases from within Rust.
//!
//! Code forked & utilized from [matlkad/config/xtool](https://github.com/matklad/config/tree/master/xtool)

use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context};
use rssh::{walkdir, COMMANDS};
use xshell::Shell;

fn main() -> anyhow::Result<()> {
    if let true = env::args().into_iter().find(|a| a == "rssh").is_some() {
        build_link().context(anyhow!("Called `rssh` to symlink COMMANDS with rssh binary"))?;
        std::process::exit(0)
    }

    let mut args_os = env::args_os();
    let prog_arg_name: PathBuf = args_os.next().unwrap_or_default().into();
    let prog_arg_name = prog_arg_name.file_stem().unwrap_or_default().to_str().unwrap_or_default();
    let (cmd_name, run) = COMMANDS
        .iter()
        .find(|&&(name, _run)| name == prog_arg_name)
        .ok_or_else(|| anyhow::format_err!("unknown command: `{prog_arg_name}`"))?;
    let sh = Shell::new()?;
    run(&sh).map_err(|err| anyhow!("Failed to run {}: {:#?}", cmd_name, err))
}

fn build_link() -> anyhow::Result<()> {
    println!("Welcome to rssh link setup");
    let sh = Shell::new()?;
    let bin = Path::new("/home/lloyd/bin");
    let src = &"./target/release/rssh";
    for &(cmd, _) in COMMANDS {
        let dst: PathBuf = bin.join(cmd);
        sh.remove_path(&dst).unwrap();
        let _ = xshell::cmd!(sh, "git rm {dst} -f").ignore_stderr().quiet().run();
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use rssh::{walkdir, COMMANDS};
    use xshell::{cmd, Shell};

    use super::*;

    /// Extracts the stem (non-extension) portion of [`self.file_name`].
    #[test]
    fn it_file_stem() {
        assert_eq!("foo", Path::new("foo.rs").file_stem().unwrap());
        assert_eq!("foo.tar", Path::new("foo.tar.gz").file_stem().unwrap());
    }
    #[test]
    fn it_rssh_file_stem() {
        assert_eq!("rssh", Path::new("rssh.rs").file_stem().unwrap());
    }

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
            // Removes the file or directory at the given path.
            sh.remove_path(&dest).unwrap();
            // Creates the specified directory.
            sh.create_dir(dest.parent().unwrap()).unwrap();
            // Creates a new symbolic link on the filesystem. The `link` path will be a symbolic
            // link pointing to the `original` path.
            std::os::unix::fs::symlink(abs_path, dest).unwrap();
        }
    }
}
