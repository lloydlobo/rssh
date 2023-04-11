use xshell::{cmd, Shell};

// # Source: Command Line Cheat Sheets by Elijah Manor.
// alias tldrf='tldr --list | fzf --preview "tldr {1} --color=always" \
//    --preview-window=right,70% | xargs tldr'
pub fn run(sh: &Shell) -> anyhow::Result<()> {
    let tldr_output = Some(cmd!(sh, "tldr --list").read()?);
    #[rustfmt::skip]
    let selected_command = Some(cmd!(
        sh, 
        "echo {tldr_output...} | fzf --preview 'tldr {{1}} --color=always' --preview-window=right,70%" 
    )
    .read()?);
    cmd!(sh, "echo {selected_command...} | xargs tldr").run()?;

    Ok(())
}
