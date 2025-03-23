# Build stage
FROM rust:1.81.0 AS builder

# Set the working directory
WORKDIR /app

# Copy Cargo files separately to leverage Docker's cache
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies first
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && cargo build --release

# Copy the actual source code
COPY . .

# Final runtime image
FROM ubuntu:22.04

# Install dependencies
RUN apt-get update && apt-get install -y curl netcat build-essential && rm -rf /var/lib/apt/lists/*

# Install Rust in the runtime container
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory
WORKDIR /app

# Copy the source code again for cargo run
COPY . .

# Expose ports for communication
EXPOSE 6000-6001

# Set default environment variables
ENV NODE_PORT=6000
ENV PEER_PORT=6001

# Run the app using cargo run
CMD [ "cargo", "run" ]
