mod tests;

use rust_search::SearchBuilder;
use std::path::PathBuf;

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

pub fn multiple_searches_raw<'a, I: IntoIterator<Item = &'a str>>(
    builder: SearchBuilder,
    targets: I,
) -> SearchBuilder {
    let targets: Vec<&str> = targets.into_iter().collect();
    let input = concatenate_targets(&targets);

    builder.strict().search_input(input).ext("{0}")
}

pub fn multiple_searches<'a, I: IntoIterator<Item = &'a str>>(
    builder: SearchBuilder,
    targets: I,
) -> SearchBuilder {
    let targets: Vec<String> = targets.into_iter().map(regex::escape).collect();
    let target_refs: Vec<&str> = targets.iter().map(|x| &**x).collect();

    multiple_searches_raw(builder, target_refs)
}

fn concatenate_targets(targets: &[&str]) -> String {
    if targets.len() <= 1 {
        return String::from(targets.first().copied().unwrap_or_default());
    }

    let length = 3 + targets.len() + targets.iter().map(|x| x.len()).sum::<usize>();
    let mut input = String::with_capacity(length);

    let mut targets = targets.iter().copied();
    input.push_str(r"(?:");
    input.push_str(targets.next().unwrap_or_default());
    for target in targets {
        input.push('|');
        input.push_str(target);
    }
    input.push(')');

    input
}
