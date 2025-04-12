//! The core EVM execution engine

use std::collections::HashMap;
use super::{opcodes::Opcode, stack::Stack, memory::Memory, u256, u64_to_u256};
use super::opcodes::gas_cost;

/// Represents the execution result
#[derive(Debug)]
pub enum ExecutionResult {
    Success,
    Revert(String),
    Error(String),
}

/// Simplified EVM implementation
pub struct EVM {
    /// Program counter
    pc: usize,
    /// Gas remaining
    gas: u64,
    /// EVM stack (last in, first out)
    stack: Stack,
    /// EVM memory (byte-addressable)
    memory: Memory,
    /// EVM storage (key-value store)
    storage: HashMap<u256, u256>,
    /// Contract bytecode
    code: Vec<u8>,
    /// Whether execution has stopped
    stopped: bool,
    /// Valid jump destinations
    jump_destinations: Vec<usize>,
}

impl EVM {
    /// Create a new EVM instance with the given bytecode
    pub fn new(code: Vec<u8>) -> Self {
        let mut evm = EVM {
            pc: 0,
            gas: 1_000_000, // Arbitrary gas limit
            stack: Stack::new(),
            memory: Memory::new(),
            storage: HashMap::new(),
            code,
            stopped: false,
            jump_destinations: Vec::new(),
        };
        
        // Pre-compute valid jump destinations
        evm.compute_jump_destinations();
        
        evm
    }
    
    /// Pre-compute valid jump destinations
    fn compute_jump_destinations(&mut self) {
        let mut pc = 0;
        while pc < self.code.len() {
            let opcode_byte = self.code[pc];
            let opcode = Opcode::from(opcode_byte);
            
            if opcode == Opcode::JUMPDEST {
                self.jump_destinations.push(pc);
            }
            
            // Move to next opcode
            pc += 1;
            
            // Handle PUSH instructions which have additional data
            if opcode == Opcode::PUSH1 {
                pc += 1;
            } else if opcode == Opcode::PUSH2 {
                pc += 2;
            } else if opcode == Opcode::PUSH32 {
                pc += 32;
            }
        }
    }

    /// Execute the bytecode
    pub fn execute(&mut self) -> ExecutionResult {
        while !self.stopped && self.pc < self.code.len() {
            if self.gas == 0 {
                return ExecutionResult::Error("Out of gas".to_string());
            }

            let opcode_byte = self.code[self.pc];
            let opcode = Opcode::from(opcode_byte);
            self.pc += 1;
            
            // Deduct gas for this operation
            let operation_gas = gas_cost(opcode);
            if self.gas < operation_gas {
                return ExecutionResult::Error("Out of gas".to_string());
            }
            self.gas -= operation_gas;
            
            if let Err(msg) = self.execute_opcode(opcode) {
                return ExecutionResult::Error(msg);
            }
        }

        ExecutionResult::Success
    }

    /// Execute a single opcode
    fn execute_opcode(&mut self, opcode: Opcode) -> Result<(), String> {
        match opcode {
            Opcode::STOP => {
                self.stopped = true;
                Ok(())
            },
            Opcode::ADD => {
                let b = self.stack.pop().map_err(|e| format!("ADD: {:?}", e))?;
                let a = self.stack.pop().map_err(|e| format!("ADD: {:?}", e))?;
                
                // Simple implementation - would need proper 256-bit arithmetic
                let mut result = [0u8; 32];
                let mut carry = 0u16;
                
                for i in (0..32).rev() {
                    let sum = a[i] as u16 + b[i] as u16 + carry;
                    result[i] = (sum & 0xFF) as u8;
                    carry = sum >> 8;
                }
                
                self.stack.push(result).map_err(|e| format!("ADD: {:?}", e))?;
                Ok(())
            },
            Opcode::SUB => {
                let b = self.stack.pop().map_err(|e| format!("SUB: {:?}", e))?;
                let a = self.stack.pop().map_err(|e| format!("SUB: {:?}", e))?;
                
                // Simple subtraction implementation
                let mut result = [0u8; 32];
                let mut borrow = false;
                
                for i in (0..32).rev() {
                    let mut diff = a[i] as i16 - b[i] as i16;
                    if borrow {
                        diff -= 1;
                    }
                    if diff < 0 {
                        diff += 256;
                        borrow = true;
                    } else {
                        borrow = false;
                    }
                    result[i] = diff as u8;
                }
                
                self.stack.push(result).map_err(|e| format!("SUB: {:?}", e))?;
                Ok(())
            },
            Opcode::MUL => {
                let b = self.stack.pop().map_err(|e| format!("MUL: {:?}", e))?;
                let a = self.stack.pop().map_err(|e| format!("MUL: {:?}", e))?;
                
                // Very simplified multiplication for educational purposes
                // Only works properly for small values
                let mut result = [0u8; 32];
                
                // Try using our helper from mod.rs if it's available
                result = super::mul_u256(&a, &b);
                
                self.stack.push(result).map_err(|e| format!("MUL: {:?}", e))?;
                Ok(())
            },
            Opcode::DIV => {
                let b = self.stack.pop().map_err(|e| format!("DIV: {:?}", e))?;
                let a = self.stack.pop().map_err(|e| format!("DIV: {:?}", e))?;
                
                // Check for division by zero
                if b.iter().all(|&x| x == 0) {
                    self.stack.push([0u8; 32]).map_err(|e| format!("DIV: {:?}", e))?;
                    return Ok(());
                }
                
                // Simplified division implementation
                if let Some(result) = super::div_u256(&a, &b) {
                    self.stack.push(result).map_err(|e| format!("DIV: {:?}", e))?;
                } else {
                    self.stack.push([0u8; 32]).map_err(|e| format!("DIV: {:?}", e))?;
                }
                
                Ok(())
            },
            Opcode::PUSH1 => {
                if self.pc >= self.code.len() {
                    return Err("PUSH1: Out of bounds".to_string());
                }
                
                let value = self.code[self.pc];
                self.pc += 1;
                
                // Create a full u256 with only the least significant byte set
                let mut bytes = [0u8; 32];
                bytes[31] = value;
                
                self.stack.push(bytes).map_err(|e| format!("PUSH1: {:?}", e))?;
                Ok(())
            },
            Opcode::POP => {
                self.stack.pop().map_err(|e| format!("POP: {:?}", e))?;
                Ok(())
            },
            Opcode::MSTORE => {
                let offset = self.stack.pop().map_err(|e| format!("MSTORE: {:?}", e))?;
                let value = self.stack.pop().map_err(|e| format!("MSTORE: {:?}", e))?;
                
                // Convert offset to usize for memory access
                let offset_val = super::u256_to_u64(&offset)
                    .ok_or_else(|| "MSTORE: Memory offset too large".to_string())?;
                
                self.memory.store32(offset_val as usize, &value);
                Ok(())
            },
            Opcode::MLOAD => {
                let offset = self.stack.pop().map_err(|e| format!("MLOAD: {:?}", e))?;
                
                // Convert offset to usize for memory access
                let offset_val = super::u256_to_u64(&offset)
                    .ok_or_else(|| "MLOAD: Memory offset too large".to_string())?;
                
                let value = self.memory.load32(offset_val as usize);
                self.stack.push(value).map_err(|e| format!("MLOAD: {:?}", e))?;
                Ok(())
            },
            Opcode::JUMP => {
                let dest = self.stack.pop().map_err(|e| format!("JUMP: {:?}", e))?;
                
                // Convert to usize for pc
                let dest_val = super::u256_to_u64(&dest)
                    .ok_or_else(|| "JUMP: Destination too large".to_string())?;
                
                // Check if destination is valid
                if !self.jump_destinations.contains(&(dest_val as usize)) {
                    return Err(format!("JUMP: Invalid destination: {}", dest_val));
                }
                
                self.pc = dest_val as usize;
                Ok(())
            },
            Opcode::JUMPI => {
                let dest = self.stack.pop().map_err(|e| format!("JUMPI: {:?}", e))?;
                let condition = self.stack.pop().map_err(|e| format!("JUMPI: {:?}", e))?;
                
                // Check if condition is non-zero
                let is_non_zero = condition.iter().any(|&b| b != 0);
                
                if is_non_zero {
                    // Convert to usize for pc
                    let dest_val = super::u256_to_u64(&dest)
                        .ok_or_else(|| "JUMPI: Destination too large".to_string())?;
                    
                    // Check if destination is valid
                    if !self.jump_destinations.contains(&(dest_val as usize)) {
                        return Err(format!("JUMPI: Invalid destination: {}", dest_val));
                    }
                    
                    self.pc = dest_val as usize;
                }
                
                Ok(())
            },
            Opcode::JUMPDEST => {
                // JUMPDEST is just a marker, it does nothing during execution
                Ok(())
            },
            _ => {
                // For educational purposes, we're keeping it simple
                Err(format!("Opcode {:?} not implemented yet", opcode))
            }
        }
    }
    
    /// Get the current stack for inspection
    pub fn get_stack(&self) -> &Stack {
        &self.stack
    }
    
    /// Get the current memory for inspection
    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }
    
    /// Get the current storage for inspection
    pub fn get_storage(&self) -> &HashMap<u256, u256> {
        &self.storage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        // Example: PUSH1 0x03, PUSH1 0x02, ADD, STOP
        let bytecode = vec![0x60, 0x03, 0x60, 0x02, 0x01, 0x00];
        let mut evm = EVM::new(bytecode);
        
        match evm.execute() {
            ExecutionResult::Success => {
                // Get the stack
                let stack = evm.get_stack();
                assert_eq!(stack.size(), 1);
                
                // Expected result is 5 (0x05)
                let expected = u64_to_u256(5);
                let result = stack.peek().unwrap();
                assert_eq!(result, &expected);
            },
            _ => panic!("Execution failed"),
        }
    }
    
    #[test]
    fn test_memory_operations() {
        // PUSH1 0x20, PUSH1 0x42, MSTORE, PUSH1 0x20, MLOAD, STOP
        let bytecode = vec![0x60, 0x20, 0x60, 0x42, 0x52, 0x60, 0x20, 0x51, 0x00];
        let mut evm = EVM::new(bytecode);
        
        match evm.execute() {
            ExecutionResult::Success => {
                // Check the stack
                let stack = evm.get_stack();
                assert_eq!(stack.size(), 1);
                
                // Expected result is 0x42 at the end
                let mut expected = [0u8; 32];
                expected[31] = 0x42;
                let result = stack.peek().unwrap();
                assert_eq!(result, &expected);
            },
            _ => panic!("Execution failed"),
        }
    }
}