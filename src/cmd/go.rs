use xshell::{cmd, Shell};

// * `xshell::Cmd::run()` - By default the command itself is echoed to stderr, its standard streams
//   are inherited, and non-zero return code is considered an error. These behaviors can be
//   overridden by using various builder methods of the [`Cmd`].
pub fn run(sh: &Shell) -> anyhow::Result<()> {
    // println!("current_dir: {}", std::env::current_dir().unwrap().display());
    let flags = xflags::parse_or_exit! {required dest: String};
    match flags.dest.as_str() {
        // nvim - $EDITOR
        "tmp" => cmd!(sh, "nvim /home/lloyd/Documents/tmp.md").run()?,
        "notes" => cmd!(sh, "nvim /home/lloyd/Documents/02-areas/notes.md").run()?,

        // Unknown
        dest => anyhow::bail!("unknown destination: `{}`", dest),
    }

    Ok(())
}
