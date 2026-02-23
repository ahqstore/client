use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

use reqwest::{Client, ClientBuilder};
use std::sync::LazyLock;

pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
  ClientBuilder::new()
    .user_agent("AHQ Store Types / Rust / AHQ Softwares")
    .build()
    .unwrap()
});

pub type MapData = Vec<String>;

#[cfg_attr(feature = "export", derive(specta::Type))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Clone)]

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

#[cfg_attr(feature = "export", derive(specta::Type))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Home {
  pub splash: Option<Splash>,
  pub home: HomeMapData,
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Splash {
  pub hero: Hero,
  pub subhero: SubHero,
  pub third: Semi,
  pub fourth: Semi,
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Hero {
  pub title: String,
  pub description: String,
  pub button: String,
  pub background: String,
  pub author: String,
  pub appId: String,
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct SubHero {
  pub title: String,
  pub background: String,
  pub appId: String,
  pub color: Option<String>,
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Semi {
  pub title: String,
  pub background: String,
  pub appId: String,
  pub color: Option<String>,
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]

pub struct SearchEntry {
  pub name: String,
  pub title: String,
  pub id: String,
}

#[cfg_attr(feature = "export", derive(specta::Type))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug)]

pub struct DevData {
  pub name: Option<String>,
  pub id: String,
  pub github: String,
  pub avatar_url: Option<String>,
  #[serde(default)]
  pub verified: bool,
}
