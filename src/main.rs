use clap::Parser;

#[derive(Parser)]
#[command(name = "Riff")]
#[command(version = "0.1")]
#[command(about = "Diff tool for environment files")]
struct Cli {
    file_a: String,
    file_b: String,
    #[arg(long, short='s')]
    separator: Option<char>,
    #[arg(long, short='c')]
    comment: Option<char>,
}

/// A Rust Diff tool for finding the differences between two environment variable files.
/// By: Spencer Newton
fn main() {
    let cli = Cli::parse();

    println!("File A: {:?}", cli.file_a);
    println!("File B: {:?}", cli.file_b);
}
