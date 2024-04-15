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
/// This function is optimized for Ubuntu because that is the target,
/// but ideally it should be cross platform so anyone can easily test it on their
/// local machine.
pub fn find_files() -> Result<(String, String), String> {
    let search = rust_search::SearchBuilder::default()
        .search_input(r"(?:special|secret)_file")
        .ext(r"txt")
        .location(r"~")
        .strict()
        .build();

    let targets = ["special_file.txt", "secret_file.txt"];
    let mut paths = [None, None];

    process_results(&mut paths, search, &targets, false, cfg!(debug_assertions));

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
///
/// This is optimized for `Search` to be blocking like
/// [`Reciever.into_iter()`](std::sync::mpsc::IntoIter) and return results in real time,
/// but it turns out not to be the case, at least at this scale.
fn process_results<S: Deref<Target = str>>(
    paths: &mut [Option<String>],
    search: rust_search::Search,
    targets: &[S],
    log: bool,
    find_all: bool,
) {
    let start = Instant::now();

    assert_eq!(targets.len(), paths.len());

    for res in search {
        if log {
            println!("{}s: {}", start.elapsed().as_secs_f64(), res);
        }
        let filename: &str = Path::new(&res)
            .file_name()
            .expect("no file name of path")
            .to_str()
            .unwrap();

        let i = (0..targets.len())
            .find(|&i| *filename == *targets[i])
            .expect("file name didn't match targets");
        paths[i].get_or_insert(res);

        if !find_all && paths.iter().all(Option::is_some) {
            return;
        }
    }
}
