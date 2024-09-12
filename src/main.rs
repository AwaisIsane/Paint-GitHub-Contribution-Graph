use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

fn main() -> std::io::Result<()> {
    // Add a new line to files.txt
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("files.txt")?;
    writeln!(file, "hello")?;

    // Git add files.txt
    Command::new("git").arg("add").arg("files.txt").status()?;

    // Git commit
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("hello")
        .status()?;

    Ok(())
}
