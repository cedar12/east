use std::{
    fs::{self, File},
    io::Write,
};

use rsa::{
    pkcs8::{DecodePrivateKey, EncodePrivateKey, EncodePublicKey, LineEnding},
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};

use crate::config;

const BITS: usize = 2048;

pub fn generate_key() -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, BITS)?;
    let pub_key = RsaPublicKey::from(&priv_key);

    let key = config::CONF.server.key.clone();
    match key {
        Some(key_name) => {
          let private_key_pem = priv_key.to_pkcs8_pem(LineEnding::CRLF)?;
          let mut file = File::create(key_name.clone())?;
          file.write_all(private_key_pem.as_bytes())?;
          let public_key_pem = pub_key.to_public_key_pem(LineEnding::CRLF)?;
          let mut file = File::create(format!("pub_{}", key_name.clone()))?;
          file.write_all(public_key_pem.as_bytes())?;
          Ok(key_name)
        },
        None => {
          Err(anyhow::anyhow!("未配置密钥"))
        },
    }

    
    
}

pub fn is_exist_key() -> bool {
    let key_name = config::CONF.server.key.clone();
    match key_name {
        Some(key_name) => {
          if let Ok(metadata) = fs::metadata(key_name) {
              return true;
          }
        },
        None => {
          return true;
        },
    }
    
    false
}

pub fn decrypt(enc_data: Vec<u8>) -> anyhow::Result<String> {
    let key:Option<String> = config::CONF.server.key.clone();
    if let Some(key_name)=key{
      let priv_key = RsaPrivateKey::read_pkcs8_pem_file(key_name)?;
      let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data)?;
      let data = String::from_utf8(dec_data.clone()).unwrap();
      Ok(data)
    }else{
      match String::from_utf8(enc_data){
        Ok(s)=>Ok(s),
        Err(e)=>Err(anyhow::anyhow!("{:?}",e))
      }
    }
    
}


pub fn init(){
  if !is_exist_key(){
    match generate_key() {
        Ok(file_name) => {
          log::info!("生成密钥文件\n私钥文件: {}\n公钥文件: pub_{}",file_name,file_name)
        },
        Err(e) => {
          log::error!("生成密钥文件失败{:?}",e)
        },
    }
  }
}