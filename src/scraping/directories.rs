use std::path::Path;

use rust_search::{FilterExt as _, SearchBuilder};

/// We use the [`dirs`] crate because it is used by [`rust_search`] internally.
pub use dirs::home_dir;

#[cfg(unix)]
static SKIPED_DIRECTORY_NAMES: [&str; 4] = ["bin", "lib", "sbin", "usr"];

#[cfg(all(unix, not(target_os = "macos")))]
static SKIPED_PATHS: [&str; 4] = ["boot", "dev", "opt", "tmp"];

#[cfg(target_os = "macos")]
static SKIPED_PATHS: [&str; 6] = ["System", "Volumes", "boot", "dev", "opt", "tmp"];

#[cfg(windows)]
static SKIPED_PATHS: [&str; 3] = ["Program Files", "Program Files (x86)", "Windows"];

#[cfg(all(unix, not(target_os = "macos")))]
static ALWAYS_SKIPED_PATHS: [&str; 1] = ["dev"];

#[cfg(target_os = "macos")]
static ALWAYS_SKIPED_PATHS: [&str; 3] = ["System", "Volumes", "dev"];

#[cfg(all(unix, not(target_os = "macos")))]
static USERS: &str = "home";

#[cfg(any(target_os = "macos", windows))]
static USERS: &str = "Users";

pub(super) fn relative_filter_name(builder: SearchBuilder) -> SearchBuilder {
    #[cfg(unix)]
    {
        assert!(is_sorted(&SKIPED_DIRECTORY_NAMES));

        builder.custom_filter(|dir| {
            if dir.metadata().unwrap().is_dir() {
                if let Some(name) = dir.file_name().to_str() {
                    SKIPED_DIRECTORY_NAMES.binary_search(&name).is_err()
                } else {
                    true
                }
            } else {
                true
            }
        })
    }

    #[cfg(not(unix))]
    builder
}

pub(super) fn filter_top(builder: SearchBuilder) -> SearchBuilder {
    #[cfg(any(unix, windows))]
    {
        assert!(is_sorted(&SKIPED_PATHS));

        builder.custom_filter(|dir| {
            if dir.depth() <= 1 {
                if let Some(name) = dir.file_name().to_str() {
                    SKIPED_PATHS.binary_search(&name).is_err()
                } else {
                    true
                }
            } else {
                true
            }
        })
    }

    #[cfg(not(any(unix, windows)))]
    builder
}

pub(super) fn filter_top_min(builder: SearchBuilder) -> SearchBuilder {
    #[cfg(unix)]
    {
        #[cfg(unix)]
        assert!(is_sorted(&ALWAYS_SKIPED_PATHS));

        builder.custom_filter(|dir| {
            if dir.depth() <= 1 {
                if let Some(name) = dir.file_name().to_str() {
                    ALWAYS_SKIPED_PATHS.binary_search(&name).is_err()
                } else {
                    true
                }
            } else {
                true
            }
        })
    }

    #[cfg(not(unix))]
    builder
}

/// This modifies the builder so that it will skip the user's home
/// directory for when it has already been checked.
///
/// To be avoid be effecient while avoiding [`OnceLock`](std::sync::OnceLock),
/// we first check that the visited directory is a direct child of
/// [`USERS`], and only then do we call [`home_dir`].
pub(super) fn skip_home(builder: SearchBuilder) -> SearchBuilder {
    #[cfg(unix)]
    {
        builder.custom_filter(|dir| {
            dir.path().parent().map(|x| x.strip_prefix("/")) != Some(Ok(Path::new(USERS)))
                || Some(dir.path()) != home_dir().as_deref()
        })
    }

    #[cfg(windows)]
    {
        builder.custom_filter(|dir| {
            !dir.path()
                .parent()
                .map_or(false, |x| x.ends_with(Path::new(USERS)))
                || Some(dir.path()) != home_dir().as_deref()
        })
    }

    #[cfg(not(any(unix, windows)))]
    builder
}

fn is_sorted<O: PartialOrd>(checked_slice: &[O]) -> bool {
    checked_slice.windows(2).all(|x| x[0] <= x[1])
}
