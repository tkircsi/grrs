pub fn find_matches(
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
    use std::io::BufReader;
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
