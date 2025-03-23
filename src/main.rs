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
    // Get environment variables (fallback values for standalone mode)
    let node_port = env::var("NODE_PORT").unwrap_or_else(|_| "6000".to_string());
    let peer_host = env::var("PEER_HOST").unwrap_or_else(|_| "".to_string());  // Empty if no peer
    let peer_port = env::var("PEER_PORT").unwrap_or_else(|_| "6000".to_string());

    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    // Start a TCP server in a separate thread
    let blockchain_clone = Arc::clone(&blockchain);
    let node_port_clone = node_port.clone();
    thread::spawn(move || {
        start_server(blockchain_clone, node_port_clone.parse().unwrap());
    });

    // Wait for the network to stabilize
    thread::sleep(Duration::from_secs(2));

    // Sync blockchain with a peer if provided
    if !peer_host.is_empty() {
        let peer_address = format!("{}:{}", peer_host, peer_port);
        println!("Attempting to sync with peer at {}", peer_address);
        request_blockchain(&peer_address, Arc::clone(&blockchain));

        // Simulate mining a block and broadcasting it
        {
            let mut bc = blockchain.lock().unwrap();
            let new_block = bc.add_block("New transaction!".to_string());
            println!("New block mined: {:?}", new_block);
            broadcast_new_block(&peer_address, &new_block);
        }
    } else {
        println!("No peer specified. Running as standalone node.");
    }

    // Keep node running without consuming 100% CPU
    loop {
        thread::sleep(Duration::from_secs(5));
    }
}
