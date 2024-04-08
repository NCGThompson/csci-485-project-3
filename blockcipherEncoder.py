from Crypto.Cipher import AES
from Crypto.Util.Padding import pad
import base64

def encrypt_aes_cbc(key, iv, plaintext):
    cipher = AES.new(key, AES.MODE_CBC, iv)
    padded_data = pad(plaintext.encode('utf-8'), AES.block_size)
    ciphertext = cipher.encrypt(padded_data)
    return ciphertext

def main():
    # Replace these values with your plaintext and key
    plaintext = "PLAINTEXT"
    key = b'AES 192 BIT KEY HERE'  # AES-192 key must be 24 bytes long
    iv = b'Intialization VECTOR'  # Initialization Vector (IV) must be 16 bytes long

    # Encrypt the plaintext
    ciphertext = encrypt_aes_cbc(key, iv, plaintext)

    # Encode ciphertext as base64
    encrypted_data_base64 = base64.b64encode(ciphertext).decode('utf-8')

    # Print the base64 encoded ciphertext
    print("Base64 encoded ciphertext:", encrypted_data_base64)

if __name__ == "__main__":
    main()