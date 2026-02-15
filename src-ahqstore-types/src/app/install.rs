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

fn android_abi() -> Platform {
  match ARCH {
    "x86" => Platform::AndroidX86,
    "x86_64" => Platform::AndroidX64,
    "arm" => Platform::AndroidArm7,
    "aarch64" => Platform::AndroidArm64,
    _ => unreachable!(),
  }
}

pub fn current_platform() -> Platform {
  match OS {
    "windows" => match ARCH {
      "aarch64" => Platform::WindowsArm64,
      "x86_64" => Platform::WindowsX64,
      _ => unreachable!(),
    },
    "linux" => match ARCH {
      "aarch64" => Platform::LinuxArm64,
      "x86_64" => Platform::LinuxX64,
      _ => unreachable!(),
    },
    "android" => android_abi(),
    _ => unreachable!(),
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
  pub asset: AndroidAssetId,
  pub min_sdk: u32,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "assetType")]
pub enum AndroidAssetId {
  Universal {
    assetId: u8,
  },
  AbiBased {
    aarch64: Option<u8>,
    armv7: Option<u8>,
    x86: Option<u8>,
    x86_64: Option<u8>,
  },
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
  ($x:ident -> $y:ident.$install: ident, $arch: expr) => {
    if let Some(_) = &$y.$install {
      $x.push($arch);
    }
  };
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Platform {
  WindowsX64,
  WindowsArm64,
  LinuxX64,
  LinuxArm64,
  AndroidX64,
  AndroidX86,
  AndroidArm7,
  AndroidArm64,
}

impl InstallerOptions {
  /// 🎯 Introduced in v2
  pub fn list_os_arch(&self) -> Vec<Platform> {
    let mut arch = vec![];

    // If there's win32, it means we can use it on both arm and x86
    if let Some(_) = &self.win32 {
      arch.push(Platform::WindowsX64);
      arch.push(Platform::WindowsArm64);
    }

    // If only arm build is there, no x86
    if !arch.contains(&Platform::WindowsX64) {
      if let Some(_) = &self.winarm {
        arch.push(Platform::WindowsArm64);
      }
    }

    // Self explanatory
    push_install_arch!(arch -> self.linux, Platform::LinuxX64);
    push_install_arch!(arch -> self.linuxArm64, Platform::LinuxArm64);

    if let Some(x) = &self.android {
      match x.asset {
        AndroidAssetId::Universal { .. } => {
          arch.push(Platform::AndroidArm64);
          arch.push(Platform::AndroidArm7);
          arch.push(Platform::AndroidX64);
          arch.push(Platform::AndroidX86);
        }
        AndroidAssetId::AbiBased {
          aarch64,
          armv7,
          x86,
          x86_64,
        } => {
          if let Some(_) = aarch64 {
            arch.push(Platform::AndroidArm64);
          }
          if let Some(_) = armv7 {
            arch.push(Platform::AndroidArm7);
          }
          if let Some(_) = x86 {
            arch.push(Platform::AndroidX86);
          }
          if let Some(_) = x86_64 {
            arch.push(Platform::AndroidX64);
          }
        }
      }
    }

    arch
  }

  /// 🎯 Introduced in v2
  pub fn is_supported(&self) -> bool {
    let os = self.list_os_arch();

    os.contains(&current_platform())
  }

  /// 🎯 Introduced in v3
  pub fn is_supported_android(&self, sdk: u32) -> bool {
    let os = self.list_os_arch();

    let Some(x) = &self.android else {
      return false;
    };

    if OS == "android" {
      return os.contains(&android_abi()) && x.min_sdk <= sdk;
    }

    false
  }

  /// 🎯 Introduced in v2
  pub fn has_platform(&self) -> bool {
    self.is_supported()
  }
}
