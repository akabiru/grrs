use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use clap::Parser;

// Search for a pattern in a fle and display the lines that contain it
//
// Model programs around the data they handle; struct like an object’s data attributes (in OOP).
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf, // PathBuff is like a String but for file system path that work cross platform
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let f = File::open(&args.path)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        // Extract the line, or handle the error.
        match line {
            Ok(line) => {
                if line.trim().contains(&args.pattern) {
                    println!("{}", line)
                }
            }
            Err(err) => panic!("failed to reader line {}", err),
        };
    }

    Ok(())
}
