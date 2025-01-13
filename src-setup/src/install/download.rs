use std::{fs, io::Write};

use reqwest::Client;

pub async fn download<T: FnMut(f32)>(client: &mut Client, url: &str, path: &str, mut call: T) {
  let _ = fs::remove_file(&path);

  println!("Path: {}", &path);
  let mut response = client.get(url).send().await.unwrap();

  let mut c = 0;
  let t = (response.content_length().unwrap_or(10000)) as u64;

  let mut file = vec![];

  let mut last = 0u64;

  while let Some(chunk) = response.chunk().await.unwrap() {
    c += chunk.len();

    if last != (c as u64 * 100) / t {
      last = (c as u64 * 100) / t;
      call(last as f32 / 100.0);
    }

    file.write_all(&chunk.to_vec()).unwrap();
  }

  fs::write(path, file).unwrap();
}
