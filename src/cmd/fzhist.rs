use std::{io::Read, path::PathBuf};

use anyhow::{anyhow, Context, Result};
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
pub fn run(sh: &Shell) -> Result<()> {
    let mut shell = std::env::var("SHELL").unwrap_or_else(|_| "".to_string());

    shell = match shell.as_str() {
        s if s.ends_with("bash") => "bash",
        s if s.ends_with("zsh") => "zsh",
        _ => "bash",
    }
    .to_string();

    let mut history_file_path = PathBuf::new();
    let base_path = std::env::var("HOME").unwrap_or_default(); // PERF: Is this secure?
    history_file_path.push(&base_path);
    match &shell[..] {
        "bash" => history_file_path.push(".bash_history"),
        "zsh" => history_file_path.push(".zsh_history"),
        _ => return Err(anyhow!("Unknown shell: {}", shell)),
    }

    let mut history = String::new();
    std::fs::File::open(&history_file_path)?.read_to_string(&mut history).with_context(|| {
        anyhow!(format!("Should open history file: `{}`", history_file_path.display()))
    })?;
    if shell.starts_with("zsh") {
        history = history
            .lines()
            .map(|line| {
                #[allow(clippy::single_char_pattern)]
                let args = line.splitn(2, ";").collect::<Vec<_>>();
                match args.last() {
                    Some(arg) => arg.trim().to_owned(),
                    None => args.first().unwrap().to_owned().to_string(),
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
    }

    // Interactively display the contents of the history file in fzf by
    // passing history slice to the standard input of the spawned process.
    let selected_cmd = cmd!(sh, "fzf --height 40% --reverse --tac").stdin(history).read()?;
    if selected_cmd.is_empty() {
        return Ok(());
    }
    // Copy the selected command to clipboard.
    cmd!(sh, "xsel -ib").stdin(&selected_cmd).output()?;

    println!("Copied command `{}` to your clipboard.", &selected_cmd);
    print!("Press [Ctrl/Cmd + Shift/Option + v] to paste and reuse.");
    // TODO: Flush the selected command to stdout in real-time
    // let mut stdout = std::io::stdout(); writeln!(stdout, "{}", &selected_cmd)?; stdout.flush()?;

    Ok(())
}

// let clipboard = cmd!(sh, "xsel -ob").output()?;
// std::io::stdout().write_all(clipboard.stdout.as_bytes())?;

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
//
// let is_zsh = shells.into_iter().find(|x| get_current_shell.contains(x));
// dbg!(&get_current_shell, &is_zsh);
// // let curr_shell = shells
// //     .into_iter()
// //     .find(|x| (*x).cmp(get_current_shell.as_str()) == Ordering::Equal)
// //     .unwrap();
