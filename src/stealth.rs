use std::{thread, time::Duration};
use sysinfo::System;
use whoami;
use hostname;
use chrono::Utc;

/// Checks for sandbox indicators.
pub fn is_sandbox() -> bool {
    let sys = System::new_all();

    // Check for low CPU core count.
    if sys.physical_core_count().unwrap_or(0) < 2 {
        return true;
    }

    // Check for low total memory (<2GB).
    if sys.total_memory() < 2 * 1024 * 1024 {
        return true;
    }

    // Check for suspicious hostnames.
    let hostname = hostname::get().unwrap_or_default().to_string_lossy().to_lowercase();
    if ["sandbox", "maltest", "viruslab", "win10test"].iter().any(|&h| hostname.contains(h)) {
        return true;
    }

    // Check for suspicious usernames.
    let username = whoami::username().to_lowercase();
    if ["admin", "sandbox", "malware"].iter().any(|&u| username.contains(u)) {
        return true;
    }

    // Detect sleep skipping (common in sandboxes).
    let start = std::time::Instant::now();
    thread::sleep(Duration::from_secs(2));
    if start.elapsed().as_secs() < 2 {
        return true;
    }

    false
}