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

    UninstallMsi();

    cb("Uninstalling AHQ Store Apps");

    UninstallApps();

    cb("Finishing up");

    Finish();

    cb("Uninstalled!");
  }

  public readonly string preUninstall = @"C:\Program Files\AHQ Store Neo\.msiexec";

  public void UninstallMsi()
  {

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
      var userahqstore = Path.Combine(users, user, "AHQStoreNEO");

      if (FileSystem.DirectoryExists(userahqstore))
      {
        try
        {
          FileSystem.DeleteDirectory(apps, DeleteDirectoryOption.DeleteAllContents);

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
      var userahqstore = Path.Combine(users, user, "AHQStoreNEO", "Applications");

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