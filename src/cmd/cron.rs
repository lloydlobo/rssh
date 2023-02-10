//! The run function is hardcoded to only run the rustup command at specific times as defined by the
//! expression string.
//! To allow for more flexible scheduling, you may want to consider allowing the user to provide a
//! cron-style expression as a command line argument.

use std::time::Duration;

// use std::str::FromStr;
// use std::sync::RwLock;
// use chrono::Utc;
// use cron::Schedule;
use anyhow::Result;
use job_scheduler_ng::{Job, JobScheduler};
use xshell::{cmd, Shell};

/// The `run` function takes a reference to a `Shell` instance and returns a `Result`.
/// It uses the `xflags` crate to parse the command line arguments, specifically the required
/// `utility` argument and the optional `expression` argument.
///
/// If the `expression` argument is not provided, the default value of "1/10 * * * * *" is used,
/// which means the command will get executed every 10 seconds. If the `utility` argument is
/// `"fortune"`, the `run_fortune_command` function is called with the parsed `expression`.
///
/// If the `utility` argument is unknown, an error message is returned.
pub fn run(sh: &Shell) -> Result<()> {
    let flags = xflags::parse_or_exit! {
        required utility: String       // The utility or command to run.
        optional expression: String    // Cron expresion.
    };

    let expression: &str = flags.expression.as_deref().unwrap_or("1/10 * * * * *");

    match flags.utility.as_str() {
        "fortune" => {
            run_fortune_cmd(expression)?;
        }
        utility => {
            return Err(anyhow::Error::msg(format!("unknown utility: `{}`", utility)));
        }
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
fn run_fortune_cmd(expression: &str) -> Result<()> {
    let mut sched = JobScheduler::new();
    sched.add(Job::new(expression.parse().unwrap(), || {
        let sh = Shell::new().unwrap();
        cmd!(sh, "fortune").run().unwrap();
    }));

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn test_run_fortune_command() {
        let expression = "1/10 * * * * *";
        let result = run_test_thread(expression);
        assert!(result.is_ok());
    }

    #[test]
    #[should_panic]
    fn test_run_expression_invalid() {
        let expression = "invalid expression";
        let result = run_test_thread(expression);
        assert!(result.is_err());
    }

    fn run_test_thread<'a>(expression: &'static str) -> Result<()> {
        let join_handle = thread::spawn(move || run_fortune_cmd(expression));
        join_handle.join().unwrap()
    }
}
