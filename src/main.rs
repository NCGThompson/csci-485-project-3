// basic outline for this program. Hello, Nic and Simeon here is what I have so far.
use libproj3::{decrypt_file, find_file, read_file, send_to_remote_server};
//not sure if a remote server exist got to go back and reread the document.
const SPECIAL_FILE_NAME: &str = "special_file.txt";
const SECRET_FILE_NAME: &str = "secret_file.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Find special_file.txt and secret_file.txt
    let special_file_path = find_file(SPECIAL_FILE_NAME)?;
    let secret_key_path = find_file(SECRET_FILE_NAME)?;

    // Read the secret key
    let mut secret_key: Vec<u8> = vec!();
    read_file(&secret_key_path, &mut secret_key)?;

    // Read the encrypted file
    let mut encrypted_data = Vec::new();
    read_file(&special_file_path, &mut encrypted_data)?;

    // Decrypt the encrypted data
    let decrypted_data = decrypt_file(&encrypted_data, &secret_key)?;

    // Send decrypted data to remote server
    send_to_remote_server(&decrypted_data)?;

    print!("Hello Nic and Simeon!");

    Ok(())
}

// wiggle sauce my goats
