use xshell::{cmd, Shell};

/// Perform a memory flush and cleanup by disabling and enabling swap areas.
///
/// # Usage
///
/// ```bash
/// $ swapoffon
/// ```
///
/// This function performs a memory flush and cleanup by disabling and enabling all swap areas.
///
/// Before the memory flush and cleanup, the function displays the current memory usage in
/// human-readable units using the `free -h` command.
///
/// After the memory flush and cleanup, the function displays the new memory usage in human-readable
/// units using the `free -h` command again.
pub fn run(sh: &Shell) -> anyhow::Result<()> {
    // Display current memory usage in human-readable units:
    cmd!(sh, "free -h").run()?;

    // Disable all swap areas:
    cmd!(sh, "sudo swapoff -a").run()?;

    // Enable all swap areas:
    cmd!(sh, "sudo swapon -a").run()?;

    // Display new memory usage in human-readable units:
    cmd!(sh, "free -h").run()?;

    Ok(())
}
