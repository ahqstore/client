fn main() {
  let mut res = winres::WindowsResource::new();
  res
    .set_icon("./ui/icon.ico")
    .set("InternalName", "installAHQStore.exe");

  res.compile().expect("Error");
}
