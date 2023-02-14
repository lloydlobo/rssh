// PROJECT: ❯ fd . 'scripts/' | entr -c -s "ls"

use xshell::{cmd, Shell};

/// Watch a directory for any file changes and run `command` passed.
pub fn run(sh: &Shell) -> anyhow::Result<()> {
    let flags = xflags::parse_or_exit! {
        required command: String // The utility or command to run.
        optional -f,--flags flags: String
    };

    let command: String = flags.command;
    let opts: &str = flags.flags.as_deref().unwrap_or("-c");
    cmd!(sh, "funzzy watch {opts} {command}").run()?;

    Ok(())
}

// PERF: See if `fd` is awailable in the system, else use ls.
//     See `xtask`
// PERF: Use tokei to find the top two file type extensions in dir. and watch those if precision is
// required.
//
// $ fd ~/rssh | entr -a ls
// $ funzzy -c 'cargo r'
