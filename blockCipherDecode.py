from Crypto.Cipher import AES
from Crypto.Util.Padding import unpad
import base64

def decrypt_aes_cbc(key, iv, ciphertext):
    cipher = AES.new(key, AES.MODE_CBC, iv)
    decrypted_data = cipher.decrypt(ciphertext)
    return unpad(decrypted_data, AES.block_size)

def main():
    # Input values for our key and initialization vector
    encrypted_data_base64 = "YOUR_BASE64_ENCODED_CIPHERTEXT_HERE"
    key = b'YOUR_AES_192_BIT_KEY_HERE'  # AES-192 key must be 24 bytes long
    iv = b'YOUR_INITIALIZATION_VECTOR_HERE'  # Initialization Vector (IV) must be 16 bytes long

    # Decode base64 encoded ciphertext
    ciphertext = base64.b64decode(encrypted_data_base64)

    # Decrypt the ciphertext
    decrypted_data = decrypt_aes_cbc(key, iv, ciphertext)

    # Convert decrypted data to string 
    plaintext = decrypted_data.decode('utf-8')

    # Print the decrypted plaintext
    print("Decrypted plaintext:", plaintext)

if __name__ == "__main__":
    main()