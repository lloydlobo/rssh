use xshell::{cmd, Shell};

pub fn run(sh: &Shell) -> anyhow::Result<()> {
    cmd!(sh, "git add . ").run()?;

    Ok(())
}
