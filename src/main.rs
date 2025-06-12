//! Entry point for the Ethereum PoW implementation

mod evm;

use evm::{EVM, ExecutionResult};
use std::env;

fn main() {
    println!("Simple Ethereum PoW Implementation");
    println!("==================================");
    
    // Example: Create bytecode for a simple program
    // This program computes 3 + 2 = 5
    // PUSH1 0x03, PUSH1 0x02, ADD, STOP
    let bytecode = vec![0x60, 0x03, 0x60, 0x02, 0x01, 0x00];
    
    println!("Executing sample bytecode: PUSH1 0x03, PUSH1 0x02, ADD, STOP");
    let mut evm = EVM::new(bytecode);
    
    match evm.execute() {
        ExecutionResult::Success => {
            println!("Execution successful!");
            // In a real implementation, we'd print the stack result here
            println!("Stack top value should be 5 (0x05)");
        },
        ExecutionResult::Revert(msg) => {
            println!("Execution reverted: {}", msg);
        },
        ExecutionResult::Error(msg) => {
            println!("Execution error: {}", msg);
        },
    }
    
    println!("\nTry running with different bytecode by providing hex as command line argument");
    println!("Example: cargo run -- 600360020100");
    
    // Check if there's a command line argument for custom bytecode
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if let Ok(custom_bytecode) = hex_to_bytes(&args[1]) {
            println!("\nExecuting custom bytecode: {}", args[1]);
            let mut custom_evm = EVM::new(custom_bytecode);
            
            match custom_evm.execute() {
                ExecutionResult::Success => {
                    println!("Execution successful!");
                },
                ExecutionResult::Revert(msg) => {
                    println!("Execution reverted: {}", msg);
                },
                ExecutionResult::Error(msg) => {
                    println!("Execution error: {}", msg);
                },
            }
        } else {
            println!("Invalid hex string for bytecode!");
        }
    }
}

/// Convert a hex string to a byte vector
fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    // Remove 0x prefix if present
    let hex = hex.strip_prefix("0x").unwrap_or(hex);
    
    // Ensure even number of characters
    if hex.len() % 2 != 0 {
        return Err("Hex string must have an even number of characters".to_string());
    }
    
    // Convert to bytes
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for i in (0..hex.len()).step_by(2) {
        let byte_str = &hex[i..i+2];
        match u8::from_str_radix(byte_str, 16) {
            Ok(byte) => bytes.push(byte),
            Err(_) => return Err(format!("Invalid hex characters: {}", byte_str)),
        }
    }
    
    Ok(bytes)
}