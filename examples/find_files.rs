use clap::Parser;
use clio::ClioPath;

fn main() {
    let _ = Args::parse();
}

/// Searches computer for files by name
///
/// This is a test binary for Project 3. It represents the part of the program that
/// searches for the target files (`secret_file.txt` and `special_file.txt`),
/// but it provides debugging output and is configurable at runtime.
#[derive(Parser, Debug)]
#[command(author)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    privaleged_user: bool,

    #[arg(long, default_value_t = true)] // TODO defaults
    sanitize_input: bool,

    #[arg(short, long)]
    file_names: Option<Vec<String>>,

    /// path to directory
    #[arg(short = 'r', long)]
    #[clap(value_parser = clap::value_parser!(ClioPath).exists().is_dir())]
    search_root: Option<ClioPath>,
}
