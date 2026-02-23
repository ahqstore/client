using System.Diagnostics;
using System.IO;
using System.Security.Cryptography.X509Certificates;
using Microsoft.VisualBasic.FileIO;

public class AHQStoreCoreUninstaller
{
  public readonly string apps = @"C:\ProgramData\AHQStoreNEO";

  public readonly string users = @"C:\Users";

  public async Task Uninstall(
    Action<string> cb
  )
  {
    cb("Uninstalling AHQ Store Core");

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

    UninstallMsi();

    cb("Uninstalling AHQ Store Apps");

    UninstallApps();

    cb("Finishing up");

    Finish();

    cb("Uninstalled!");
  }

  public readonly string msiUninstall = @"C:\Program Files\AHQ Store Neo\.msiexec";

  public void UninstallMsi()
  {
    var execString = FileSystem.ReadAllText(msiUninstall).Split(" ");

    var proc = new Process();
    proc.StartInfo.FileName = execString[0];
    proc.StartInfo.ArgumentList.Add(execString[1]);
    proc.StartInfo.ArgumentList.Add("/qn");
    proc.StartInfo.Verb = "runas";

    proc.StartInfo.CreateNoWindow = true;
    proc.StartInfo.CreateNewProcessGroup = true;

    proc.Start();
    proc.WaitForExit();
  }

  public readonly string inst = @"C:\Program Files\AHQ Store Neo\";

  public readonly string certLocation = @"C:\Program Files\AHQ Store Neo\windows.cer";

  public void Finish()
  {
    try
    {
      var certificate = X509CertificateLoader.LoadCertificateFromFile(certLocation);

      using (var store = new X509Store(StoreName.Root, StoreLocation.LocalMachine))
      {
        store.Open(OpenFlags.ReadWrite);

        if (store.Certificates.Contains(certificate))
        {
          store.Remove(certificate);
        }

        store.Close();
      }

      FileSystem.DeleteDirectory(inst, DeleteDirectoryOption.DeleteAllContents);
    }
    catch (Exception)
    {

    }

  }

  public void UninstallApps()
  {
    FileSystem.DeleteDirectory(apps, DeleteDirectoryOption.DeleteAllContents);

    foreach (var user in FileSystem.GetDirectories(users))
    {
      var userahqstore = Path.Combine(user, "AHQStoreNEO");

      if (FileSystem.DirectoryExists(userahqstore))
      {
        try
        {
          FileSystem.DeleteDirectory(userahqstore, DeleteDirectoryOption.DeleteAllContents);

        }
        catch (Exception) { }

      }
    }
  }

}

public class AHQStoreUninstallerEligibility
{
  public readonly string apps = @"C:\ProgramData\AHQStoreNEO\Applications";

  public readonly string users = @"C:\Users";

  public bool Eligible()
  {
    var files = FileSystem.GetDirectories(apps).Count == 0;

    var files2 = (FileSystem.GetDirectories(users)?.All((user) =>
    {
      var userahqstore = Path.Combine(user, "AHQStoreNEO", "Applications");

      if (FileSystem.DirectoryExists(userahqstore))
      {
        try
        {
          return FileSystem.GetDirectories(userahqstore).Count == 0;
        }
        catch (Exception) { }

        return false;
      }

      return true;
    })).GetValueOrDefault(false);

    return files && files2;
  }

  public void RunMainUninstaller()
  {
    var tempFolder = SpecialDirectories.Temp;
    var guid = Guid.NewGuid();
    var tempFile = Path.Combine(tempFolder, $"ahqstore_cleanup_{guid}.exe");

    FileSystem.CopyFile(Environment.ProcessPath!, tempFile);

    var proc = new Process();
    proc.StartInfo.FileName = tempFile;
    proc.StartInfo.ArgumentList.Add("uninstall-step2");
    proc.StartInfo.Verb = "runas";

    proc.StartInfo.CreateNoWindow = false;
    proc.StartInfo.CreateNewProcessGroup = true;

    proc.Start();
  }
}