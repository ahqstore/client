using System.Diagnostics;
using System.IO;
using System.Net.Http;
using Microsoft.VisualBasic.FileIO;

public enum Typeof
{
  Downloading,
  Indeterminate,
  None,
  Installed,
  Error
}

public class Installer
{

  public string GetTmpPath()
  {
    var idx = $"ahqstore_{Guid.NewGuid()}.msi";

    var tmp = SpecialDirectories.Temp;

    return Path.Combine(tmp, idx); ;
  }

  public async Task Install(
    string url,
    Action<string, Typeof, int?> data
  )
  {
    data("Downloading...", Typeof.Indeterminate, null);

    var msiexecRan = GetTmpPath();

    /**
      This part handling all of download
    */
    HttpClient client = new HttpClient();

    HttpResponseMessage response = await client.GetAsync(url, HttpCompletionOption.ResponseHeadersRead);
    response.EnsureSuccessStatusCode();

    long totalBytes = (long)response.Content.Headers.ContentLength!;
    long downloaded = 0;

    using (var fileStream = new FileStream(msiexecRan, FileMode.Create, FileAccess.Write, FileShare.None, 8192, true))
    {

      var stream = await response.Content.ReadAsStreamAsync();
      var buffer = new byte[8192];
      int read;


      while ((read = await stream.ReadAsync(buffer, 0, buffer.Length)) != 0)
      {
        await fileStream.WriteAsync(buffer, 0, read);
        downloaded += read;

        data("Downloading...", Typeof.Downloading, (int)((downloaded * 100) / totalBytes));
      }

      await fileStream.FlushAsync();
    }

    /**
      Installer module
    */
    data("Installing...", Typeof.Indeterminate, null);

    var msiexec = @"C:\Windows\System32\msiexec.exe";

    var process = new Process();

    process.StartInfo.FileName = msiexecRan;

    process.Start();

    await process.WaitForExitAsync();
    var code = process.ExitCode;

    if (code == 0 || code == 3010)
    {
      data("Installed", Typeof.Installed, null);
    }
    else
    {
      data($"Something went wrong, err {code}!", Typeof.Error, null);
    }

    FileSystem.DeleteFile(msiexecRan);
  }
}