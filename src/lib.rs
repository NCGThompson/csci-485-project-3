pub mod decryption;
pub mod posting;
pub mod scraping;

use std::fs;
use std::io::{self, Read as _};
use std::path::PathBuf;
//not sure if a remote server exist got to go back and reread the document.

pub use decryption::decrypt_file;
pub use posting::send_to_remote_server;
pub use scraping::find_files;

pub fn read_file(file_path: &PathBuf, buffer: &mut Vec<u8>) -> Result<(), io::Error> {
    let mut file = fs::File::open(file_path)?;
    file.read_to_end(buffer)?;
    Ok(())
}
