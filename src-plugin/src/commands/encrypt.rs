use std::sync::LazyLock;

use crate::error::Result;
use anyhow::Context;
use chacha20poly1305::{
  aead::{generic_array::GenericArray, Aead, KeyInit, OsRng},
  AeadCore, ChaCha20Poly1305,
};

macro_rules! stitch_32 {
  ($a:expr, $b:expr) => {{
    let mut res = [0u8; $a.len() + $b.len()];
    let mut i = 0;
    while i < $a.len() {
      res[i] = $a[i];
      i += 1;
    }
    let mut j = 0;
    while j < $b.len() {
      res[i + j] = $b[j];
      j += 1;
    }
    res
  }};
}

const A: u128 = const_random::const_random!(u128);
const B: u128 = const_random::const_random!(u128);

const _: () = assert!(A != B, "Predictive Randomness");

static CRYPTER: LazyLock<ChaCha20Poly1305> = LazyLock::new(|| {
  const KEY: [u8; 32] = stitch_32!(A.to_ne_bytes(), B.to_ne_bytes());

  ChaCha20Poly1305::new_from_slice(&KEY).expect("The length of validated to be correct")
});

use serde_json::to_string;

#[tauri::command(async)]
pub fn encrypt(payload: String) -> Result<Vec<u8>> {
  let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

  let mut nonce_vect = nonce.to_vec();
  assert_eq!(nonce_vect.len(), 12);

  nonce_vect.extend(
    CRYPTER
      .encrypt(&nonce, payload.as_bytes())
      .ok()
      .context("Cannot encrypt")?,
  );

  Ok(nonce_vect)
}

#[tauri::command(async)]
pub fn decrypt(encrypted: Vec<u8>) -> Result<String> {
  if encrypted.len() <= 12 {
    return Err(crate::error::Error::SearchError);
  }

  let nonce = GenericArray::from_slice(&encrypted[0..12]);

  let decrypted = CRYPTER
    .decrypt(nonce, &encrypted[12..])
    .ok()
    .context("Cannot decrypt")?;

  Ok(String::from_utf8(decrypted)?)
}
