use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use cccompress::compressor;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: PathBuf,

    /// Output file
    #[arg(short, long)]
    output: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Compress,
    Uncompress,
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Compress => {
            println!("Compressing {:?}...", args.input);
            let contents = fs::read(args.input).unwrap();
            let compressed = compressor::compress(contents);

            println!("Writing compressed file {:?}...", args.output);
            fs::write(args.output, compressed).unwrap();
        }
        Commands::Uncompress => {
            println!("Uncompressing {:?}...", args.input);
            let contents = fs::read(args.input).unwrap();
            let uncompressed = compressor::uncompress(contents);

            println!("Writing uncompressed file {:?}...", args.output);
            fs::write(args.output, uncompressed).unwrap();
        }
    }
}
