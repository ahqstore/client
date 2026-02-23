use std::fs::File;
use std::io::Write;
use std::path::Path;

#[cfg(windows)]
use std::os::windows::fs::FileExt;

#[cfg(unix)]
use std::os::unix::fs::FileExt;

use anyhow::Context;

#[derive(Debug)]
pub struct TurboFile {
  inner: File,
}

impl TurboFile {
  pub fn create<T: AsRef<Path>>(path: T, size: u64) -> crate::Result<Self> {
    let f = File::create(path)?;

    f.set_len(size)?;

    Ok(Self { inner: f })
  }

  pub fn flush(mut self) -> crate::Result<()> {
    self.inner.flush()?;
    self.inner.sync_all()?;

    Ok(())
  }

  pub fn write_all(&mut self, data: &[u8]) -> crate::Result<()> {
    self.inner.write_all(data)?;
    self.inner.flush()?;

    Ok(())
  }

  pub fn write_seekable(&self, data: &[u8], offset: u64) -> crate::Result<()> {
    #[cfg(windows)]
    {
      _ = self.inner.seek_write(data, offset)?;

      return Ok(());
    }

    #[cfg(unix)]
    {
      return Ok(self.inner.write_all_at(data, offset)?);
    }
  }
}
