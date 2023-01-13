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
    find_matches(buf, &pattern, &mut std::io::stdout()).expect("error reading file");

    info!("finished");
    Ok(())
}

fn find_matches(
    reader: impl std::io::BufRead,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.contains(pattern) {
                    writeln!(writer, "{}", l)?;
                }
            }
            Err(err) => return Err(err.into()),
        }
    }
    Ok(())
}

#[test]
fn find_a_match() {
    use stringreader::StringReader;

    let input = r#"
    [package]
    name = "grrs"
    version = "0.1.0"
    edition = "2021"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]
    anyhow = "1.0.68"
    clap = { version = "4.0.32", features = ["derive"] }
    env_logger = "0.10.0"
    log = "0.4.17"
    "#;

    let output = r#"    version = "0.1.0"
    clap = { version = "4.0.32", features = ["derive"] }
"#;
    let string_reader = StringReader::new(input);
    let reader = BufReader::new(string_reader);
    let mut result = Vec::new();
    find_matches(reader, "version", &mut result).expect("error reading file");

    assert_eq!(result, output.as_bytes());
}
