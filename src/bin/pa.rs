use std::{error::Error, fs, io, process::Command};

fn read_clipboard() -> io::Result<String> {
    let clipboard_path = xdg::BaseDirectories::with_prefix("cucopa")
        .get_state_file("clipboard")
        .ok_or_else(|| io::Error::other("clipboard file not found"))?;

    let clipboard = fs::read_to_string(clipboard_path)?;

    Ok(clipboard)
}

fn parse_clipboard(raw: &str) -> Result<(&str, Vec<&str>), &str> {
    let (cmd, rest) = raw.split_once(" ").ok_or("Parsing failed: No space")?;
    let args = rest.split(",").collect();
    Ok((cmd, args))
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let raw = read_clipboard()?;
    let (cmd, mut args) = parse_clipboard(&raw)?;

    args.push(".");

    Command::new(cmd).args(args).status()?;

    Ok(())
}
