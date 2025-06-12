//! Opcode definitions for the simplified EVM

/// Represents the available operations in our simplified EVM
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    STOP,       // 0x00: Halts execution
    ADD,        // 0x01: Addition operation
    SUB,        // 0x02: Subtraction operation
    MUL,        // 0x03: Multiplication operation
    DIV,        // 0x04: Integer division operation
    SDIV,       // 0x05: Signed integer division operation
    MOD,        // 0x06: Modulo operation
    ADDMOD,     // 0x08: Modulo addition operation
    MULMOD,     // 0x09: Modulo multiplication operation
    LT,         // 0x10: Less-than comparison
    GT,         // 0x11: Greater-than comparison
    EQ,         // 0x14: Equality comparison
    ISZERO,     // 0x15: Simple not operator
    AND,        // 0x16: Bitwise AND operation
    OR,         // 0x17: Bitwise OR operation
    XOR,        // 0x18: Bitwise XOR operation
    NOT,        // 0x19: Bitwise NOT operation
    BYTE,       // 0x1A: Retrieve single byte from word
    SHL,        // 0x1B: Shift left
    SHR,        // 0x1C: Logical shift right
    SAR,        // 0x1D: Arithmetic shift right
    SHA3,       // 0x20: Compute Keccak-256 hash
    ADDRESS,    // 0x30: Get address of currently executing account
    BALANCE,    // 0x31: Get balance of the given account
    ORIGIN,     // 0x32: Get execution origination address
    CALLER,     // 0x33: Get caller address
    CALLVALUE,  // 0x34: Get deposited value by the instruction/transaction
    CALLDATALOAD, // 0x35: Get input data of current environment
    CALLDATASIZE, // 0x36: Get size of input data in current environment
    CALLDATACOPY, // 0x37: Copy input data in current environment to memory
    CODESIZE,   // 0x38: Get size of code running in current environment
    CODECOPY,   // 0x39: Copy code running in current environment to memory
    GASPRICE,   // 0x3A: Get price of gas in current environment
    POP,        // 0x50: Remove item from stack
    MLOAD,      // 0x51: Load word from memory
    MSTORE,     // 0x52: Save word to memory
    MSTORE8,    // 0x53: Save byte to memory
    SLOAD,      // 0x54: Load word from storage
    SSTORE,     // 0x55: Save word to storage
    JUMP,       // 0x56: Alter the program counter
    JUMPI,      // 0x57: Conditionally alter the program counter
    PC,         // 0x58: Get the value of the program counter
    MSIZE,      // 0x59: Get the size of active memory
    GAS,        // 0x5A: Get the amount of available gas
    JUMPDEST,   // 0x5B: Mark a valid jump destination
    PUSH1,      // 0x60: Place 1 byte item on stack
    PUSH2,      // 0x61: Place 2 byte item on stack
    PUSH32,     // 0x7F: Place 32 byte item on stack
    DUP1,       // 0x80: Duplicate 1st stack item
    DUP2,       // 0x81: Duplicate 2nd stack item
    SWAP1,      // 0x90: Exchange 1st and 2nd stack items
    SWAP2,      // 0x91: Exchange 1st and 3rd stack items
    LOG0,       // 0xA0: Append log record with no topics
    CREATE,     // 0xF0: Create a new account with associated code
    CALL,       // 0xF1: Message-call into an account
    RETURN,     // 0xF3: Halt execution returning output data
    REVERT,     // 0xFD: Halt execution reverting state changes
    INVALID,    // 0xFE: Invalid instruction
    SELFDESTRUCT, // 0xFF: Halt execution and register account for deletion
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Opcode::STOP,
            0x01 => Opcode::ADD,
            0x02 => Opcode::SUB,
            0x03 => Opcode::MUL,
            0x04 => Opcode::DIV,
            0x05 => Opcode::SDIV,
            0x06 => Opcode::MOD,
            0x08 => Opcode::ADDMOD,
            0x09 => Opcode::MULMOD,
            0x10 => Opcode::LT,
            0x11 => Opcode::GT,
            0x14 => Opcode::EQ,
            0x15 => Opcode::ISZERO,
            0x16 => Opcode::AND,
            0x17 => Opcode::OR,
            0x18 => Opcode::XOR,
            0x19 => Opcode::NOT,
            0x1A => Opcode::BYTE,
            0x1B => Opcode::SHL,
            0x1C => Opcode::SHR,
            0x1D => Opcode::SAR,
            0x20 => Opcode::SHA3,
            0x30 => Opcode::ADDRESS,
            0x31 => Opcode::BALANCE,
            0x32 => Opcode::ORIGIN,
            0x33 => Opcode::CALLER,
            0x34 => Opcode::CALLVALUE,
            0x35 => Opcode::CALLDATALOAD,
            0x36 => Opcode::CALLDATASIZE,
            0x37 => Opcode::CALLDATACOPY,
            0x38 => Opcode::CODESIZE,
            0x39 => Opcode::CODECOPY,
            0x3A => Opcode::GASPRICE,
            0x50 => Opcode::POP,
            0x51 => Opcode::MLOAD,
            0x52 => Opcode::MSTORE,
            0x53 => Opcode::MSTORE8,
            0x54 => Opcode::SLOAD,
            0x55 => Opcode::SSTORE,
            0x56 => Opcode::JUMP,
            0x57 => Opcode::JUMPI,
            0x58 => Opcode::PC,
            0x59 => Opcode::MSIZE,
            0x5A => Opcode::GAS,
            0x5B => Opcode::JUMPDEST,
            0x60 => Opcode::PUSH1,
            0x61 => Opcode::PUSH2,
            0x7F => Opcode::PUSH32,
            0x80 => Opcode::DUP1,
            0x81 => Opcode::DUP2,
            0x90 => Opcode::SWAP1,
            0x91 => Opcode::SWAP2,
            0xA0 => Opcode::LOG0,
            0xF0 => Opcode::CREATE,
            0xF1 => Opcode::CALL,
            0xF3 => Opcode::RETURN,
            0xFD => Opcode::REVERT,
            0xFE => Opcode::INVALID,
            0xFF => Opcode::SELFDESTRUCT,
            _ => Opcode::INVALID,
        }
    }
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        match self {
            Opcode::STOP => 0x00,
            Opcode::ADD => 0x01,
            Opcode::SUB => 0x02,
            Opcode::MUL => 0x03,
            Opcode::DIV => 0x04,
            Opcode::SDIV => 0x05,
            Opcode::MOD => 0x06,
            Opcode::ADDMOD => 0x08,
            Opcode::MULMOD => 0x09,
            Opcode::LT => 0x10,
            Opcode::GT => 0x11,
            Opcode::EQ => 0x14,
            Opcode::ISZERO => 0x15,
            Opcode::AND => 0x16,
            Opcode::OR => 0x17,
            Opcode::XOR => 0x18,
            Opcode::NOT => 0x19,
            Opcode::BYTE => 0x1A,
            Opcode::SHL => 0x1B,
            Opcode::SHR => 0x1C,
            Opcode::SAR => 0x1D,
            Opcode::SHA3 => 0x20,
            Opcode::ADDRESS => 0x30,
            Opcode::BALANCE => 0x31,
            Opcode::ORIGIN => 0x32,
            Opcode::CALLER => 0x33,
            Opcode::CALLVALUE => 0x34,
            Opcode::CALLDATALOAD => 0x35,
            Opcode::CALLDATASIZE => 0x36,
            Opcode::CALLDATACOPY => 0x37,
            Opcode::CODESIZE => 0x38,
            Opcode::CODECOPY => 0x39,
            Opcode::GASPRICE => 0x3A,
            Opcode::POP => 0x50,
            Opcode::MLOAD => 0x51,
            Opcode::MSTORE => 0x52,
            Opcode::MSTORE8 => 0x53,
            Opcode::SLOAD => 0x54,
            Opcode::SSTORE => 0x55,
            Opcode::JUMP => 0x56,
            Opcode::JUMPI => 0x57,
            Opcode::PC => 0x58,
            Opcode::MSIZE => 0x59,
            Opcode::GAS => 0x5A,
            Opcode::JUMPDEST => 0x5B,
            Opcode::PUSH1 => 0x60,
            Opcode::PUSH2 => 0x61,
            Opcode::PUSH32 => 0x7F,
            Opcode::DUP1 => 0x80,
            Opcode::DUP2 => 0x81,
            Opcode::SWAP1 => 0x90,
            Opcode::SWAP2 => 0x91,
            Opcode::LOG0 => 0xA0,
            Opcode::CREATE => 0xF0,
            Opcode::CALL => 0xF1,
            Opcode::RETURN => 0xF3,
            Opcode::REVERT => 0xFD,
            Opcode::INVALID => 0xFE,
            Opcode::SELFDESTRUCT => 0xFF,
        }
    }
}

/// Get gas cost for an opcode (simplified version)
pub fn gas_cost(opcode: Opcode) -> u64 {
    match opcode {
        Opcode::STOP => 0,
        Opcode::ADD | Opcode::SUB | Opcode::LT | Opcode::GT | Opcode::EQ | 
        Opcode::ISZERO | Opcode::AND | Opcode::OR | Opcode::XOR | Opcode::NOT => 3,
        Opcode::MUL | Opcode::DIV | Opcode::SDIV | Opcode::MOD => 5,
        Opcode::ADDMOD | Opcode::MULMOD => 8,
        Opcode::SHA3 => 30,  // Base cost, actual cost depends on data size
        Opcode::ADDRESS | Opcode::ORIGIN | Opcode::CALLER | Opcode::CALLVALUE |
        Opcode::CALLDATASIZE | Opcode::CODESIZE | Opcode::GASPRICE | Opcode::MSIZE => 2,
        Opcode::BALANCE => 400,
        Opcode::CALLDATALOAD => 3,
        Opcode::CALLDATACOPY | Opcode::CODECOPY => 3,  // Plus data size cost
        Opcode::POP => 2,
        Opcode::MLOAD | Opcode::MSTORE => 3,
        Opcode::MSTORE8 => 3,
        Opcode::SLOAD => 200,
        Opcode::SSTORE => 5000,  // For non-zero value, actual cost is context-dependent
        Opcode::JUMP | Opcode::JUMPI => 10,
        Opcode::PC | Opcode::GAS => 2,
        Opcode::JUMPDEST => 1,
        Opcode::PUSH1 | Opcode::PUSH2 | Opcode::PUSH32 => 3,
        Opcode::DUP1 | Opcode::DUP2 => 3,
        Opcode::SWAP1 | Opcode::SWAP2 => 3,
        Opcode::LOG0 => 375,  // Base cost, actual cost depends on data size
        Opcode::CREATE => 32000,
        Opcode::CALL => 700,  // Base cost, actual cost is context-dependent
        Opcode::RETURN => 0,
        Opcode::REVERT => 0,
        Opcode::INVALID => 0,
        Opcode::SELFDESTRUCT => 5000,
        _ => 0,
    }
}