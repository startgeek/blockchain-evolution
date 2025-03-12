# Blockchain Evolution in Rust

## Overview
This repository serves as an **educational resource** for understanding **distributed systems, the Rust programming language, and the historical evolution of blockchain technology**. It is designed to provide a hands-on experience in building a simplified blockchain network with **Proof of Work (PoW)**. Nothing is better than **learning by doing**, and this project allows you to **practice real blockchain concepts in code**, ensuring a deep and practical understanding.

## Features
- A simplified **Proof of Work (PoW)** mechanism inspired by Bitcoin.
- A **peer-to-peer network** allowing nodes to sync and communicate.
- A **local testing environment** with two nodes for experimentation.
- Open for **community contributions**—new developments in blockchain can be added through pull requests!

## Getting Started
### Prerequisites
Ensure you have the following installed:
- **Rust** (Install via [rustup](https://rustup.rs/))
- **Cargo** (Rust’s package manager)
- **Git** (for cloning the repository)

### Installation & Running
Clone the repository and build the project:
```sh
git clone https://github.com/yourusername/blockchain-evolution.git
cd blockchain-evolution
cargo build
```

To start the first node:

```sh
cargo run -- --port 6000
```

To start a second node:

```sh
cargo run -- --port 6001
```
The nodes will sync and validate blocks between each other.

### Contributing

This project is open-source and welcomes contributions! If a new hot topic in blockchain emerges, feel free to create a pull request and add your work.

### Why This Approach?

The best way to truly understand blockchain is by building it from scratch. Reading whitepapers is great, but coding and experimenting with real implementations is where the true learning happens. This repository gives you the practical experience needed to understand the inner workings of blockchain.