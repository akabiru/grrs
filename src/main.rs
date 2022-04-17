use clap::Parser;

// Model programs around the data they handle; struct
// A struct is like an object’s data attributes (in OOP).


// Search for a pattern in a fle and display the lines that contain it
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf, // PathBuff is like a String but for file system path that work cross platform
}

fn main() {
    let _args = Cli::parse();
}
