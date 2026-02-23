use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AndroidBuildOutput {
  pub sdk: u32,
  pub release: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowCodeRequest {
  pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoomRequest {
  pub zoom: f32,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum DownloadEvent {
  #[serde(rename_all = "camelCase")]
  Started { length: u64 },
  #[serde(rename_all = "camelCase")]
  Progress { progress: f64 },
  #[serde(rename_all = "camelCase")]
  Finished {},
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum AppInstallStatus {
  Downloading(DownloadEvent),
  AVScanning,
  Installing,
  AppInstallStat(InstallStat),
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum InstallStat {
  AVFailed,
  Installed,
  InstallFailed,
}
