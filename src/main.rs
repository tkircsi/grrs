use anyhow::{Context, Result};
use clap::Parser;
use log::info;
use std::fs::File;
use std::io::BufReader;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    #[arg(long)]
    pattern: String,
    /// The path to the file to read
    #[arg(short, long)]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("starting up");

    let args = Cli::parse();
    let path = &args.path.to_string_lossy();
    let pattern = args.pattern;

    info!("open file {}", &path);
    let f = File::open(&args.path).with_context(|| format!("could not read file `{}`", path))?;
    let buf = BufReader::new(f);

    info!("reading file...");
    grrs::find_matches(buf, &pattern, &mut std::io::stdout()).expect("error reading file");

    info!("finished");
    Ok(())
}
