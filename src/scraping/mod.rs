mod tests;

use rust_search::SearchBuilder;
use std::{ops::Deref, path::PathBuf};

/// Searches for the files we need, `special_file.txt` and `secret_file.txt`
/// and returns there locations if found each as a `PathBuf` in the order listed here.
/// This uses the library `rust_search` which indirectly wraps `walkdir`.
///
/// `scrape()` will always search the running users home directory if possible.
/// Iff it still hasn't found one or more of the files, and the `sudoer`
/// feature is enabled, it will search the root directory next.
///
/// This function is optimized for Ubuntu because that is the target,
/// but ideally it should be cross platform so anyone can easily test it on their
/// local machine.
pub fn find_files() -> Result<(PathBuf, PathBuf), String> {
    let search = rust_search::SearchBuilder::default()
        .search_input(r"(?:special|secret)_file")
        .ext(r"txt")
        .location(r"~")
        .strict()
        .build();

    let paths_vec: Vec<String> = search.collect();

    let mut special: Option<PathBuf> = None;
    let mut secret: Option<PathBuf> = None;
    for path_string in paths_vec {
        let path = PathBuf::from(path_string);
        // This should be a cheap conversion, see std/ffi/struct.OsString.html
        // It's useful because ends_with behaves differently for &str than it does for &Path

        assert!(path.ends_with("special_file.txt") || path.ends_with("secret_file.txt"));

        if special.is_none() && path.ends_with("special_file.txt") {
            special = Some(path);
        } else if secret.is_none() && path.ends_with("secret_file.txt") {
            secret = Some(path);
        }
        #[cfg(not(debug_assertions))]
        // cfg so the whole list gets checked by above assert
        if special.is_some() && secret.is_some() {
            break;
        }
    }

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
