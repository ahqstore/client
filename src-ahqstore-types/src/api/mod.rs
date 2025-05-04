#[cfg(feature = "js")]
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[cfg(feature = "js")]
use kfghdfghdfkgh_js_macros::TsifyAsync;

#[cfg(feature = "js")]
use tsify::{declare, JsValueSerdeExt};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "search")]
use fuse_rust::{FuseProperty, Fuseable};

#[cfg(feature = "internet")]
pub mod internet;

#[cfg(feature = "internet")]
pub use internet::*;

#[cfg(feature = "internet")]
pub mod methods;

#[cfg(feature = "internet")]
pub mod ahqstore;

#[cfg(feature = "internet")]
pub mod winget;

#[cfg(feature = "internet")]
pub mod linux;

#[cfg(feature = "internet")]
pub mod fdroid;

#[cfg(all(feature = "internet", feature = "search"))]
pub mod search;

use reqwest::{Client, ClientBuilder};
use std::sync::LazyLock;

pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
  ClientBuilder::new()
    .user_agent("AHQ Store Types / Rust / AHQ Softwares")
    .build()
    .unwrap()
});

#[cfg_attr(feature = "js", declare)]
pub type MapData = HashMap<String, Vec<String>>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "js", wasm_bindgen)]
/// This is exactly `Vec<(String, Vec<String>)>`
pub struct HomeMapData {
  inner: Vec<(String, Vec<String>)>,
}

impl Serialize for HomeMapData {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.inner.serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for HomeMapData {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let inner = Vec::deserialize(deserializer)?;
    Ok(HomeMapData { inner })
  }
}

#[cfg(feature = "js")]
#[wasm_bindgen]
impl HomeMapData {
  #[wasm_bindgen(constructor)]
  pub fn new() -> HomeMapData {
    HomeMapData {
      inner: vec![],
    }
  }

  #[wasm_bindgen(getter)]
  pub fn inner(&self) -> JsValue {
    serde_wasm_bindgen::to_value(&self.inner).unwrap()
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
pub struct Home {
  pub splash: Option<Splash>,
  pub home: HomeMapData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
pub struct Splash {
  pub hero: Hero,
  pub subhero: SubHero,
  pub third: Semi,
  pub fourth: Semi,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
#[allow(non_snake_case)]
pub struct Hero {
  pub title: String,
  pub description: String,
  pub button: String,
  pub background: String,
  pub author: String,
  pub appId: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
#[allow(non_snake_case)]
pub struct SubHero {
  pub title: String,
  pub background: String,
  pub appId: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
#[allow(non_snake_case)]
pub struct Semi {
  pub title: String,
  pub background: String,
  pub appId: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
pub struct SearchEntry {
  pub name: String,
  pub title: String,
  pub id: String,
}

#[cfg(feature = "search")]
impl Fuseable for SearchEntry {
  fn properties(&self) -> Vec<FuseProperty> {
    vec![
      FuseProperty {
        value: "id".into(),
        weight: 0.34,
      },
      FuseProperty {
        value: "title".into(),
        weight: 0.33,
      },
      FuseProperty {
        value: "name".into(),
        weight: 0.33,
      },
    ]
  }

  fn lookup(&self, key: &str) -> Option<&str> {
    match key {
      "name" => Some(&self.name),
      "title" => Some(&self.title),
      "id" => Some(&self.id),
      _ => None,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
pub struct DevData {
  pub name: String,
  pub id: String,
  pub github: String,
  pub avatar_url: String,
}
