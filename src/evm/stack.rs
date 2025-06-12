//! Stack implementation for EVM

use super::u256;

/// The EVM stack
#[derive(Debug, Default)]
pub struct Stack {
    /// The underlying stack storage
    items: Vec<u256>,
    /// Maximum stack size
    max_size: usize,
}

/// Error types for stack operations
#[derive(Debug)]
pub enum StackError {
    /// Stack overflow error
    Overflow,
    /// Stack underflow error
    Underflow,
    /// Invalid stack access
    InvalidAccess(String),
}

impl Stack {
    /// Create a new stack with the default max size of 1024
    pub fn new() -> Self {
        Stack {
            items: Vec::new(),
            max_size: 1024,
        }
    }
    
    /// Create a new stack with a custom max size
    pub fn with_capacity(max_size: usize) -> Self {
        Stack {
            items: Vec::with_capacity(max_size),
            max_size,
        }
    }
    
    /// Push a value onto the stack
    pub fn push(&mut self, value: u256) -> Result<(), StackError> {
        if self.items.len() >= self.max_size {
            return Err(StackError::Overflow);
        }
        self.items.push(value);
        Ok(())
    }
    
    /// Pop a value from the stack
    pub fn pop(&mut self) -> Result<u256, StackError> {
        self.items.pop().ok_or(StackError::Underflow)
    }
    
    /// Peek at the top value without popping it
    pub fn peek(&self) -> Result<&u256, StackError> {
        self.items.last().ok_or(StackError::Underflow)
    }
    
    /// Get stack size
    pub fn size(&self) -> usize {
        self.items.len()
    }
    
    /// Duplicate nth item from the top of the stack (0-based)
    /// DUP1 in EVM is dup(0) in this function
    pub fn dup(&mut self, n: usize) -> Result<(), StackError> {
        if n >= self.items.len() {
            return Err(StackError::Underflow);
        }
        
        if self.items.len() >= self.max_size {
            return Err(StackError::Overflow);
        }
        
        let idx = self.items.len() - 1 - n;
        let value = self.items[idx];
        self.items.push(value);
        
        Ok(())
    }
    
    /// Swap 1st and (n+1)th item on the stack
    /// SWAP1 in EVM is swap(1) in this function
    pub fn swap(&mut self, n: usize) -> Result<(), StackError> {
        if n >= self.items.len() || n == 0 {
            return Err(StackError::InvalidAccess(format!("Cannot swap with position {}", n)));
        }
        
        let len = self.items.len();
        self.items.swap(len - 1, len - 1 - n);
        
        Ok(())
    }
    
    /// Get a reference to an item at specified depth from the top (0-based)
    pub fn get(&self, depth: usize) -> Result<&u256, StackError> {
        if depth >= self.items.len() {
            return Err(StackError::Underflow);
        }
        
        Ok(&self.items[self.items.len() - 1 - depth])
    }
    
    /// Clear the stack
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evm::u64_to_u256;
    
    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::new();
        let value = u64_to_u256(42);
        
        // Test push
        assert!(stack.push(value).is_ok());
        assert_eq!(stack.size(), 1);
        
        // Test peek
        assert_eq!(stack.peek().unwrap(), &value);
        assert_eq!(stack.size(), 1);
        
        // Test pop
        let popped = stack.pop().unwrap();
        assert_eq!(popped, value);
        assert_eq!(stack.size(), 0);
        
        // Test underflow
        assert!(stack.pop().is_err());
    }
    
    #[test]
    fn test_stack_dup_swap() {
        let mut stack = Stack::new();
        
        // Push some values
        stack.push(u64_to_u256(1)).unwrap();
        stack.push(u64_to_u256(2)).unwrap();
        stack.push(u64_to_u256(3)).unwrap();
        
        // Test dup
        stack.dup(1).unwrap(); // Duplicate the 2nd item (value 2)
        assert_eq!(stack.peek().unwrap(), &u64_to_u256(2));
        assert_eq!(stack.size(), 4);
        
        // Test swap
        stack.swap(2).unwrap(); // Swap top with 3rd item
        assert_eq!(stack.peek().unwrap(), &u64_to_u256(1));
        
        // Check the swapped item
        stack.pop().unwrap();
        stack.pop().unwrap();
        assert_eq!(stack.peek().unwrap(), &u64_to_u256(2));
    }
}