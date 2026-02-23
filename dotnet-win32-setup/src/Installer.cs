using System.Diagnostics;
using System.IO;
using System.Net.Http;
using System.Security.Cryptography.X509Certificates;
using Microsoft.VisualBasic.FileIO;
using Microsoft.Win32;
using WixToolset.Dtf.WindowsInstaller;

public enum Typeof
{
  Downloading,
  Indeterminate,
  None,
  Installed,
  Error
}

public class AHQInstaller
{

  public string GetTmpPath()
  {
    var idx = $"ahqstore_{Guid.NewGuid()}.msi";

    var tmp = SpecialDirectories.Temp;

    return Path.Combine(tmp, idx); ;
  }

  public async Task Install(
    string url,
    bool installCerts,
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
      await fileStream.DisposeAsync();
    }

    /**
      Installer module
    */
    data("Closing running ahqstore processes...", Typeof.Indeterminate, null);

    foreach (var process in Process.GetProcesses())
    {
      try
      {
        if (process.ProcessName.Contains("ahqstore"))
        {
          process.Kill(true);
          process.WaitForExit(3000);
        }
      }
      catch { /* Ignore if already exiting */ }
    }

    data("Preparing install...", Typeof.Indeterminate, null);

    await Task.Delay(3000);

    data("Installing...", Typeof.Indeterminate, null);

    try
    {

      Installer.SetInternalUI(InstallUIOptions.Silent);
      Installer.InstallProduct(msiexecRan, "");

      data("Finalizing...", Typeof.Indeterminate, null);

      var hook = new InstallerHook();

      if (installCerts)
      {
        hook.InstallRootCA();
      }

      hook.InstallTheInstaller();
      hook.RegisterRegistryEntries();

      FileSystem.DeleteFile(msiexecRan);

      data("Installed", Typeof.Installed, null);
    }
    catch (InstallCanceledException)
    {
      data("Installation cancelled by user.", Typeof.Error, null);
    }
    catch (InstallerException ex)
    {
      Console.WriteLine(ex);

      if (ex.ErrorCode == 1618)
      {
        data($"Another installation is in progress, try again later", Typeof.Error, null);
        return;
      }

      if (ex.ErrorCode == 112)
      {
        data($"Your disk is full. Free up space and try again.", Typeof.Error, null);
        return;
      }

      if (ex.ErrorCode == 1603)
      {
        data("Uninstall AHQ Store first to install again!", Typeof.Error, null);
        return;
      }

      if (ex.ErrorCode == 3010 || ex.ErrorCode == 1641)
      {
        data($"AHQ Store should be installed. If not, please restart your computer and try again.", Typeof.Error, null);
        return;
      }

      data($"Failed: {ex.Message} (Error {ex.ErrorCode})", Typeof.Error, null);
    }
    catch (Exception e)
    {
      data($"Critical Error: {e.Message}", Typeof.Error, null);
    }
  }
}

public class InstallerHook
{
  public readonly string setupclone = @"C:\Program Files\AHQ Store Neo\instmgnt.exe";

  public readonly string certLocation = @"C:\Program Files\AHQ Store Neo\windows.cer";

  public readonly string preUninstall = @"C:\Program Files\AHQ Store Neo\.msiexec";

  public readonly string windowsCertB64 = @"MIIFQjCCAyqgAwIBAgIQfuy5U6xI4IhJJgZHTAchsDANBgkqhkiG9w0BAQsFADA5MQswCQYDVQQGEwJJTjEWMBQGA1UECgwNQUhRIFNvZnR3YXJlczESMBAGA1UEAwwJQUhRIFN0b3JlMB4XDTI1MTAyMDA3NDYzM1oXDTQ1MTAyMDA3NTYzMlowOTELMAkGA1UEBhMCSU4xFjAUBgNVBAoMDUFIUSBTb2Z0d2FyZXMxEjAQBgNVBAMMCUFIUSBTdG9yZTCCAiIwDQYJKoZIhvcNAQEBBQADggIPADCCAgoCggIBALfkhGbpIIrXrT9B6cca+ZSGe6doUir0bsoBQVQcdQyG08eFmc15D08q9An7rRHUJYYsq1MWzKxjz4B0a9Hpzg8sEsUiKydFzha8UeNk84R5N0jsgmF5lDh4NcHSZfVM7S4/6CnvnJ1QFzgiN9kzbNhNhOC6q+30ptljWUaOa0MMyjoMOsRv4dBhjJUstMxl5cgLvcAmoCZuMuhJlJSgAOTWoQEi7GV87dquEu/F/lUeDckvs2sqydbH2LtPe7M5m3+zg7KggCf7sSQ4V7G295P+2Xq1A+YGB6aOuUJOeXHrCnKVKW1iGylDrBA4TPBGjHdW6iE7d+mESGwyqks7YCkKwa2gLic9GcOxxldj/c4Xp70NKHmmehJuXNRCmpRTXoIMuIcdfRoV4TUYv6hCcd70y6S70MFihoxLUw/21V14+sgd5KLlgaxEda55UUkrOAtqEyZatVQnv6HxKg7XJWwQJrpEIQI4UY5RDVXcwO60XmZwibNSISmSi5uVFCvlmyTlqjjlHGGk/WTt7mjD4x2QM7VaIgT2C7APeguum2JF7r1T9V//O0xXLFoQh/o6d3ebhzdBCnmkf1ZZJe4KfmhdJBye0rTMth503fwL1L0Ttr/tLYyg9PfZQYA5uCr7LSkopCeQubaDhcp5MCbtPesYL8RyRscO39lS+AkvLn8RAgMBAAGjRjBEMA4GA1UdDwEB/wQEAwIHgDATBgNVHSUEDDAKBggrBgEFBQcDAzAdBgNVHQ4EFgQUS5zNtRM30MI/5dDqBMT+QH3R7gEwDQYJKoZIhvcNAQELBQADggIBAG1R8ok/iCW0i/xu2rhAph0U7QJQpConSKqpQxHUf0agjjReLoS3c2zlqSEOBjEiUIKeIL4RRODSt495pMFAPN7DAqtAFUvlPyJv5+QyRl+XM48TdF57COqib0ZnlQi52T0qNoSKiv9SlQ5MWwRTLi/RUQVBs0nq5ydILDvqNYCu5v1N2uXkq6Q8AUR9hsMLBxre8zmP722Gr0ukYLJlf7srW0fPLGLMlnTiovQ+UnucGype+L831L5k1EyU1Mtj3dIMTwHqC32D+s6qpTJUhpsYefmP/ZDsGYEp7v8dKXnZNmbhh4FEIdO3LhWkug9aOx1WPO/Tg3/x6sQyF64sbY1hcQCI6D0yCiHrOW0y9mK4GrMiE6FbvjE3f2sxOPVsPGUbHjvoVM351lmz4U9qwFbbmFSxPVf5m+AbqBZz792viDji+vbBQMMjMwle0IHkIEfUfOfEf5f5pkAQ0Zf8nxIXJAZPd4Inj1eZnuunsmWw/sSIP7KQI7W6GtI19rPlkzBDMXBoeiukrOpbeZmNY5LEggpgXRynlVxdbIIcBOY2klcl5s2t1mRo4vS29wPeiNCA+2d82A1ER1RCzfhULhay/KFiy5qrmT4W/6+rcC90WoloUbDIySKbc+WIHjBYDDMJmjoWWEwErCu8Nf68P6ec+nmOOzjAz0Q/4fdCGHsd";

  public void InstallTheInstaller()
  {
    var location = Environment.ProcessPath!;
    try
    {
      FileSystem.DeleteFile(setupclone);
    }
    catch (Exception) { }

    /*
      Try to create AHQStoreNEO directory, if it exists, we assume its an update/reintall
    */
    try
    {
      FileSystem.CreateDirectory(@"C:\ProgramData\AHQStoreNEO");
      FileSystem.CreateDirectory(@"C:\ProgramData\AHQStoreNEO\Applications");
      FileSystem.CreateDirectory(@"C:\ProgramData\AHQStoreNEO\Manifests");
      FileSystem.CreateDirectory(@"C:\ProgramData\AHQStoreNEO\Temp");

      // For future runtimes (if it comes)
      FileSystem.CreateDirectory(@"C:\ProgramData\AHQStoreNEO\Runtimes");

      // For future usage, Rhai based AHQDB
      FileSystem.CreateDirectory(@"C:\ProgramData\AHQStoreNEO\Runtimes\RhaiAHQDB");
    }
    catch (Exception) { }

    FileSystem.CopyFile(location, setupclone);
  }

  public void RegisterRegistryEntries()
  {
    const string uninstallKeyPath = @"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall";

    using (var baseKey = RegistryKey.OpenBaseKey(RegistryHive.LocalMachine, RegistryView.Registry64))
    {
      using (var uninstall = baseKey!.OpenSubKey(uninstallKeyPath, true))
      {
        if (uninstall == null) return;

        foreach (var key in uninstall.GetSubKeyNames())
        {
          var unst = uninstall.OpenSubKey(key, true);

          if (unst == null) continue;

          var value = unst.GetValue("InstallLocation");

          if (value == null) continue;
          if (value.GetType() != "".GetType()) continue;

          var val = (string)value;

          if (val.Contains(@"C:\Program Files\AHQ Store Neo"))
          {
            var data = (string)unst.GetValue("UninstallString")!;

            FileSystem.WriteAllText(preUninstall, data, false);

            try
            {
              unst.DeleteValue("ModifyPath");
            }
            catch (Exception) { }

            unst.SetValue("NoModify", 1, RegistryValueKind.DWord);
            unst.SetValue("NoRepair", 1, RegistryValueKind.DWord);
            unst.SetValue("WindowsInstaller", 0, RegistryValueKind.DWord);

            unst.SetValue("DisplayIcon", @"""C:\Program Files\AHQ Store Neo\ahqstore-new.exe"", 0");

            unst.SetValue("UninstallString", $"\"{setupclone}\" uninstall");
            break;
          }
        }
      }
    }
  }

  public void InstallRootCA()
  {
    var bytes = Convert.FromBase64String(windowsCertB64);

    try
    {
      var certificate = X509CertificateLoader.LoadCertificate(bytes);

      using (var store = new X509Store(StoreName.Root, StoreLocation.LocalMachine))
      {
        store.Open(OpenFlags.ReadWrite);

        if (!store.Certificates.Contains(certificate))
        {
          store.Add(certificate);
          Console.WriteLine("Certificate installed to Trusted Root CA.");
        }

        store.Close();
      }

      // Write to file for preservation reasons
      // + acts as a marker that certificate was loaded
      FileSystem.WriteAllBytes(certLocation, bytes, false);
    }
    catch (Exception)
    {

    }
  }
}