//! Common cryptographic interfaces for level.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use backend_common::*;
use sha2::{Digest, Sha256};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

/// The number of bytes to use for an AES key.
pub const AES_KEY_SIZE: usize = 32;

/// The number of bytes to use for an AES nonce.
pub const AES_NONCE_SIZE: usize = 12;

/// Encrypts data with AES.
pub fn aes_encrypt(key: &[u8; AES_KEY_SIZE], plaintext: &[u8]) -> CryptoResult<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    let nonce_slice: [u8; AES_NONCE_SIZE] = rand::random();
    let nonce = Nonce::from(nonce_slice);
    let ciphertext = cipher.encrypt(&nonce, plaintext)?;

    let mut ciphertext_with_nonce = nonce_slice.to_vec();
    ciphertext_with_nonce.extend(ciphertext);

    Ok(ciphertext_with_nonce)
}

/// Decrypts data with AES.
pub fn aes_decrypt(
    key: &[u8; AES_KEY_SIZE],
    ciphertext_with_nonce: &[u8],
) -> CryptoResult<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    let (nonce_slice, ciphertext) = ciphertext_with_nonce.split_at(AES_NONCE_SIZE);
    let nonce_slice_sized: [u8; AES_NONCE_SIZE] =
        nonce_slice.try_into().map_err(|_| aes_gcm::Error)?;
    let nonce = Nonce::from(nonce_slice_sized);
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;

    Ok(plaintext)
}

/// Encrypts a file in chunks.
pub async fn encrypt_file(
    src: &mut File,
    dest: &mut File,
    key: &[u8; AES_KEY_SIZE],
) -> CryptoResult<()> {
    let mut buffer = [0u8; READER_CAPACITY];

    loop {
        let n = src.read(&mut buffer).await?;

        if n == 0 {
            break;
        }

        let encrypted_data = aes_encrypt(key, &buffer[..n])?;
        write_section(dest, &encrypted_data).await?;
    }

    dest.rewind().await?;
    dest.flush().await?;

    Ok(())
}

/// Decrypts a file in chunks.
pub async fn decrypt_file(
    src: &mut File,
    dest: &mut File,
    key: &[u8; AES_KEY_SIZE],
) -> CryptoResult<()> {
    loop {
        let data = match read_section(src).await? {
            Some(data) => data,
            None => break,
        };

        let decrypted_data = aes_decrypt(key, &data)?;

        dest.write_all(&decrypted_data).await?;
    }

    dest.rewind().await?;
    dest.flush().await?;

    Ok(())
}

/// Attempts to decrypt a file in chunks, without writing the decrypted data anywhere. Useful for validating the crypto key.
pub async fn try_decrypt_file(src: &mut File, key: &[u8; AES_KEY_SIZE]) -> CryptoResult<()> {
    loop {
        let data = match read_section(src).await? {
            Some(data) => data,
            None => break,
        };

        aes_decrypt(key, &data)?;
    }

    Ok(())
}

/// Convert a password of arbitrary length to an AES key by performing a SHA-256 hash.
pub fn password_to_key(password: &str) -> [u8; AES_KEY_SIZE] {
    let mut hasher = Sha256::new();
    hasher.update(password);
    let result = hasher.finalize();
    result.into()
}

/// Crypto tests.
#[cfg(test)]
mod tests {
    use super::*;
    use rand::{random, thread_rng, Fill};
    use std::io;
    use std::path::Path;
    use tokio::fs::{self, File, OpenOptions};

    const SAVES_DIR: &str = "saves";

    const SAVE_EXT: &str = "level";

    fn rand_range(min: usize, max: usize) -> usize {
        (random::<usize>() % (max - min)) + min
    }

    fn random_save_path() -> String {
        let id: u64 = random();
        let hex_id = format!("{id:x}");
        let root_path = project_root::get_project_root().unwrap();
        let save_path = format!(
            "{}/{}/test_{}.{}",
            root_path.display(),
            SAVES_DIR,
            hex_id,
            SAVE_EXT
        );
        save_path
    }

    async fn create_rw_file(path: impl AsRef<Path>) -> io::Result<File> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .await
    }

    async fn encrypt_decrypt_file(data: &[u8], password: &str) -> (Vec<u8>, Vec<u8>) {
        let key = password_to_key(password);

        let plaintext_path = random_save_path();
        let ciphertext_path = random_save_path();
        let decrypted_path = random_save_path();

        let (ciphertext, plaintext) = {
            let mut plaintext_file = create_rw_file(&plaintext_path).await.unwrap();
            plaintext_file.write_all(data).await.unwrap();
            plaintext_file.rewind().await.unwrap();

            let mut ciphertext_file = create_rw_file(&ciphertext_path).await.unwrap();
            encrypt_file(&mut plaintext_file, &mut ciphertext_file, &key)
                .await
                .unwrap();

            plaintext_file.rewind().await.unwrap();
            let mut plaintext_value = Vec::new();
            plaintext_file
                .read_to_end(&mut plaintext_value)
                .await
                .unwrap();
            plaintext_file.rewind().await.unwrap();
            ciphertext_file.rewind().await.unwrap();
            let mut ciphertext_value = Vec::new();
            ciphertext_file
                .read_to_end(&mut ciphertext_value)
                .await
                .unwrap();
            ciphertext_file.rewind().await.unwrap();

            let mut decrypted_file = create_rw_file(&decrypted_path).await.unwrap();
            decrypt_file(&mut ciphertext_file, &mut decrypted_file, &key)
                .await
                .unwrap();

            decrypted_file.rewind().await.unwrap();
            let mut decrypted_value = Vec::new();
            decrypted_file
                .read_to_end(&mut decrypted_value)
                .await
                .unwrap();
            decrypted_file.rewind().await.unwrap();

            (ciphertext_value, decrypted_value)
        };

        fs::remove_file(&plaintext_path).await.unwrap();
        fs::remove_file(&ciphertext_path).await.unwrap();
        fs::remove_file(&decrypted_path).await.unwrap();

        (ciphertext, plaintext)
    }

    #[test]
    fn test_aes() {
        let aes_message = "Hello, AES!";
        let key = password_to_key("password123");
        let aes_encrypted = aes_encrypt(&key, aes_message.as_bytes()).unwrap();
        let aes_decrypted = aes_decrypt(&key, &aes_encrypted).unwrap();
        let aes_decrypted_message = std::str::from_utf8(&aes_decrypted).unwrap();
        assert_eq!(aes_decrypted_message, aes_message);
        assert_ne!(aes_encrypted, aes_message.as_bytes());
    }

    #[test]
    fn test_encode_section_size() {
        assert_eq!(encode_section_size(0), [0, 0, 0, 0, 0]);
        assert_eq!(encode_section_size(1), [0, 0, 0, 0, 1]);
        assert_eq!(encode_section_size(255), [0, 0, 0, 0, 255]);
        assert_eq!(encode_section_size(256), [0, 0, 0, 1, 0]);
        assert_eq!(encode_section_size(257), [0, 0, 0, 1, 1]);
        assert_eq!(encode_section_size(4311810305), [1, 1, 1, 1, 1]);
        assert_eq!(encode_section_size(4328719365), [1, 2, 3, 4, 5]);
        assert_eq!(encode_section_size(47362409218), [11, 7, 5, 3, 2]);
        assert_eq!(
            encode_section_size(1099511627775),
            [255, 255, 255, 255, 255]
        );
    }

    #[test]
    fn test_decode_section_size() {
        assert_eq!(decode_section_size(&[0, 0, 0, 0, 0]), 0);
        assert_eq!(decode_section_size(&[0, 0, 0, 0, 1]), 1);
        assert_eq!(decode_section_size(&[0, 0, 0, 0, 255]), 255);
        assert_eq!(decode_section_size(&[0, 0, 0, 1, 0]), 256);
        assert_eq!(decode_section_size(&[0, 0, 0, 1, 1]), 257);
        assert_eq!(decode_section_size(&[1, 1, 1, 1, 1]), 4311810305);
        assert_eq!(decode_section_size(&[1, 2, 3, 4, 5]), 4328719365);
        assert_eq!(decode_section_size(&[11, 7, 5, 3, 2]), 47362409218);
        assert_eq!(
            decode_section_size(&[255, 255, 255, 255, 255]),
            1099511627775
        );
    }

    #[tokio::test]
    async fn test_file_encryption() {
        let mut rng = thread_rng();

        let file_message = "Hello, encrypted file!";
        let password = "password123";

        let (ciphertext, plaintext) = encrypt_decrypt_file(file_message.as_bytes(), password).await;
        assert_ne!(&ciphertext, file_message.as_bytes());
        assert_eq!(&plaintext, file_message.as_bytes());
        assert_ne!(plaintext, ciphertext);

        let large_data_size = rand_range(524288, 1048576);
        let mut large_data = vec![0u8; large_data_size];
        large_data.try_fill(&mut rng).unwrap();

        let (ciphertext, plaintext) = encrypt_decrypt_file(&large_data, password).await;
        assert_ne!(ciphertext, large_data);
        assert_eq!(plaintext, large_data);
        assert_ne!(plaintext, ciphertext);
    }

    #[test]
    fn test_password_to_key() {
        let key1 = password_to_key("password123");
        let key2 = password_to_key("password123");
        let key3 = password_to_key("password124");
        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }
}
