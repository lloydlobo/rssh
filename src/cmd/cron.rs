//! The code is a Rust program that runs a given shell utility using cron syntax to schedule the
//! execution. The program parses the command line arguments to get the utility to run and the cron
//! expression (if provided), and then creates a Schedule value from the expression string. If the
//! expression string is not provided, a default value is used, which runs the command every 10
//! seconds. If the expression string is invalid, the program prints a default values chart and an
//! ASCII art illustration of the cron syntax, and returns an error message.!
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
//! The format for cron syntax is as follows:
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

use std::time::Duration;

use anyhow::{anyhow, Result};
use cron::Schedule;
use job_scheduler_ng::{Job, JobScheduler};
use xshell::{cmd, Shell};

const DEFAULT_EXPRESSION: &str = "1/10 * * * * *";

/// It uses the `xflags` crate to parse the command line arguments, specifically the required
/// `utility` argument and the optional `expression` argument.
///
/// If the `expression` argument is not provided, the default value of "1/10 * * * * *" is used,
/// which means the command will get executed every 10 seconds. If the `utility` argument is
/// `"fortune"`, the `run_fortune_command` function is called with the parsed `expression`.
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
///
/// To allow for more flexible scheduling, you may want to consider allowing the user to provide a
/// cron-style expression as a command line argument.
pub fn run(_sh: &Shell) -> Result<()> {
    // Parse the command-line arguments using xflags.
    let flags = xflags::parse_or_exit! {
        required utility: String       // The utility or command to run.
        optional expression: String    // Cron expresion.
    };

    let expression: &str = flags.expression.as_deref().unwrap_or(DEFAULT_EXPRESSION);

    let schedule: Schedule = match expression.parse::<Schedule>() {
        Ok(sched) => sched,
        Err(err) => {
            let available = print_cron_default_values();
            let help = generate_cron_ascii_art();
            eprintln!("{available}\n{help}\n{expression}");
            return Err(anyhow!(r#"unknown cron syntax: `{err}`"#,));
        }
    };

    match flags.utility.as_str() {
        "fortune" => run_fortune_cmd(schedule, false)?,
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
        fmt::Write,
        io::{self, BufRead},
        sync::{mpsc, mpsc::TryRecvError},
        thread,
        time::Duration,
    };

    use super::*;

    // Run command every 2 seconds till `dur` elapses.
    fn try_fortune_cmd(signal: bool) {
        let expression = "1/2 * * * * *".parse::<Schedule>().expect("Failed to parse cron syntax");
        assert!(run_fortune_cmd(expression, signal).is_ok());
    }

    /// Terminated externally
    ///
    /// On each iteration of a worker loop, we check if someone notified us through a channel. If
    /// yes or if the other end of the channel has gone out of scope we break the loop.
    /// https://stackoverflow.com/a/26200583
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
    /// Suspending and resuming
    ///
    /// We use recv() which suspends the thread until something arrives on the channel. In order to
    /// resume the thread, you need to send something through the channel; the unit value () in this
    /// case. If the transmitting end of the channel is dropped, recv() will return Err(()) - we use
    /// this to exit the loop.      
    /// https://stackoverflow.com/a/26200583
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

    /// This code generates a table with two columns, one for the field name and one for its default
    /// values, separated by a fixed width space. The maximum width of the field names is calculated
    /// to determine the width of the first column, so that both columns are aligned. The values
    /// of the CRON_DEFAULTS array are used to populate the rows of the table, and the write!
    /// macro is used to format each row.
    #[allow(dead_code)]
    fn create_table() -> String {
        const CRON_DEFAULTS: [(&str, &str); 6] = [
            ("Minutes", "0-59"),
            ("Hours", "0-23"),
            ("Day of the Month", "1-31"),
            ("Month", "1-12 (or names)"),
            ("Day of the Week", "0-7 (0 or 7 is Sunday, or use names)"),
            ("Command", "Any command to be executed"),
        ];
        let mut table = String::new();
        let column_width = CRON_DEFAULTS.iter().map(|(title, _)| title.len()).max().unwrap();
        for (title, default) in CRON_DEFAULTS.iter() {
            writeln!(table, "{title:<column_width$}  {default}").unwrap();
        }
        table
    }

    #[allow(dead_code)]
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
}
