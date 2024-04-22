use std::io;

use aes::{
    cipher::{block_padding::Pkcs7, BlockDecryptMut as _, KeyIvInit as _},
    Aes192,
};
use cbc;

mod tests;

/// Decrypts a file containing AES-192 CBC encrypted data.
///
/// # Errors
///
/// This function will return an error if the file cannot be read or if decryption fails.
pub fn decrypt_file(ciphertext: &[u8], key: &[u8]) -> io::Result<Vec<u8>> {
    let iv = [0u8; 16]; // initialization vector

    let cipher = cbc::Decryptor::<Aes192>::new_from_slices(key, &iv)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    cipher
        .decrypt_padded_vec_mut::<Pkcs7>(&ciphertext)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
