// basic outline for this program. Hello, Nic and Simeon here is what I have so far.
use libproj3::{decrypt_file, find_files, read_file, send_to_remote_server};
// not sure if a remote server exist got to go back and reread the document.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Find special_file.txt and secret_file.txt
    let (special_file_path, secret_key_path) = find_files()?;

    // Read the secret key
    let mut secret_key: Vec<u8> = vec![];
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
