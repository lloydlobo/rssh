//! The run function is hardcoded to only run the rustup command at specific times as defined by the
//! expression string.
//!
//! To allow for more flexible scheduling, you may want to consider allowing the user to provide a
//! cron-style expression as a command line argument.
//!
//! ```yml
//! "0 0 * * * *" - Run every day at midnight (i.e. 0 hours, 0 minutes, 0 seconds).
//! "0 0 0 * * *" - Run once per day, at midnight (i.e. 0 hours, 0 minutes, 0 seconds).
//! "0 0 * * 0 *" - Run every Sunday at midnight (i.e. 0 hours, 0 minutes, 0 seconds).
//! "0 0 0 1 * *" - Run once per month, on the first day of the month at midnight (i.e. 0 hours, 0 minutes, 0 seconds).
//! "0 0 0 1 1 *" - Run once per year, on January 1st at midnight (i.e. 0 hours, 0 minutes, 0 seconds).
//! "0 0 9 * * *" - Run every day at 9:00 AM (i.e. 9 hours, 0 minutes, 0 seconds).
//! "0 0 17 * * *" - Run every day at 5:00 PM (i.e. 17 hours, 0 minutes, 0 seconds).
//! "0 0 12 * * *" - Run every day at noon (i.e. 12 hours, 0 minutes, 0 seconds).
//! "0 0 0 * * 6" - Run every Saturday at midnight (i.e. 0 hours, 0 minutes, 0 seconds).
//! ```
//!
//! These are just a few examples, and you can adjust the values based on your specific needs. The
//! format for cron syntax is as follows:
//!
//! ```txt
//! * * * * * *
//! | | | | | |
//! | | | | | ----- Day of week (0 - 7) (Sunday = both 0 and 7)
//! | | | | ------- Month (1 - 12)
//! | | | --------- Day of month (1 - 31)
//! | | ----------- Hour (0 - 23)
//! | ------------- Minute (0 - 59)
//! ------------- Second (0 - 59)
//! ```

use std::{fmt::Write, time::Duration};

use anyhow::{anyhow, Result};
use cron::Schedule;
use job_scheduler_ng::{Job, JobScheduler};
use xshell::{cmd, Shell};

// use std::{str::FromStr, sync::RwLock};
// use chrono::Utc; use cron::Schedule;

/// The `run` function takes a reference to a `Shell` instance and returns a `Result`.
/// It uses the `xflags` crate to parse the command line arguments, specifically the required
/// `utility` argument and the optional `expression` argument.
///
/// If the `expression` argument is not provided, the default value of "1/10 * * * * *" is used,
/// which means the command will get executed every 10 seconds. If the `utility` argument is
/// `"fortune"`, the `run_fortune_command` function is called with the parsed `expression`.
///
/// If the `utility` argument is unknown, an error message is returned.
///
/// # Usage
///
/// ```sh
/// ❯ cron fortune "1/10 * * * * *"
/// $ fortune
/// Understatement of the century:
/// "Hello everybody out there using minix - I'm doing a (free) operating
///  system (just a hobby, won't be big and professional like gnu) for
///  386(486) AT clones"
///
///         - Linus Torvalds, August 1991
/// $ fortune
/// Sorry.  I forget what I was going to say.
/// ```
pub fn run(_sh: &Shell) -> Result<()> {
    // Parse the command-line arguments using xflags.
    let flags = xflags::parse_or_exit! {
        required utility: String       // The utility or command to run.
        optional expression: String    // Cron expresion.
    };

    // Parse the `expression` string into a `Schedule` value.
    const DEFAULT_EXP: &str = "1/10 * * * * *";
    let expression: &str = flags.expression.as_deref().unwrap_or(DEFAULT_EXP); // default value
    let expression: Schedule = match expression.parse::<Schedule>() {
        Err(err) => {
            eprintln!(
                "{}\n{}\n{expression}",
                print_cron_default_values(),
                generate_cron_ascii_art(),
            );
            return Err(anyhow!(r#"unknown cron syntax: `{err}`"#,));
        }
        Ok(it) => it,
    };

    // Check if the `utility` string matches the expected value, and run the corresponding command.
    // If the `utility` string is unknown, return an error.
    match flags.utility.as_str() {
        "fortune" => run_fortune_cmd(expression, false)?,
        utility => return Err(anyhow!("unknown utility: `{utility}`")),
    }

    Ok(())
}

/// `run_fortune_command` takes a `expression` string and returns a `Result`.
/// It uses the `job_scheduler_ng` crate to schedule the `fortune` command to run according to the
/// provided cron-style `expression`. The function creates a `JobScheduler` instance and adds a new
/// job to it using the `expression` string.
///
/// The function then enters a loop, where it calls `sched.tick()` and sleeps for 500 milliseconds
/// between each iteration. This allows the scheduled `fortune` command to run according to the
/// provided `expression`.
///
/// # Arguments
///
/// * `expression` - a cron-style string that specifies when the `fortune` command should run.
///   (default: gets executed every 10 seconds!)
fn run_fortune_cmd(expression: Schedule, signal: bool) -> Result<()> {
    let mut sched = JobScheduler::new();

    sched.add(Job::new(expression, || {
        let sh = Shell::new().unwrap();
        cmd!(sh, "fortune").run().unwrap();
    }));

    while !signal {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
    Ok(())
}

const CRON_DEFAULTS: [(&str, &str); 6] = [
    ("Minutes", "0-59"),
    ("Hours", "0-23"),
    ("Day of the Month", "1-31"),
    ("Month", "1-12 (or names)"),
    ("Day of the Week", "0-7 (0 or 7 is Sunday, or use names)"),
    ("Command", "Any command to be executed"),
];

/// This code generates a table with two columns, one for the field name and one for its default
/// values, separated by a fixed width space. The maximum width of the field names is calculated to
/// determine the width of the first column, so that both columns are aligned. The values of the
/// CRON_DEFAULTS array are used to populate the rows of the table, and the write! macro is used to
/// format each row.
fn create_table() -> String {
    let mut table = String::new();

    let column_width = CRON_DEFAULTS.iter().map(|(title, _)| title.len()).max().unwrap();

    for (title, default) in CRON_DEFAULTS.iter() {
        writeln!(
            table,
            "{title:<column_width$}  {default}"
        )
        .unwrap();
    }

    table
}

/// The format is a crontab expression, which is a string that defines the schedule for a Unix-style
/// cron job. Each field in a crontab expression corresponds to a different aspect of the schedule,
/// such as the minute, hour, day of the month, etc. The fields are separated by spaces, and each
/// field can contain either an asterisk (*) to match any value, or a specific value or range of
/// values.
fn generate_cron_ascii_art() -> String {
    let cron_syntax = [
        " ┌───────────── minute (0 - 59)",
        " │ ┌───────────── hour (0 - 23)",
        " │ │ ┌───────────── day of the month (1 - 31)",
        " │ │ │ ┌───────────── month (1 - 12)",
        " │ │ │ │ ┌───────────── day of the week (0 - 7) (Sunday is both 0 and 7)",
        " │ │ │ │ │",
        " │ │ │ │ │",
        " * * * * *",
    ];
    cron_syntax.join("\n")
}

fn generate_cron_default_values() -> Vec<&'static str> {
    let cron_syntax = [
        "0 0 * * * *",
        "0 0 0 * * *",
        "0 0 * * 0 *",
        "0 0 0 1 * *",
        "0 0 0 1 1 *",
        "0 0 9 * * *",
        "0 0 17 * * *",
        "0 0 12 * * *",
        "0 0 0 * * 6",
    ];
    cron_syntax.to_vec()
}
fn print_cron_default_values() -> String {
    let cron_syntax = [
        r#""0 0 * * * *" - Run every day at midnight (i.e. 0 hours, 0 minutes, 0 seconds),"#,
        r#""0 0 0 * * *" - Run once per day, at midnight (i.e. 0 hours, 0 minutes, 0 seconds)."#,
        r#""0 0 * * 0 *" - Run every Sunday at midnight (i.e. 0 hours, 0 minutes, 0 seconds)."#,
        r#""0 0 0 1 * *" - Run once per month, on the first day of the month at midnight (i.e. 0 hours, 0 minutes, 0 seconds)."#,
        r#""0 0 0 1 1 *" - Run once per year, on January 1st at midnight (i.e. 0 hours, 0 minutes, 0 seconds)."#,
        r#""0 0 9 * * *" - Run every day at 9:00 AM (i.e. 9 hours, 0 minutes, 0 seconds)."#,
        r#""0 0 17 * * *" - Run every day at 5:00 PM (i.e. 17 hours, 0 minutes, 0 seconds)."#,
        r#""0 0 12 * * *" - Run every day at noon (i.e. 12 hours, 0 minutes, 0 seconds)."#,
        r#""0 0 0 * * 6" - Run every Saturday at midnight (i.e. 0 hours, 0 minutes, 0 seconds)."#,
    ];
    cron_syntax.join("\n")
}

#[cfg(test)]
mod tests {
    use std::{
        io::{self, BufRead},
        sync::{
            mpsc,
            mpsc::TryRecvError,
        },
        thread,
        time::{Duration},
    };

    use super::*;

    #[test]
    fn try_terminate() {
        println!("Press enter to terminate the child thread");
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || loop {
            println!("Working...");
            try_fortune_cmd(false);
            thread::sleep(Duration::from_millis(500));
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating.");
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
        });

        let mut line = String::new();
        let stdin = io::stdin();
        let _ = stdin.lock().read_line(&mut line);

        let _ = tx.send(());
    }

    #[test]
    fn try_suspend_term_main() {
        println!("Press enter to wake up the child thread");
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || loop {
            println!("Suspending...");
            match rx.recv() {
                Ok(_) => {
                    println!("Working...");
                    try_fortune_cmd(false);
                    thread::sleep(Duration::from_millis(500));
                }
                Err(_) => {
                    println!("Terminating.");
                    break;
                }
            }
        });

        let mut line = String::new();
        let stdin = io::stdin();
        for _ in 0..4 {
            let _ = stdin.lock().read_line(&mut line);
            let _ = tx.send(());
        }
    }

    fn try_fortune_cmd(signal: bool) {
        // Run command every 2 seconds till `dur` elapses.
        let expression = "1/2 * * * * *".parse::<Schedule>().expect(
            "Failed to parse cron
    syntax",
        );
        assert!(run_fortune_cmd(expression, signal).is_ok());
    }
    //
    // #[test]
    // #[should_panic]
    // fn test_run_expression_invalid() {
    //     wait_to_exit_par_thread();
    //     let expression = "invalid expression";
    //     let result = run_test_thread(expression);
    //     assert!(result.is_err());
    // }
    //
    // #[test]
    // fn test_run_fortune_cmd_with_default_cron_syntax() {
    //     for syntax in generate_cron_default_values() {
    //         let expression = syntax.parse::<Schedule>().expect("Failed to parse cron syntax");
    //         assert!(run_fortune_cmd(expression).is_ok());
    //     }
    // }
}
