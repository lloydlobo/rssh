//! `rssh` or Rust Shell is a tool to implement maintainble bash aliases from within Rust.
//!
//! Code forked & utilized from [matlkad/config/xtool](https://github.com/matklad/config/tree/master/xtool)

use std::{env, path::PathBuf};

use anyhow::{anyhow, Context};
use rssh::{build_link, COMMANDS};
use xshell::Shell;

// NOTE:
// Looks like linking build_link works when in the projects directory.
// * fixed: `let src = &"./target/release/rssh";`
// * with: `let src = &"/home/lloyd/.cargo/bin/rssh";`

fn main() -> anyhow::Result<()> {
    if let true = env::args().into_iter().find(|a| a == "rssh").is_some() {
        build_link().context(anyhow!("Called `rssh` to symlink COMMANDS with rssh binary"))?;
        std::process::exit(0)
    }

    let prog_arg_name: PathBuf = env::args_os().next().unwrap_or_default().into();
    let prog_arg_name = prog_arg_name.file_stem().unwrap_or_default().to_str().unwrap_or_default();

    let (cmd_name, run) = COMMANDS
        .iter()
        .find(|&&(name, _run)| name == prog_arg_name)
        .ok_or_else(|| anyhow::format_err!("unknown command: `{prog_arg_name}`"))?;

    let sh = Shell::new()?;
    run(&sh).map_err(|err| anyhow!("Failed to run {}: {:#?}", cmd_name, err))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use rssh::{walkdir, COMMANDS};
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
