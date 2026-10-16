use std::{
    fs::File,
    io::{self, Write},
    path,
};

pub fn cuco(cmd: &str, args: Vec<String>) -> io::Result<()> {
    // Print what we are doing.
    let keyword = match cmd {
        "cu" => "Cut",
        "co" => "Copy",
        _ => unreachable!()
    };
    println!("{}", keyword);
    for arg in &args {
        println!("\t{}", arg);
    }

    let clipboard_path =
        xdg::BaseDirectories::with_prefix("cucopa").place_state_file("clipboard")?;

    let mut clipboard_file = File::create(clipboard_path)?;

    // Get absolute paths for the args.
    let args: Vec<_> = args
        .iter()
        .map(path::absolute)
        .collect::<io::Result<Vec<_>>>()?
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();

    write!(clipboard_file, "{} {}", cmd, args.join(","))?;

    Ok(())
}
