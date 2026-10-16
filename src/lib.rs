use std::{
    fs::File,
    io::{self, Write},
    path,
};

pub fn cuco() -> io::Result<()> {
    let mut args: Vec<_> = std::env::args().collect();
    let cmd = args.remove(0);

    // Print what we are doing.
    let (keyword, actual_cmd) = if cmd.ends_with("cu") {
        ("Cut", "mv")
    } else if cmd.ends_with("co") {
        ("Copy", "cp")
    } else {
        unreachable!()
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

    write!(clipboard_file, "{} {}", actual_cmd, args.join(","))?;

    Ok(())
}
