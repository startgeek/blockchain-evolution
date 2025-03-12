mod block;
mod blockchain;
mod network;
use std::env;

use blockchain::Blockchain;
use network::{start_server, request_blockchain, broadcast_new_block};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;



fn main() {
    let port = env::var("NODE_PORT").unwrap_or_else(|_| "6000".to_string());
    let peer_port = env::var("PEER_PORT").unwrap_or_else(|_| "6001".to_string());

    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    // Start a TCP server in a separate thread
    let blockchain_clone = Arc::clone(&blockchain);
    let port_clone = port.clone();
    thread::spawn(move || {
        start_server(blockchain_clone, port_clone.parse().unwrap());
    });

    // Wait for peers to start
    thread::sleep(Duration::from_secs(2));

    // Sync blockchain with another peer
    request_blockchain(&format!("127.0.0.1:{}", peer_port), Arc::clone(&blockchain));

    // Simulate mining a block and broadcasting it
    {
        let mut bc = blockchain.lock().unwrap();
        let new_block = bc.add_block("New transaction!".to_string());
        broadcast_new_block(&format!("127.0.0.1:{}", peer_port), &new_block);
    }

    loop {}
}