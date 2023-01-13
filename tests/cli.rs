use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

const INPUT: &str = r#"
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

const OUTPUT: &str = r#"    version = "0.1.0"
    clap = { version = "4.0.32", features = ["derive"] }
"#;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("--path")
        .arg("test/file/doesnt/exist")
        .arg("--pattern")
        .arg("let");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str(INPUT)?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("--path")
        .arg(file.path())
        .arg("--pattern")
        .arg("version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(OUTPUT));

    Ok(())
}
