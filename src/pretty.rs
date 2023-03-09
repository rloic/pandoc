use std::io;
use std::io::{BufRead, stdin};
use pandoc::definition::Pandoc;

fn main() -> io::Result<()> {
    let mut pandoc_json = String::new();
    for line in  stdin().lock().lines() {
        if let Ok(line) = line {
            pandoc_json += line.as_str();
        }
    }
    let pandoc: Pandoc = serde_json::from_str(&pandoc_json)?;
    println!("{}", serde_json::to_string_pretty(&pandoc)?);
    Ok(())
}