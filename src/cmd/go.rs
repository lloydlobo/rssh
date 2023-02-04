use xshell::{cmd, Shell};

/// # Usage
///
/// * `required dest` - Destination to "go to".
///
/// ```bash
/// $ go notes
/// ```
///
/// # Available `dest`
///
/// * `notes` - `$EDITOR notes.md`
/// * `tmp` - `$EDITOR tmp.md`
///
/// # Errors
///
/// This function will return an error if the `dest` flag is an unkown `dest` i.e. not hardcoded in
/// the set of available `dest` destinations.
pub fn run(sh: &Shell) -> anyhow::Result<()> {
    let flags = xflags::parse_or_exit! {required dest: String};
    match flags.dest.as_str() {
        // nvim - $EDITOR
        "notes" => cmd!(sh, "nvim /home/lloyd/p/notes.md").run()?,
        "tmp" => cmd!(sh, "nvim /home/lloyd/p/tmp.md").run()?,

        // Unknown
        dest => anyhow::bail!("unknown destination: `{}`", dest),
    }

    Ok(())
}
