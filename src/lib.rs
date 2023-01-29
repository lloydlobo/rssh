pub mod cmd;
pub mod util;

use xshell::Shell;

pub use crate::{
    cmd::*,
    util::*,
};

pub const COMMANDS: &[(&str, fn(&Shell) -> anyhow::Result<()>)] = &[
    // ("ammend", ammend::run),
    // ("autostart", autostart::run),
    // ("commit", commit::run),
    ("go", go::run),
    // ("script", script::run),
];
