use std::fs;
use anyhow::{Context, Result};

use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
struct Cli {
	/// The pattern to look for
	#[structopt(short = "p", long = "pattern")]
    pattern: String,
	/// The path to the file to read
	#[structopt(parse(from_os_str), short = "f", long = "file")]
    path: std::path::PathBuf,
}


fn main() -> Result<()> { 
    let args = Cli::from_args();
	let content = fs::read_to_string(&args.path)
		.with_context(|| format!("could not read file `{:?}`", &args.path))?;
	for line in content.lines() {
		if line.contains(&args.pattern) {
			println!("{}", line)
		}
	}
	Ok(())
}