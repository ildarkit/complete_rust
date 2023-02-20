use clap_verbosity_flag::Verbosity;
use clap::Parser;

const CONN_ADDR: &str = "127.0.0.1:3002";

/// This is small CLI tool to shorten urls using the hyperurl
/// url shortening service
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The url to shorten
    #[arg(short, long)]
    url: String,
    /// Setting logging for this CLI tool
    #[command(flatten)]
    verbosity: Verbosity,
}

fn main() -> Result<(), reqwest::Error> {
    let args = Cli::parse();
    println!("Shortening: {}", args.url);
    let client = reqwest::blocking::Client::new();
    let res = client
        .post(&format!("http://{}/shorten", CONN_ADDR))
        .body(args.url)
        .send()?;
    let a: String = res.text().unwrap();
    println!("{}", a);
    Ok(())
}
