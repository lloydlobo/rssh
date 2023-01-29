use xshell::{
    cmd,
    Shell,
};

pub fn run(sh: &Shell) -> anyhow::Result<()> {
    let args = std::env::args().skip(1);
    cmd!(sh, "cargo run -q --manifest-path /home/lloyd/config/script/Cargo.toml -- {args...}")
        .run()?;
    Ok(())
}
