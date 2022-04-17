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

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("Could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }
}
