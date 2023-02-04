//! GitUi - Git GUI `gg` alias

use xshell::{cmd, Shell};

pub fn run(sh: &Shell) -> anyhow::Result<()> {
    let flags = xflags::parse_or_exit! {
        /// Arguments passed to gitui.
        optional args: String
    };

    match flags.args {
        Some(arg) => {
            cmd!(sh, "gitui {arg}").run()?;
        }
        None => {
            cmd!(sh, "gitui").run()?;
        }
    }

    Ok(())
}
