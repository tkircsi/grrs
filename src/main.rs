use clap::Parser;
use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::{Context, Result};
use log::{info, warn};

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


fn main() -> Result<()>{
    env_logger::init();
    info!("starting up");

    let args = Cli::parse();
    let path = &args.path.to_string_lossy();

    info!("open file {}", &path);
    let f = File::open(&args.path)
        .with_context(|| { format!("could not read file `{}`", path) })?;
    let buf = BufReader::new(f); 


    // let content = std::fs::read_to_string(&args.path).expect("could not read file");

    info!("reading file...");
    for line in buf.lines() {
        match line {
            Ok(l) => {
                if l.contains(&args.pattern) {
                    println!("{}", l);
                }
            },
            Err(err) => {
                panic!("{}", err);
            }
        }
        
    }

    info!("finished");
    Ok(())
}