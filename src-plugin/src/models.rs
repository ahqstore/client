use serde::{Deserialize, Serialize};

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
enum DownloadEvent<'a> {
  #[serde(rename_all = "camelCase")]
  Started {
    url: &'a str,
    download_id: usize,
    content_length: usize,
  },
  #[serde(rename_all = "camelCase")]
  Progress {
    download_id: usize,
    chunk_length: usize,
  },
  #[serde(rename_all = "camelCase")]
  Finished {
    download_id: usize,
  },
}