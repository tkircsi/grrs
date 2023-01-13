# Rust grep
Let’s write a small grep clone.

## Usage
```bash
$ RUST_LOG=info ./target/debug/grrs --help
[2023-01-13T13:09:36Z INFO  grrs] starting up
Search for a pattern in a file and display the lines that contain it

Usage: grrs.exe --pattern <PATTERN> --path <PATH>

Options:
      --pattern <PATTERN>  The pattern to look for
  -p, --path <PATH>        The path to the file to read
  -h, --help               Print help information
```

## Example

```bash
$ RUST_LOG=info ./target/debug/grrs --pattern let --path ./src/main.rs
[2023-01-13T13:08:51Z INFO  grrs] starting up
[2023-01-13T13:08:51Z INFO  grrs] open file ./src/main.rs
[2023-01-13T13:08:51Z INFO  grrs] reading file...
    let args = Cli::parse();
    let path = &args.path.to_string_lossy();
    let f = File::open(&args.path)
    let buf = BufReader::new(f);
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
[2023-01-13T13:08:51Z INFO  grrs] finished
```
