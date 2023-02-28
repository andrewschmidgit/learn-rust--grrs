use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let file_result = File::open(args.path);
    let reader = BufReader::new(file_result?);

    for line_result in reader.lines() {
        let line = line_result?;
        if !line.contains(&args.pattern) { continue; }

        println!("{}", line);
    };

    Ok(())
}

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
