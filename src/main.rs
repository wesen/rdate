// convert a timestamp to a formatted date
use clap::Parser;
use chrono::prelude::*;

#[derive(Parser)]
struct Cli {
    // timestamp to convert
    timestamp: String,
    // date format (default "%Y-%m-%d %H:%M:%S")
    #[clap(short, long, default_value = "%Y-%m-%d %H:%M:%S")]
    format: String,
}

fn main() {
    let args = Cli::parse();
    let timestamp : i64 = args.timestamp.parse().unwrap();
    let date = Utc.timestamp(timestamp, 0);
    println!("{}", date.format(&args.format).to_string());
}
