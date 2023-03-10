use xshell::{cmd, Shell};

/// Add all changes and commit with message or "."
///
/// # Usage
///
/// * `optional message` - Commit message.
///
/// ```bash
/// $ commit 'fix: fix something'
/// ```
///
/// * `optional -b,--branch branch` - Move all changes out of the way as a commit onto a new branch.
///
/// ```bash
/// $ commit -b feat_new_branch 'feat: add some new feature'
/// ```
pub fn run(sh: &Shell) -> anyhow::Result<()> {
    let flags = xflags::parse_or_exit! {
        /// Commit message.
        optional message: String
        /// Move all changes out of the way as a commit onto a new branch.
        optional -b,--branch branch: String
    };
    let message: &str = flags.message.as_deref().unwrap_or(".");

    cmd!(sh, "git add --all").run()?;
    cmd!(sh, "git --no-pager diff --cached --color=always").run()?;

    match flags.branch {
        Some(branch) => {
            cmd!(sh, "git switch -c {branch}").run()?;
            cmd!(sh, "git commit -m {message}").run()?;
            cmd!(sh, "git switch -").run()?;
        }
        None => cmd!(sh, "git commit -m {message}").run()?,
    }

    Ok(())
}
