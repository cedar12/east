use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Encrypt, RsaPublicKey};

use crate::config;

pub fn encrypt(data: String) -> anyhow::Result<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let key = config::CONF.key.clone();
    match key {
        Some(key_path) => {
            let pub_key = RsaPublicKey::read_public_key_pem_file(key_path)?;
            let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data.as_bytes())?;
            Ok(enc_data)
        },
        None => {
            Ok(data.as_bytes().to_vec())
        },
    }
    
}
