[![Rust](https://github.com/0xfauzi/bitcoin_address_generator/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/0xfauzi/bitcoin_address_generator/actions/workflows/rust.yml)

# Bitcoin Address Generator

This Rust project implements a Bitcoin address generator. It creates a private key, derives the corresponding public key, and generates a Bitcoin address using the standard process.

## Features

- Generates a random private key
- Derives the corresponding public key
- Performs necessary hashing steps (SHA256 and RIPEMD160)
- Implements Base58Check encoding
- Validates the generated address using the `bitcoin` crate

## Usage

To use this Bitcoin address generator, you can run the `main()` function. It will generate a new Bitcoin address and print the intermediate steps. Here's an example of the output:

```
Private key: [32 bytes of hexadecimal]
Public Key: [33 bytes of hexadecimal]
Sha256 hashed public key: [32 bytes of hexadecimal]
RIPEMD160 hashed public key: [20 bytes of hexadecimal]
Result with version byte: [21 bytes of hexadecimal]
Result with version byte and 2x sha256: [32 bytes of hexadecimal]
Base58Check encoded address: [Bitcoin address]
Generated address is valid [Address details]
```

## Function Explanations

### `generate_private_key()`
Generates a random 32-byte private key using the `rand` crate.

### `generate_public_key(private_key: &[u8; 32])`
Derives the corresponding 33-byte compressed public key from the private key using the secp256k1 elliptic curve.

### `hash_sha256(public_key: &[u8])`
Performs SHA256 hashing on the input data.

### `hash_ripemd160(hash_sha256: &[u8; 32])`
Performs RIPEMD160 hashing on the input data.

### `base58check_encode(hashed_2x: &[u8; 32], result_with_version_byte: &[u8; 21])`
Implements the Base58Check encoding process to create the final Bitcoin address.

## Address Generation Process

1. Generate a random private key
2. Derive the corresponding public key
3. Perform SHA256 hashing on the public key
4. Perform RIPEMD160 hashing on the result of step 3
5. Add a version byte (0x00 for mainnet addresses)
6. Perform double SHA256 hashing for checksum
7. Append the first 4 bytes of the checksum to the result of step 5
8. Perform Base58Check encoding on the result

## Security Considerations

While this implementation uses cryptographically secure random number generation and standard cryptographic libraries, it's important to note that this is a basic implementation for educational purposes. For real-world applications involving cryptocurrency, it's crucial to use well-audited libraries and follow best practices for secure key management.

## Contributing

Contributions to improve the code or documentation are welcome. Please feel free to submit issues or pull requests to the repository.


## Disclaimer

This code is provided for educational purposes only. Use at your own risk. The authors and contributors are not responsible for any loss of funds or other damages that may occur from using this code. Always double-check generated addresses and never use this code for managing real funds without thorough testing and security audits.
