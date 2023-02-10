//! `rssh` or Rust Shell is a tool to implement maintainble bash aliases from within Rust.
//!
//! Code forked & utilized from [matlkad/config/xtool](https://github.com/matklad/config/tree/master/xtool)

use std::{env, path::PathBuf};

use anyhow::{anyhow, Context};
use rssh::{build_link, COMMANDS};
use xshell::Shell;

fn main() -> anyhow::Result<()> {
    if let true = env::args().any(|a| &a == "rssh") {
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
