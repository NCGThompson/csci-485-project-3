pub mod directories;
mod tests;

use rust_search::SearchBuilder;
use std::{ops::Deref, path::Path, time::Instant};

/// Searches for the files we need, `special_file.txt` and `secret_file.txt`
/// and returns there locations if found each as a `String` in the order listed here.
/// This uses the library `rust_search` which indirectly wraps `walkdir`.
///
/// `scrape()` will always search the running users home directory if possible.
/// Iff it still hasn't found one or more of the files, and the `sudoer`
/// feature is enabled, it will search the root directory next.
///
/// This function has optimizations for most major desktop environments,
/// but it primarily focuses on the Filesystem Hierchy Standard because
/// that is what Ubuntu (our target's OS) uses.
pub fn find_files() -> Result<(String, String), String> {
    let targets = ["special_file.txt", "secret_file.txt"];
    let mut paths = [None, None];

    #[cfg(not(feature = "sudoer"))]
    {
        let mut builder = stage_1(rust_search::SearchBuilder::default());
        builder = multiple_searches_default(builder);

        execute_search(&mut paths, builder, &targets, false);
    }
    #[cfg(feature = "sudoer")]
    {
        let presets = [stage_1, stage_2, stage_3];
        for f in presets {
            let mut search = multiple_searches_default(SearchBuilder::default());
            search = f(search);

            execute_search(&mut paths, search, &targets, false);

            if paths.iter().all(Option::is_some) {
                break;
            }
        }
    }

    let [special, secret] = paths;
    Ok((special.ok_or("No Special")?, secret.ok_or("No Secret")?))
}

/// Same as [`multiple_searches`], but won't escape
/// [regex meta characters](https://docs.rs/regex/latest/regex/#syntax).
pub fn multiple_searches_raw<S: Deref<Target = str>>(
    builder: SearchBuilder,
    targets: &[S],
) -> SearchBuilder {
    let input = concatenate_targets(targets);

    builder.strict().search_input(input).ext("{0}")
}

/// Tricks a [`SearchBuilder`] into searching for multiple strings
pub fn multiple_searches<S: Deref<Target = str>>(
    builder: SearchBuilder,
    targets: impl IntoIterator<Item = S>,
) -> SearchBuilder {
    let targets: Vec<String> = targets
        .into_iter()
        .map(|x| regex_syntax::escape(&x))
        .collect();

    multiple_searches_raw(builder, &targets)
}

/// Sets a [`SearchBuilder`] to find `special_file.txt` and `secret_file.txt`
pub fn multiple_searches_default(builder: SearchBuilder) -> SearchBuilder {
    builder
        .strict()
        .search_input(r"(?:special|secret)_file")
        .ext("txt")
}

/// Creates a regex pattern that matches either of the provided strings.
///
/// ``` text
/// (?:str1|str2|str3)
/// ```
fn concatenate_targets<S: Deref<Target = str>>(targets: &[S]) -> String {
    if targets.len() <= 1 {
        return String::from(targets.first().map(|x| &**x).unwrap_or_default());
    }

    let length = 3 + targets.len() + targets.iter().map(|x| x.len()).sum::<usize>();
    let mut input = String::with_capacity(length);

    let mut targets = targets.iter().map(|x| &**x);
    input.push_str(r"(?:");
    input.push_str(targets.next().unwrap_or_default());
    for target in targets {
        input.push('|');
        input.push_str(target);
    }
    input.push(')');

    input
}

/// This function consumes a [`Search`](rust_search::Search) instance and writes
/// the results to `paths`.
fn process_results<S: Deref<Target = str>>(
    paths: &mut [Option<String>],
    search: rust_search::Search,
    targets: &[S],
    log: bool,
) {
    assert_eq!(targets.len(), paths.len());

    for res in search {
        let filename: &str = Path::new(&res)
            .file_name()
            .expect("no file name of path")
            .to_str()
            .unwrap();

        let i = (0..targets.len())
            .find(|&i| *filename == *targets[i])
            .expect("file name didn't match targets");

        if log {
            println!("found: {}", res);
        }
        paths[i].get_or_insert(res);
    }
}

pub fn execute_search<S: Deref<Target = str>>(
    paths: &mut [Option<String>],
    builder: SearchBuilder,
    targets: &[S],
    log: bool,
) {
    let start = Instant::now();

    let search = builder.limit(4096).build();

    if log {
        println!("This search complete: {}s", start.elapsed().as_secs_f64());
    }

    process_results(paths, search, targets, log);

    if log {
        println!(
            "This search and processing complete: {}s",
            start.elapsed().as_secs_f64()
        );
    }
}

/// Preset for searching all files within the user's home directory.
pub fn stage_1(builder: SearchBuilder) -> SearchBuilder {
    builder.hidden().location(directories::home_dir().unwrap())
}

/// Preset for searching from the root with many folders excluded,
/// such as the user's home directory.
pub fn stage_2(builder: SearchBuilder) -> SearchBuilder {
    directories::skip_home(directories::filter_top(directories::relative_filter_name(
        builder,
    )))
    .location(std::path::MAIN_SEPARATOR_STR)
}

/// Preset for searching almost everything excluding the user's home directory.
pub fn stage_3(builder: SearchBuilder) -> SearchBuilder {
    directories::skip_home(directories::filter_top_min(builder))
        .location(std::path::MAIN_SEPARATOR_STR)
        .hidden()
}
