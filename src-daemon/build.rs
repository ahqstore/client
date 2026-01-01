

fn main() {
  if cfg!(target_os = "windows") {
    let mut res = tauri_winres::WindowsResource::new();
    res.set_icon("icon.ico")
      .set("ProductName", "AHQ Store Elevation Agent")
      .set("LegalCopyright", "Copyright © 2025 AHQ Softwares")
      .set("OriginalFilename", "AHQ Store Elevation Agent")
      .set("Comments", "AHQ Store Elevation Agent")
      .set_manifest_file("manifest.xml");
    res.compile().unwrap();
  }
}