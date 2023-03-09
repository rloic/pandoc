pub mod definition;
pub mod walkable;

extern crate serde;

use std::io;
use std::io::{BufRead, stdin};
use crate::definition::Pandoc;
use crate::walkable::Walkable;

pub fn to_json_filter<F, I, O>(f: &mut F) -> io::Result<()> where Pandoc: Walkable<I, O>, F: FnMut(I) -> O {
    let mut pandoc_json = String::new();
    for line in stdin().lock().lines() {
        if let Ok(line) = line {
            pandoc_json += line.as_str();
        }
    }
    let pandoc: Pandoc = serde_json::from_str(&pandoc_json)?;
    println!("{}", serde_json::to_string(&pandoc.walk(f))?);
    Ok(())
}






