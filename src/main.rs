mod register;
mod encryptedregister;

use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32, ServerKey};

fn main() {
    let config = ConfigBuilder::default().build();

    // generate client and server keys
    let (client_key, server_key) = generate_keys(config);

    // generate plaintext
    let clear_a: u32 = 27;
    let clear_b: u32 = 128;

    let encrypted_a = FheUint32::encrypt(clear_a, &client_key);
    let encrypted_b = FheUint32::encrypt(clear_b, &client_key);

    // encrypt plaintext and "send to server"
    let result = server_compute(
        server_key,
        encrypted_a,
        encrypted_b,
    );

    // decrypt the result
    let decrypted_result: u32 = result.decrypt(&client_key);

    // assert that the result is what we expect
    assert_eq!(decrypted_result, clear_a + clear_b);
    println!("Encrypted sum equals unencrypted sum");
}

fn server_compute(key: ServerKey, cipher_a: FheUint32, cipher_b: FheUint32) -> FheUint32 {
    set_server_key(key);
    cipher_a + cipher_b
}