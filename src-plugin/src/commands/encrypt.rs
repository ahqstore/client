use bcrypt::{hash_with_salt, Version, DEFAULT_COST};
use chacha20poly1305::{
  aead::{generic_array::GenericArray, Aead, KeyInit},
  ChaCha20Poly1305,
};
use lazy_static::lazy_static;
use anyhow::Context;
use crate::error::Result;

lazy_static! {
  static ref CRYPTER: ChaCha20Poly1305 = {
    let key = GenericArray::from_slice(
      include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/encrypt")).as_bytes(),
    );
    ChaCha20Poly1305::new(&key)
  };
}

static SALT: [u8; 16] = [
  0x14, 0x4b, 0x3d, 0x69, 0x1a, 0x7b, 0x4e, 0xcf, 0x39, 0xcf, 0x73, 0x5c, 0x7f, 0xa7, 0xa7, 0x9c,
];

use serde_json::to_string;

#[tauri::command(async)]
pub fn encrypt(payload: String) -> Result<Vec<u8>> {
  let nonce = GenericArray::from_slice(b"SSSSSSSSSSSS");
  
  Ok(CRYPTER.encrypt(nonce, payload.as_bytes()).ok().context("Cannot encrypt")?)
}

#[tauri::command(async)]
pub fn decrypt(encrypted: Vec<u8>) -> Result<String> {
  let nonce = GenericArray::from_slice(b"SSSSSSSSSSSS");

  let decrypted = CRYPTER.decrypt(nonce, &*encrypted).ok().context("Cannot decrypt")?;

  Ok(String::from_utf8(decrypted)?)
}
