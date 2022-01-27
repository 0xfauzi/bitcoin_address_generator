use std::str::FromStr;
use base58::ToBase58;
use crypto::digest::Digest;
use crypto::ripemd160::Ripemd160;
use rand::{Rng, thread_rng};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use crypto::sha2::Sha256;
use itertools::Itertools;

fn main() {
    let private_key = generate_private_key();
    println!("Private key: {:02x}", private_key.iter().format(""));

    let public_key = generate_public_key(&private_key);
    println!("Public Key: {:02x}", public_key.iter().format(""));

    let sha256_hashed_public_key = hash_sha256(&public_key);
    println!("Sha256 hashed public key: {:02x}", sha256_hashed_public_key.iter().format(""));

    let ripemd160_hashed_public_key = hash_ripemd160(&sha256_hashed_public_key);
    println!("RIPEMD160 hashed public key: {:02x}", ripemd160_hashed_public_key.iter().format(""));

    let mut result_with_version_byte: [u8; 21] = [0; 21];

    for i in 1..21 {
        result_with_version_byte[i] = ripemd160_hashed_public_key[i - 1];
    }
    println!("Result with version byte: {:02x}", result_with_version_byte.iter().format(""));

    let hashed: [u8; 32] = hash_sha256(&result_with_version_byte);
    let hashed_2x: [u8; 32] = hash_sha256(&hashed);
    println!("Result with version byte and 2x sha256: {:02x}", hashed_2x.iter().format(""));

    let base_58_check_encoded_address = base58check_encode(&hashed_2x, &result_with_version_byte);
    println!("Base58Check encoded address: {}", base_58_check_encoded_address);

    let actual_address = bitcoin::util::address::Address::from_str(&base_58_check_encoded_address);
    match actual_address {
        Ok(address) => println!("Generated address is valid {:?}", address),
        Err(error) => println!("Invalid address generated. {:?}", error.to_string()),
    };
}

fn generate_private_key() -> [u8; 32] {
    thread_rng().gen::<[u8; 32]>()
}

fn generate_public_key(private_key: &[u8; 32]) -> [u8; 33] {
    let secp256k1 = Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key).unwrap();
    let public_key = PublicKey::from_secret_key(&secp256k1, &secret_key).serialize();

    public_key
}

fn hash_sha256(public_key: &[u8]) -> [u8; 32] {
    let mut sha256_hasher = Sha256::new();

    sha256_hasher.input(public_key);

    let mut hashed: [u8; 32] = [0; 32];
    sha256_hasher.result(&mut hashed);

    hashed
}

fn hash_ripemd160(hash_sha256: &[u8; 32]) -> [u8; 20] {
    let mut ripemd160_hasher = Ripemd160::new();

    ripemd160_hasher.input(hash_sha256);

    let mut hashed: [u8; 20] = [0; 20];
    ripemd160_hasher.result(&mut hashed);

    hashed
}

fn base58check_encode(hashed_2x: &[u8; 32], result_with_version_byte: &[u8; 21]) -> String {

    let checksum = &hashed_2x[0..4];
    let mut result: [u8; 25] = [0; 25];

    for i in 0..21 {
        result[i] = result_with_version_byte[i];
    }

    for i in 0..4 {
        result[21 + i] = checksum[i];
    }

    result.to_base58()
}