use xshell::{cmd, Shell};

/// Amend a commit without changing commit message.
///
/// # Errors
///
/// This function will return an error if `cmd!` fails to run.
pub fn run(sh: &Shell) -> anyhow::Result<()> {
    cmd!(sh, "git add . ").run()?;
    cmd!(sh, "git --no-pager diff --cached --color=always").run()?;
    cmd!(sh, "git --no-pager show -s --oneline --abbrev-commit --color=always HEAD").run()?;

    if is_yes_or_no_prompt("Continue") {
        cmd!(sh, "git commit --amend --no-edit").run()?;
    }

    Ok(())
}

/// Returns a `bool` indicating if the user decided to continue when prompted.
/// Locks this handle and reads a line of input, appending it to the specified buffer.
fn is_yes_or_no_prompt(arg_msg: &str) -> bool {
    println!("{arg_msg}");
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
        Err(_) | Ok(0) => false,
        Ok(_) => {
            let response: &str = buf.trim();
            matches!(response, "" | "y" | "Y")
        }
    }
}
