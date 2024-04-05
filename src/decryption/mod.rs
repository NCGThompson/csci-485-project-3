mod tests;
pub mod unpad;

use openssl::symm;

pub fn decrypt_file(data: &[u8], key: &[u8]) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let cipher = symm::Cipher::aes_256_cbc();
    let decrypted_data = symm::decrypt(cipher, key, None, data)?;
    Ok(decrypted_data)
}
