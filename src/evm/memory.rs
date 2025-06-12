//! Memory implementation for EVM

/// Simple memory implementation for the EVM
#[derive(Debug, Default)]
pub struct Memory {
    /// The underlying memory storage
    data: Vec<u8>,
}

impl Memory {
    /// Create a new memory instance
    pub fn new() -> Self {
        Memory {
            data: Vec::new(),
        }
    }
    
    /// Store a single byte at the given offset
    pub fn store8(&mut self, offset: usize, value: u8) {
        self.ensure_capacity(offset + 1);
        self.data[offset] = value;
    }
    
    /// Store a 32-byte word starting at the given offset
    pub fn store32(&mut self, offset: usize, value: &[u8; 32]) {
        self.ensure_capacity(offset + 32);
        self.data[offset..offset+32].copy_from_slice(value);
    }
    
    /// Load a single byte from the given offset
    pub fn load8(&self, offset: usize) -> u8 {
        if offset >= self.data.len() {
            return 0;
        }
        self.data[offset]
    }
    
    /// Load a 32-byte word starting at the given offset
    pub fn load32(&self, offset: usize) -> [u8; 32] {
        let mut result = [0u8; 32];
        
        // If offset is beyond memory size, return all zeros
        if offset >= self.data.len() {
            return result;
        }
        
        // Copy as much as possible, leaving zeros for out-of-bounds portions
        let copy_size = std::cmp::min(32, self.data.len() - offset);
        result[..copy_size].copy_from_slice(&self.data[offset..offset+copy_size]);
        
        result
    }
    
    /// Get current memory size in bytes
    pub fn size(&self) -> usize {
        self.data.len()
    }
    
    /// Ensure memory has at least the given capacity
    fn ensure_capacity(&mut self, capacity: usize) {
        if capacity > self.data.len() {
            self.data.resize(capacity, 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_store_load_byte() {
        let mut memory = Memory::new();
        memory.store8(100, 42);
        assert_eq!(memory.load8(100), 42);
        assert_eq!(memory.load8(101), 0); // Out of bounds is zero
    }
    
    #[test]
    fn test_memory_store_load_word() {
        let mut memory = Memory::new();
        let word = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                   17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];
        
        memory.store32(50, &word);
        let loaded = memory.load32(50);
        assert_eq!(loaded, word);
        
        // Partial overlap
        let partial = memory.load32(60);
        assert_eq!(partial[..22], word[10..32]);
        assert_eq!(partial[22..], [0; 10]);
    }
}