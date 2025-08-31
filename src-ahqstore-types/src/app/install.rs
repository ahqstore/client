#[cfg(feature = "js")]
use wasm_bindgen::prelude::wasm_bindgen;

use std::env::consts::{ARCH, OS};

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
pub struct InstallerOptions {
  #[doc = "🎯 Introduced in v1\n\n"]
  pub win32: Option<InstallerOptionsWindows>,
  #[doc = "🎯 Introduced in v2\n\n"]
  pub winarm: Option<InstallerOptionsWindows>,
  #[doc = "🎯 Introduced in v1\n\n"]
  pub linux: Option<InstallerOptionsLinux>,
  #[doc = "🎯 Introduced in v2\n\n"]
  pub linuxArm64: Option<InstallerOptionsLinux>,
  #[doc = "🎯 Introduced in v2\n\n"]
  pub linuxArm7: Option<InstallerOptionsLinux>,
  #[doc = "🔬 Planned\n🎯 Introduced in v2\n\n"]
  pub android: Option<InstallerOptionsAndroid>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum WindowsInstallScope {
  User,
  Machine,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
pub struct InstallerOptionsWindows {
  #[doc = "🎯 Introduced in v2\n\n"]
  pub assetId: u8,
  /// The exe to link as a shortcut[^1]
  ///
  /// [^1]: Only if you choose WindowsZip
  pub exec: Option<String>,
  #[doc = "🎯 Introduced in v1\n\n"]
  /// The scope of the installer[^1]
  ///
  /// [^1]: Applicable for WindowsInstallerExe or WindowsZip only, WindowsInstallerMsi is treated as Machine
  pub scope: Option<WindowsInstallScope>,
  #[doc = "🎯 Stable as of v3\n\n"]
  /// Args to pass to the custom exe installer[^1]
  ///
  /// [^1]: Only if you choose WindowsInstallerExe
  pub installerArgs: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "🔬 Under Development\n\n"]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum AndroidAbi {
  Aarch64,
  Armv7,
  X86,
  X64,
}

impl AndroidAbi {
  fn normalize(&self) -> &'static str {
    match self {
      &Self::Aarch64 => "android-aarch64",
      &Self::Armv7 => "android-armv7",
      &Self::X86 => "android-x86",
      &Self::X64 => "android-x86_64",
    }
  }
}

fn android_abi() -> &'static str {
  match ARCH {
    "x86" => "android-x86",
    "x86_64" => "android-x86_64",
    "arm" => "android-armv7",
    "aarch64" => "android-aarch64",
    _ => "none",
  }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "🔬 Under Development\n\n"]
#[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
pub struct InstallerOptionsAndroid {
  #[doc = "🎯 Introduced in v2\n\n"]
  pub assetId: u8,
  pub min_sdk: u32,
  pub abi: Vec<AndroidAbi>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[doc = "🔬 Under Development\n\n"]
#[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
pub struct InstallerOptionsLinux {
  #[doc = "🎯 Introduced in v2\n\n"]
  pub assetId: u8,
}

macro_rules! push_install_arch {
  ($x:ident -> $y:ident.$install: ident, $arch: literal) => {
    if let Some(_) = &$y.$install {
      $x.push($arch);
    }
  };
}

impl InstallerOptions {
  #[doc = "🎯 Introduced in v2"]
  pub fn list_os_arch(&self) -> Vec<&'static str> {
    let mut arch = vec![];

    // If there's win32, it means we can use it on both arm and x86
    if let Some(_) = &self.win32 {
      arch.push("windows-x86_64");
      arch.push("windows-aarch64");
    }

    // If only arm build is there, no x86
    if !arch.contains(&"windows-aarch64") {
      if let Some(_) = &self.winarm {
        arch.push("windows-aarch64");
      }
    }

    // Self explanatory
    push_install_arch!(arch -> self.linux, "linux-x86_64");
    push_install_arch!(arch -> self.linuxArm64, "linux-aarch64");
    push_install_arch!(arch -> self.linuxArm7, "linux-arm");

    if let Some(x) = &self.android {
      x.abi.iter().for_each(|x| arch.push(x.normalize()));
    }

    arch
  }

  #[doc = "🎯 Introduced in v2"]
  pub fn is_supported(&self) -> bool {
    let os = self.list_os_arch();
    if OS == "android" {
      return os.contains(&android_abi());
    }

    os.contains(&format!("{}-{}", OS, ARCH).as_str())
  }

  #[doc = "🎯 Introduced in v3"]
  pub fn is_supported_android(&self, sdk: u32) -> bool {
    let os = self.list_os_arch();

    let Some(x) = &self.android else {
      return false;
    };

    if OS == "android" {
      return os.contains(&android_abi()) && x.min_sdk == sdk;
    }

    false
  }

  #[doc = "🎯 Introduced in v2"]
  pub fn has_platform(&self) -> bool {
    self.is_supported()
  }
}
