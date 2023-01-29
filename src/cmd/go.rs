use xshell::{cmd, Shell};

pub fn run(sh: &Shell) -> anyhow::Result<()> {
    let flags = xflags::parse_or_exit! {required dest: String};
    match flags.dest.as_str() {
        "tmp" => cmd!(sh, "nvim /home/lloyd/Documents/tmp.md").run()?,
        "notes" => cmd!(sh, "nvim /home/lloyd/Documents/02-areas/notes.md").run()?,
        dest => anyhow::bail!("unknown destination: `{}`", dest),
    }

    Ok(())
}
