use xshell::{cmd, Shell};

/// Run a command using `tldr`, an alternative to `man` pages, by selecting the command from a
/// searchable list.
///
/// # Usage
///
/// ```bash
/// $ tldrf
/// ```
///
/// This function allows the user to search for a command using the `tldr` command-line tool. It
/// provides a more user-friendly alternative to `man` pages by providing simplified, practical
/// examples of how to use common Unix commands.
///
/// The function first retrieves a list of available commands using the `tldr --list` command. It
/// then presents the user with a searchable list of commands using the `fzf` tool. Once the user
/// selects a command from the list, the function retrieves its documentation using the `tldr`
/// command and displays it to the user.
///
/// The function uses `echo` and `xargs` to pass the selected command from `fzf` to `tldr`. It also
/// uses the `--preview` and `--preview-window` options of `fzf` to display a preview of the
/// selected command's documentation.
///
/// # Shell alias
///
/// Source: Command Line Cheat Sheets by Elijah Manor.
/// ```bash
/// alias tldrf='tldr --list | fzf --preview "tldr {1} --color=always" \
///    --preview-window=right,70% | xargs tldr'
/// ```
pub fn run(sh: &Shell) -> anyhow::Result<()> {
    // Get a list of available commands.
    let tldr_output = Some(cmd!(sh, "tldr --list").read()?);

    // Select a command using fzf.
    #[rustfmt::skip]
    let selected_command = Some(cmd!(
        sh, 
        "echo {tldr_output...} | fzf --preview 'tldr {{1}} --color=always' --preview-window=right,70%" 
    )
    .read()?);

    // Retrieve and display the selected command's documentation using tldr.
    cmd!(sh, "echo {selected_command...} | xargs tldr").run()?;

    Ok(())
}
