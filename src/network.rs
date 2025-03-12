use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use serde_json;
use crate::blockchain::Blockchain;
use crate::block::Block;

/// Starts a TCP server to handle blockchain synchronization.
pub fn start_server(blockchain: Arc<Mutex<Blockchain>>, port: u16) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Failed to bind port");
    println!("Node listening on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let blockchain = Arc::clone(&blockchain);
                std::thread::spawn(move || handle_connection(stream, blockchain));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

/// Handles incoming requests from peers
fn handle_connection(mut stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
    let mut reader = BufReader::new(&stream);
    let mut request = String::new();
    
    if reader.read_line(&mut request).is_ok() {
        let request = request.trim();
        println!("Received: {}", request);

        let mut blockchain_lock = blockchain.lock().unwrap();

        if request == "GET_BLOCKCHAIN" {
            // Send entire blockchain to peer
            let response = serde_json::to_string(&*blockchain_lock).unwrap();
            stream.write_all(response.as_bytes()).unwrap();
        } else if request.starts_with("NEW_BLOCK") {
            // Peer sent a new block, try adding it
            let block_json = request.replacen("NEW_BLOCK ", "", 1);
            if let Ok(new_block) = serde_json::from_str::<Block>(&block_json) {
                if blockchain_lock.add_block_from_peer(new_block) {
                    println!("✅ Block added successfully!");
                } else {
                    println!("❌ Received invalid block, ignoring...");
                }
            }
        }
    }
}

/// Requests the blockchain from a peer
pub fn request_blockchain(peer: &str, blockchain: Arc<Mutex<Blockchain>>) {
    if let Ok(mut stream) = TcpStream::connect(peer) {
        stream.write_all(b"GET_BLOCKCHAIN\n").unwrap();

        let mut response = String::new();
        let mut reader = BufReader::new(&stream);
        reader.read_line(&mut response).unwrap();

        if let Ok(received_chain) = serde_json::from_str::<Blockchain>(&response) {
            let mut blockchain_lock = blockchain.lock().unwrap();
            if blockchain_lock.replace_chain(received_chain) {
                println!("🔄 Updated blockchain from peer!");
            } else {
                println!("⚠️ Received shorter or invalid chain, ignoring...");
            }
        }
    }
}

/// Broadcasts a newly mined block to a peer
pub fn broadcast_new_block(peer: &str, block: &Block) {
    if let Ok(mut stream) = TcpStream::connect(peer) {
        let message = format!("NEW_BLOCK {}", serde_json::to_string(block).unwrap());
        stream.write_all(message.as_bytes()).unwrap();
    }
}
