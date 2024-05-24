use std::{fs::File, io::{self, BufRead, BufReader}};
use clap::Parser;
use anyhow::Result;
use std::io::Read;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `head`
struct Args {
    /// Input file(s)
    #[arg(default_value="-", value_name="FILE")]
    files: Vec<String>,

    /// Lines to output
    #[arg(
        short('n'),
        long,
        default_value = "10",
        value_name = "LINES",
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    lines: u64,

    /// Number of bytes
    #[arg(
        short('c'),
        long,
        value_name = "BYTES",
        conflicts_with("lines"),
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    bytes: Option<u64>
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(args: Args) -> Result<()> {
    let num_files = args.files.len();

    for (file_num, filename) in args.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {filename} <==",
                        if file_num > 0 {"\n"} else {""}
                    );
                }
                if let Some(num_bytes) = args.bytes {
                    let bytes = file
                        .bytes()
                        .take(num_bytes as usize)
                        .collect::<Result<Vec<_>, _>>();
                    print!(
                        "{}",
                        String::from_utf8_lossy(&bytes?)
                    );
                } else {
                    let mut line = String::new();
                    for  _ in 0..args.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}