use std::{
  env::{self, temp_dir},
  fs::{self, File, read_dir},
  io::{Read, Write},
  path::PathBuf,
  process::{Command, exit},
  thread::sleep,
  time::Duration,
};

use winit::event_loop::EventLoopProxy;

use crate::{DrawRequest, NEXT_SETUP};

pub fn handle_installer_loop(sink: EventLoopProxy<DrawRequest>) {
  let dotnetpath = r"C:\Program Files\dotnet\shared\Microsoft.WindowsDesktop.App";

  if let Ok(x) = read_dir(dotnetpath) {
    if x.into_iter().any(|x| {
      if let Ok(x) = x {
        if let Ok(x) = x.file_name().into_string() {
          let (x, _) = x.split_once(".").unwrap_or(("", ""));
          let x = x.parse::<u64>().unwrap_or(0);

          if x >= 10 {
            return true;
          }
        }
      }

      false
    }) {
      run_installer(&sink);
    }
  }

  _ = sink.send_event(DrawRequest {
    perc: 0,
    txt: "Downloading runtime...",
  });

  #[cfg(target_arch = "x86_64")]
  let dotneturl = "https://builds.dotnet.microsoft.com/dotnet/WindowsDesktop/10.0.1/windowsdesktop-runtime-10.0.1-win-x64.exe";

  #[cfg(target_arch = "aarch64")]
  let dotneturl = "https://builds.dotnet.microsoft.com/dotnet/WindowsDesktop/10.0.1/windowsdesktop-runtime-10.0.1-win-arm64.exe";

  let mut dotnetfilepath = temp_dir();
  dotnetfilepath.push("install_dotnet10.exe");

  let mut dotnetfile = File::create(&dotnetfilepath).expect("Unable to create dotnet");

  let mut dotnet = ureq::get(dotneturl).call().expect("Sending response");
  let total = dotnet
    .headers()
    .get("Content-Length")
    .and_then(|v| v.to_str().expect("NoErr").parse::<u64>().ok());

  let mut reader = dotnet.body_mut().as_reader();

  let mut downloaded = 0;

  let mut buf = Box::new([0u8; 16 * 1024]);

  let mut last_sent_perc = 0;

  while let Ok(n) = reader.read(buf.as_mut()) {
    if n == 0 {
      break;
    }

    dotnetfile
      .write_all(&buf[..n])
      .expect("Something else wrong!");

    downloaded += n as u64;

    if let Some(total) = total {
      let perc = downloaded * 100 / total;

      if perc != last_sent_perc {
        last_sent_perc = perc;
        let _ = sink.send_event(DrawRequest {
          perc,
          txt: "Downloading runtime...",
        });
      }
    }
  }

  dotnetfile.sync_all().expect("Unable to flush");
  drop(dotnetfile);

  let _ = sink.send_event(DrawRequest {
    perc: 100,
    txt: "Installing runtime...",
  });

  if !install_dotnet(dotnetfilepath) {
    _ = sink.send_event(DrawRequest {
      perc: 100,
      txt: "Something went wrong!",
    });

    sleep(Duration::from_secs(5));

    // 100: Dotnet Failed
    exit(100);
  }

  run_installer(&sink);
}

fn install_dotnet(path: PathBuf) -> bool {
  Command::new(path)
    .args(["/install", "/quiet", "/norestart"])
    .spawn()
    .expect("Unable to spawn")
    .wait()
    .expect("Unable to wait")
    .success()
}

fn run_installer(sink: &EventLoopProxy<DrawRequest>) {
  _ = sink.send_event(DrawRequest {
    perc: 100,
    txt: "Starting installer...",
  });

  sleep(Duration::from_secs(2));

  let mut tmp = temp_dir();
  tmp.push("ahqstore_dotnet_setup.exe");
  fs::write(&tmp, NEXT_SETUP).unwrap();

  Command::new(tmp)
    .args(env::args())
    .spawn()
    .expect("Unknown, unwanted error");

  exit(0);
}
