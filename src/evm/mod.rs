//! Ethereum Virtual Machine module

pub mod opcodes;
pub mod executor;
pub mod memory;
pub mod stack;

// Re-export the main components
pub use executor::EVM;
pub use executor::ExecutionResult;
pub use opcodes::Opcode;

/// Simple 256-bit unsigned integer to represent Ethereum values
/// In a real implementation we'd use a proper 256-bit library
pub type u256 = [u8; 32];

/// Convert a u64 to a u256
pub fn u64_to_u256(value: u64) -> u256 {
    let mut result = [0u8; 32];
    let bytes = value.to_be_bytes();
    result[24..32].copy_from_slice(&bytes);
    result
}

/// Try to convert a u256 to a u64 if it fits
pub fn u256_to_u64(value: &u256) -> Option<u64> {
    // Check if the value fits in u64 (first 24 bytes must be 0)
    if value[..24] != [0u8; 24] {
        return None;
    }
    
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&value[24..32]);
    Some(u64::from_be_bytes(bytes))
}

/// Add two u256 values
pub fn add_u256(a: &u256, b: &u256) -> u256 {
    let mut result = [0u8; 32];
    let mut carry = 0u16;
    
    for i in (0..32).rev() {
        let sum = a[i] as u16 + b[i] as u16 + carry;
        result[i] = (sum & 0xFF) as u8;
        carry = sum >> 8;
    }
    
    result
}

/// Subtract b from a (a - b)
pub fn sub_u256(a: &u256, b: &u256) -> u256 {
    let mut result = [0u8; 32];
    let mut borrow = 0i16;
    
    for i in (0..32).rev() {
        let mut diff = a[i] as i16 - b[i] as i16 - borrow;
        if diff < 0 {
            diff += 256;
            borrow = 1;
        } else {
            borrow = 0;
        }
        result[i] = diff as u8;
    }
    
    result
}

/// Multiply two u256 values (simplified implementation)
pub fn mul_u256(a: &u256, b: &u256) -> u256 {
    // This is a very simplified implementation that only works properly
    // for relatively small values. A real implementation would need
    // a proper bigint library.
    
    // Try to convert to u64 for simplicity
    if let (Some(a_val), Some(b_val)) = (u256_to_u64(a), u256_to_u64(b)) {
        // If they fit in u64, multiply them (checking for overflow)
        if let Some(result) = a_val.checked_mul(b_val) {
            return u64_to_u256(result);
        }
    }
    
    // Fallback to a more complicated algorithm for larger numbers
    // (This is a placeholder - a real implementation would need more)
    let mut result = [0u8; 32];
    // ... complex multiplication logic would go here ...
    
    result
}

/// Divide a by b (a / b)
pub fn div_u256(a: &u256, b: &u256) -> Option<u256> {
    // Check for division by zero
    if b.iter().all(|&x| x == 0) {
        return None;
    }
    
    // Try to convert to u64 for simplicity
    if let (Some(a_val), Some(b_val)) = (u256_to_u64(a), u256_to_u64(b)) {
        return Some(u64_to_u256(a_val / b_val));
    }
    
    // Fallback to a more complicated algorithm for larger numbers
    // (This is a placeholder - a real implementation would need more)
    let mut result = [0u8; 32];
    // ... complex division logic would go here ...
    
    Some(result)
}

/// Compare if a is less than b
pub fn lt_u256(a: &u256, b: &u256) -> bool {
    for i in 0..32 {
        if a[i] < b[i] {
            return true;
        }
        if a[i] > b[i] {
            return false;
        }
    }
    false // They're equal
}

/// Bitwise AND operation
pub fn and_u256(a: &u256, b: &u256) -> u256 {
    let mut result = [0u8; 32];
    for i in 0..32 {
        result[i] = a[i] & b[i];
    }
    result
}

/// Bitwise OR operation
pub fn or_u256(a: &u256, b: &u256) -> u256 {
    let mut result = [0u8; 32];
    for i in 0..32 {
        result[i] = a[i] | b[i];
    }
    result
}

/// Bitwise XOR operation
pub fn xor_u256(a: &u256, b: &u256) -> u256 {
    let mut result = [0u8; 32];
    for i in 0..32 {
        result[i] = a[i] ^ b[i];
    }
    result
}

/// Bitwise NOT operation
pub fn not_u256(a: &u256) -> u256 {
    let mut result = [0u8; 32];
    for i in 0..32 {
        result[i] = !a[i];
    }
    result
}

/// Compare if a equals b
pub fn eq_u256(a: &u256, b: &u256) -> bool {
    a == b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u64_conversion() {
        let original = 12345678u64;
        let as_u256 = u64_to_u256(original);
        let back_to_u64 = u256_to_u64(&as_u256).unwrap();
        assert_eq!(original, back_to_u64);
    }

    #[test]
    fn test_u256_addition() {
        let a = u64_to_u256(500);
        let b = u64_to_u256(300);
        let result = add_u256(&a, &b);
        assert_eq!(u256_to_u64(&result).unwrap(), 800);
    }

    #[test]
    fn test_u256_subtraction() {
        let a = u64_to_u256(500);
        let b = u64_to_u256(300);
        let result = sub_u256(&a, &b);
        assert_eq!(u256_to_u64(&result).unwrap(), 200);
    }
}