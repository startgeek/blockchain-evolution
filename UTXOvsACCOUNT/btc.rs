use std::collections::HashMap;

#[derive(Debug, Clone)]
struct UTXO {
    amount: u64,  // Amount in satoshis
    owner: String, // Owner's public key or address
}

#[derive(Debug)]
struct BTCWallet {
    utxos: Vec<UTXO>,
}

impl BTCWallet {
    fn get_balance(&self) -> u64 {
        self.utxos.iter().map(|utxo| utxo.amount).sum()
    }

    fn spend_utxo(&mut self, amount: u64, recipient: String) -> Option<Vec<UTXO>> {
        let mut selected = vec![];
        let mut total = 0;

        while total < amount && !self.utxos.is_empty() {
            let utxo = self.utxos.pop().unwrap();
            total += utxo.amount;
            selected.push(utxo);
        }

        if total < amount {
            return None; // Not enough balance
        }

        let mut new_utxos = vec![UTXO { amount, owner: recipient }];
        if total > amount {
            // Change back to sender
            new_utxos.push(UTXO { amount: total - amount, owner: selected[0].owner.clone() });
        }

        Some(new_utxos)
    }
}

fn main() {
    let mut wallet = BTCWallet { 
        utxos: vec![
            UTXO { amount: 300, owner: "Alice".to_string() },
            UTXO { amount: 200, owner: "Alice".to_string() },
        ]
    };

    println!("BTC Balance: {}", wallet.get_balance());
    if let Some(new_utxos) = wallet.spend_utxo(400, "Bob".to_string()) {
        wallet.utxos = new_utxos;
    }

    println!("New BTC Balance: {}", wallet.get_balance());
}
