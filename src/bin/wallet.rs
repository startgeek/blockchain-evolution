use rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha3::{Keccak256, Digest};
use hex;

fn main() {
    // Initialize the secp256k1 context
    let secp = Secp256k1::new();
    let mut rng = OsRng;

    // Generate random private key
    let (secret_key, public_key) = generate_keypair(&secp, &mut rng);

    // Derive Ethereum address
    let address = public_key_to_address(&public_key);

    println!("📦 Ethereum Wallet Generated!");
    println!("=============================");
    println!("🔐 Private Key: 0x{}", hex::encode(secret_key.secret_bytes()));
    println!("🔓 Public Key : 0x{}", hex::encode(public_key.serialize_uncompressed()));
    println!("🏠 Address    : 0x{}", hex::encode(address));
}

fn generate_keypair<R: rand::RngCore + rand::CryptoRng>(
    secp: &Secp256k1<secp256k1::All>,
    rng: &mut R
) -> (SecretKey, PublicKey) {
    let secret_key = SecretKey::new(rng);
    let public_key = PublicKey::from_secret_key(secp, &secret_key);
    (secret_key, public_key)
}

fn public_key_to_address(public_key: &PublicKey) -> [u8; 20] {
    let pubkey = public_key.serialize_uncompressed();
    let hash = Keccak256::digest(&pubkey[1..]); // skip the 0x04 prefix byte
    let mut address = [0u8; 20];
    address.copy_from_slice(&hash[12..]); // last 20 bytes
    address
}
