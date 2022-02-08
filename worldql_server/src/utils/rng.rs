use rand::prelude::*;

pub fn crypto_secure_token() -> String {
    let mut rng = rand::thread_rng();
    let random_bytes: [u8; 32] = rng.gen();

    base64::encode(random_bytes)
}
