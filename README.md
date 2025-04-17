# 🛠️ Ransomware Proof of Concept (Rust)

> ⚠️ **For educational and ethical red teaming purposes only.** Do not deploy this code on systems without explicit authorization.

## 📚 Table of Contents
- [Overview](#overview)
- [Features](#features)
- [How It Works](#how-it-works)
- [Setup & Compilation](#setup--compilation)
- [Usage](#usage)
- [Notes for Red Teamers](#notes-for-red-teamers)
- [Disclaimer](#disclaimer)

---

## 📌 Overview

This is a **Windows-compatible ransomware PoC** written in **Rust**, intended for cybersecurity students, red teamers, and malware analysts. The goal is to demonstrate key techniques used in ransomware operations in a safe, contained, and legal environment.

---

## 🔧 Features

- ✅ **Sandbox Detection** — Detects virtual/sandboxed environments using heuristic checks.
- ✅ **AES File Encryption** — Encrypts files in a target directory using AES-256-CBC.
- ✅ **Dropped Ransom Note** — Writes a static ransom note to the victim's desktop.
- ✅ **Debug Symbol Stripping** — Debug symbols are removed in release mode for stealth.

---

## ⚙️ How It Works

1. **Sandbox Check**  
   The binary checks for low memory, few CPU cores, known usernames/hostnames, and sleep-skipping.

2. **File Encryption**  
   Files in a specific directory (`C:\Users\Public\Documents`) are encrypted using a randomly generated AES-256 key and IV.

3. **Ransom Note Drop**  
   A ransom note is created on the desktop with fake BTC address and contact email.

---

## 🧰 Setup & Compilation

### 1. 🧪 Requirements

- OS: Windows
- Toolchain: [Rust](https://www.rust-lang.org/tools/install)
- Build Essentials
```
sudo apt update && sudo apt install build-essential pkg-config libssl-dev -y
```

### 2. 📦 Dependencies

All dependencies are managed via `Cargo.toml`, including:

- `sysinfo`, `hostname`, `whoami` – system inspection
- `reqwest`, `serde_json` – C2 HTTP client
- `aes`, `block-modes` – encryption
- `rand`, `dirs` – randomness and directory helpers

### 3. 🔨 Compile (Release Mode)

```bash
git clone https://github.com/yourusername/ransomware-poc.git
cd ransomware-poc

# Compile and strip debug symbols
cargo build --release
```

---

## 🐙 Author

Neyrian ☕🥝