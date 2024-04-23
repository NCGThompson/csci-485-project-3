//! See [Args] documentation for more information.

use std::{ops::Deref as _, time::Instant};

use libproj3::scraping::{self as s, execute_search};

use clap::Parser;
use clio::ClioPath;
use rust_search::SearchBuilder;

fn main() {
    let start = Instant::now();

    let args = Args::parse();

    let target_strs: Vec<&str>;
    if !args.target_names.is_empty() {
        target_strs = args.target_names.iter().map(String::deref).collect();
    } else {
        target_strs = vec!["special_file.txt", "secret_file.txt"];
    }
    let mut paths: Vec<Option<String>> = Vec::new();
    paths.reserve_exact(target_strs.len());
    paths.resize(target_strs.len(), None);

    fn rebuild(target_strs: Option<&[&str]>) -> SearchBuilder {
        if let Some(targets) = target_strs {
            s::multiple_searches(SearchBuilder::default(), targets.iter().copied())
        } else {
            s::multiple_searches_default(SearchBuilder::default())
        }
    }

    let mut all_found: Option<f64> = None;
    let max = if args.home_only { 1 } else { usize::MAX };
    let presets = [s::stage_1, s::stage_2, s::stage_3];
    for (i, f) in presets.into_iter().enumerate().take(max) {
        println!(
            "Beginning Stage {} at {}s",
            i + 1,
            start.elapsed().as_secs_f64()
        );
        let mut search = rebuild(if args.target_names.is_empty() {
            None
        } else {
            Some(&target_strs)
        });
        search = f(search);

        execute_search(&mut paths, search, &target_strs, true);

        let time = start.elapsed().as_secs_f64();
        if paths.iter().all(Option::is_some) {
            all_found.get_or_insert(time);
            if args.short_circuit {
                break;
            }
        }
        println!("Ending Stage {} at {}s", i + 1, time);
    }

    println!("\nResults:");
    for i in 0..target_strs.len() {
        println!(
            "{}: {}",
            target_strs[i],
            paths[i].clone().unwrap_or("Not found".to_string())
        );
    }
    println!();

    if let Some(time) = all_found {
        println!("All targets found after {}s since start.", time);
    }
    println!("Total run time: {}s", start.elapsed().as_secs_f64());
}

/// Searches computer for files by name
///
/// This is a test binary for Project 3. It represents the part of the program that
/// searches for the target files (`secret_file.txt` and `special_file.txt`),
/// but it provides debugging output and is configurable at runtime.
#[derive(Parser, Debug)]
#[command(version, author, about)]
struct Args {
    /// Only search the user's home directory.
    ///
    /// This is the inverse of the `sudoer`` flag (which has no effect here). When false,
    /// it will search almost all files on the system, but prioritizing
    /// the most likely locations. When true, it stops after the first round, making
    /// `short_circuit` redundant.
    #[arg(long, default_value_t = false)]
    home_only: bool,

    /// Currently this is ignored because it is always on
    #[arg(long, default_value_t = true)] // TODO defaults
    sanitize_input: bool,

    #[arg(short, long)]
    target_names: Vec<String>,

    /// Currently this is ignored because it has predetermined search destinations.
    #[arg(short = 'r', long)]
    #[clap(value_parser = clap::value_parser!(ClioPath).exists().is_dir())]
    search_root: Option<ClioPath>,

    /// Stop when done
    /// 
    /// When this is false (default), it will search the whole drive regardless of if it already found
    /// all target files. It will display the actual time taken to execute as well
    /// as the time it would have terminated if this were false.
    #[arg(short, long)]
    short_circuit: bool,
}
