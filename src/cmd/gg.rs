//! GitUi - Git GUI `gg` alias

use xshell::{cmd, Shell};

/// # Usage
///
/// * `optional args` - Arguments passed to `gitui`.
///
/// ```sh
/// $ gg
/// ```
///
/// # Errors
///
/// This function will return an error if the `cmd!` fails to execute.
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
