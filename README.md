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
- ✅ **AES File Decryption** — Decrypts files in a target directory using AES-256-CBC.
- ✅ **Dropped Ransom Note** — Writes a static ransom note to the victim's desktop.
- ✅ **Debug Symbol Stripping** — Debug symbols are removed in release mode for stealth.

---

## ⚙️ How It Works

1. **Sandbox Check**  
   The binary checks for low memory, few CPU cores, known usernames/hostnames, and sleep-skipping.

2. **File Encryption**  
   Files in a specific directory are encrypted using a randomly generated AES-256 key and IV, then dropped them for later decryption.

3. **Ransom Note Drop**  
   A ransom note is created on the desktop.

4. **File Decryption**  
   Files in a specific directory are decrypted using the dropped AES-256 key and IV.

---

## 🧰 Setup & Compilation

### 1. 🧪 Requirements

- OS: Linux
- Toolchain: [Rust](https://www.rust-lang.org/tools/install)
- Build Essentials
```
sudo apt update && sudo apt install build-essential pkg-config libssl-dev -y
```
- Cross-Compiler
```
sudo apt install mingw-w64
```

### 2. 📦 Dependencies

All dependencies are managed via `Cargo.toml`, including:

- `sysinfo`, `hostname`, `whoami`, `chrono` – system inspection
- `aes`, `cbc`, `cipher` – encryption
- `rand`, `dirs` – randomness and directory helpers

### 3. 🔨 Compile (Release Mode)

```bash
git clone https://github.com/yourusername/ransomware-poc.git
cd ransomware-poc

# Compile and strip debug symbols
make
make windows
make linux
```

---

## 🔐 How AES-256-CBC Encryption Works

### 1. AES (Advanced Encryption Standard)

AES is a symmetric block cipher standardized by NIST. AES-256 uses:
 - A 256-bit key (32 bytes)
 - A block size of 128 bits (16 bytes)
 - 14 rounds of substitution, permutation, and mixing steps

At a high level, the algorithm transforms each 16-byte block through a series of steps. Per Round:
* SubBytes – each byte is substituted using an S-box (non-linear transformation)
* ShiftRows – rows in the state matrix are cyclically shifted
* MixColumns – mixes each column using Galois Field math (modular polynomial multiplication in GF(2^8))
* AddRoundKey – XORs the state with a round key derived from the main key using a key schedule

### 2. CBC Mode (Cipher Block Chaining)

CBC mode ensures that identical plaintext blocks don't produce identical ciphertext blocks, by chaining the encryption. So each ciphertext block depends on the previous one — which makes tampering or pattern analysis much harder.

### 3. Padding (PKCS#7)

Since AES requires blocks of exactly 16 bytes, files that aren't a multiple of 16 are padded. In PKCS#7 padding, if 9 bytes are needed, 9 bytes with value 0x09 are added. If the file is already a multiple of 16, a full 16-byte block of 0x10 is added.

### 4. Strength 

This mode is secure as long as the IV is random and the key is protected. But CBC does not provide integrity or authentication, which is why real-world ransomware often combines it with HMAC or AES-GCM (authenticated encryption).

## 🐙 Author

Neyrian ☕🥝