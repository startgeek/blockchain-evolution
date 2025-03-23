use std::collections::HashMap;

#[derive(Debug)]
struct ETHState {
    accounts: HashMap<String, u64>, // Stores balance directly
}

impl ETHState {
    fn get_balance(&self, address: &str) -> u64 {
        *self.accounts.get(address).unwrap_or(&0)
    }

    fn transfer(&mut self, from: &str, to: &str, amount: u64) -> bool {
        if let Some(balance) = self.accounts.get_mut(from) {
            if *balance >= amount {
                *balance -= amount;
                *self.accounts.entry(to.to_string()).or_insert(0) += amount;
                return true;
            }
        }
        false // Insufficient funds
    }
}

fn main() {
    let mut state = ETHState {
        accounts: HashMap::from([
            ("Alice".to_string(), 500),
            ("Bob".to_string(), 100),
        ]),
    };

    println!("ETH Balance (Alice): {}", state.get_balance("Alice"));
    println!("ETH Balance (Bob): {}", state.get_balance("Bob"));

    state.transfer("Alice", "Bob", 400);

    println!("New ETH Balance (Alice): {}", state.get_balance("Alice"));
    println!("New ETH Balance (Bob): {}", state.get_balance("Bob"));
}
