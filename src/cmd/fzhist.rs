use anyhow::Result;
use xshell::{cmd, Shell};

/// `fzhist`: Reads the contents of the bash history file and display them in `fzf` (Command-line
/// fuzzy finder.)
///
/// The selected command will be copied to the clipboard.
///
/// # Usage:
///
/// ```markdown
/// Run command `fzhist` to read bash history to fzf.
///
/// The fzf utility will open a fuzzy search interface with your command history .
///
/// Type in a search term to filter by the search term.
///
/// Use the arrow keys to navigate the search results and select a command.
///
/// Press Enter to execute the selected command.
///
/// NOTE: It is similar to binding fzf to Ctrl+R for reverse history search.
/// ```
pub fn run(sh: &Shell) -> Result<()> {
    // Path to the bash history file.
    let history_file_path: &str = "/home/lloyd/.bash_history";

    // Read the contents of the history file.
    let history: String = cmd!(sh, "cat {history_file_path}").read()?;

    // Interactively display the contents of the history file in fzf
    // cmd!(sh, "fzf --height 40% --reverse --tac").stdin(output).run()?;
    let selected_cmd = cmd!(sh, "fzf --height 40% --reverse --tac").stdin(history).read()?;

    // Copy the selected command to clipboard.
    let _clipboard = cmd!(sh, "xsel -ib").stdin(selected_cmd.to_string()).run()?;

    Ok(())
}

// NOTE: if `$ history` command works, remove line numbers in the first column.
//
// Call the history function to read and print the contents of the history file
//
// ```ignore
// let history = cmd!(sh, "history").read();
// let output = cmd!(sh, "awk '{$1=\"\"; print $0}' {history_file}").stdin(history).read()?;
// ```

// TODO: If user exits fzf, display no errors in stdout.
// fn no_exit_status() -> result<()> {
//     let sh = shell::new();
//
//     let history_file = "~/.bash_history";
//     let output = cmd!(sh, format!("awk '!/^\\s*$/ {{ \$1=\"\"; print \$0 }}' {}",
// history_file)).output()?;
//
//     if output.status().map_or(false, |s| s.code() == some(130)) {
//         // user exited fzf voluntarily
//         return ok(());
//     }
//
//     let selected_command = cmd!(sh, "fzf --height 40% --reverse --tac")
//         .stdin(output.stdout)
//         .output()?
//         .stdout;
//
//     // remove leading/trailing white space
//     let selected_command = selected_command.trim();
//
//     if !selected_command.is_empty() {
//         writeln!(std::io::stdout(), "{}", selected_command)?;
//     }
//
//     ok(())
// }
