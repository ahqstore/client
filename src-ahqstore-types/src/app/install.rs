use std::env::consts::{ARCH, OS};

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct InstallerOptions {
  /// 🎯 Introduced in v1
///
///
  pub win32: Option<InstallerOptionsWindows>,
  /// 🎯 Introduced in v2
///
///
  pub winarm: Option<InstallerOptionsWindows>,
  /// 🎯 Introduced in v1
///
///
  pub linux: Option<InstallerOptionsLinux>,
  /// 🎯 Introduced in v2
///
///
  pub linuxArm64: Option<InstallerOptionsLinux>,
  /// 🎯 Introduced in v2
///
///
  pub linuxArm7: Option<InstallerOptionsLinux>,
  /// 🔬 Planned\n🎯 Introduced in v2
///
///
  pub android: Option<InstallerOptionsAndroid>,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum WindowsInstallScope {
  User,
  Machine,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct InstallerOptionsWindows {
  /// 🎯 Introduced in v2
///
///
  pub assetId: u8,
  /// The exe to link as a shortcut[^1]
  ///
  /// [^1]: Only if you choose WindowsZip
  pub exec: Option<String>,
  /// 🎯 Introduced in v1
///
///
  /// The scope of the installer[^1]
  ///
  /// [^1]: Applicable for WindowsInstallerExe or WindowsZip only, WindowsInstallerMsi is treated as Machine
  pub scope: Option<WindowsInstallScope>,
  /// 🎯 Stable as of v3
///
///
  /// Args to pass to the custom exe installer[^1]
  ///
  /// [^1]: Only if you choose WindowsInstallerExe
  pub installerArgs: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]
/// 🔬 Under Development
///
///

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
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]
/// 🔬 Under Development
///
///

pub struct InstallerOptionsAndroid {
  /// 🎯 Introduced in v2
///
///
  pub assetId: u8,
  pub min_sdk: u32,
  pub abi: Vec<AndroidAbi>,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]
/// 🔬 Under Development
///
///

pub struct InstallerOptionsLinux {
  /// 🎯 Introduced in v2
///
///
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
  /// 🎯 Introduced in v2
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

  /// 🎯 Introduced in v2
  pub fn is_supported(&self) -> bool {
    let os = self.list_os_arch();
    if OS == "android" {
      return os.contains(&android_abi());
    }

    os.contains(&format!("{}-{}", OS, ARCH).as_str())
  }

  /// 🎯 Introduced in v3
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

  /// 🎯 Introduced in v2
  pub fn has_platform(&self) -> bool {
    self.is_supported()
  }
}
