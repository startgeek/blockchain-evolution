# Lesson 3: Ethereum Wallet Generator (CLI)

This lesson introduces the fundamentals of Ethereum identity by building a simple command-line tool in Rust that generates a new Ethereum wallet. It shows how private keys, public keys, and Ethereum addresses are derived and interconnected using cryptographic principles.

---

## 🧠 What This Lesson Teaches

- How Ethereum wallet keypairs are generated
- How to derive an Ethereum address from a public key using Keccak-256
- How to build and run a CLI tool in Rust
- The structure of an Ethereum wallet: 
  - **Private Key**: 32-byte random number
  - **Public Key**: Derived from the private key using secp256k1
  - **Address**: Last 20 bytes of Keccak-256 hash of the uncompressed public key

---

## 🛠️ How to Run


cargo run --bin wallet 