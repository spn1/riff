mod differ;
mod file_loader;
mod tui;

use clap::Parser;

use differ::get_diff;
use tui::render_diff;

#[derive(Parser)]
#[command(name = "Riff")]
#[command(version = "0.1")]
#[command(about = "Diff tool for environment files")]
struct Cli {
    file_a: String,
    file_b: String,

    #[arg(long, short = 's')]
    separator: Option<char>,

    #[arg(long, short = 'c')]
    comment: Option<char>,
}

/// A Rust Diff tool for finding the differences between two environment variable files.
/// By: Spencer Newton
fn main() {
    let cli = Cli::parse();

    let file_a_contents = file_loader::read_file(&cli.file_a).unwrap();
    let file_b_contents = file_loader::read_file(&cli.file_b).unwrap();

    println!("File A: {:?}\n\n\n", file_a_contents);
    println!("File B: {:?}\n\n\n", file_b_contents);

    let diff = get_diff(&file_a_contents, &file_b_contents);
    println!("Diff: {:?}", diff);

    render_diff(diff);
}
