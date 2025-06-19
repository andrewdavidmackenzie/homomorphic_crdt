use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32, ServerKey};

fn main() {
    let config = ConfigBuilder::default().build();

    // generate client and server keys
    let (client_key, server_key) = generate_keys(config);

    // generate plaintext
    let clear_a: u32 = 27;
    let clear_b: u32 = 128;

    // encrypt plaintext and "send to server"
    let result = server_compute(
        server_key,
        FheUint32::encrypt(clear_a, &client_key),
        FheUint32::encrypt(clear_b, &client_key),
    );

    // decrypt the result
    let decrypted_result: u32 = result.decrypt(&client_key);

    // assert that the result is what we expect
    assert_eq!(decrypted_result, clear_a + clear_b);
}

fn server_compute(key: ServerKey, cipher_a: FheUint32, cipher_b: FheUint32) -> FheUint32 {
    set_server_key(key);
    cipher_a + cipher_b
}