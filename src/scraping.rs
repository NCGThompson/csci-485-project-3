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
            special = Some(path.into());
        } else if secret.is_none() && path.ends_with("secret_file.txt") {
            secret = Some(path.into());
        }
        if special.is_some() && secret.is_some() {
            break;
        }
    }

    Ok((special.ok_or("No Special")?, secret.ok_or("No Secret")?))
}
