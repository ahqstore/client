using System.Net.Http;
using System.Net.Http.Json;
using System.Runtime.InteropServices;
using System.Text.Json.Serialization;

namespace AHQStoreWin32Setup;

public class Commit
{
  [JsonPropertyName("sha")]
  public string SHA { get; set; } = string.Empty;
}

public class Release
{
  [JsonPropertyName("assets")]
  public List<Asset> Assets { get; set; } = [];
}

public class Asset
{
  [JsonPropertyName("name")]
  public string NAME { get; set; } = "";

  [JsonPropertyName("browser_download_url")]
  public string Url { get; set; } = "";
}

public class Urls
{
  public string? Prerelease;
  public string? Release;
}

public class GitHubService
{
  string commits = "https://api.github.com/repos/ahqstore/ahqstore.github.io/commits";

  string releases = "https://api.github.com/repos/ahqstore/client/releases";
  string latest = "https://api.github.com/repos/ahqstore/client/releases/latest";

  string terms_of_service = "https://cdn.jsdelivr.net/gh/ahqstore/ahqstore.github.io@{sha}/docs/tos.mdx";

  string privacy_policy = "https://cdn.jsdelivr.net/gh/ahqstore/ahqstore.github.io@{sha}/docs/privacy.mdx";


  HttpClient client = new HttpClient();

  public async Task Setup()
  {
    client.DefaultRequestHeaders.UserAgent.ParseAdd("AHQStore");
    client.Timeout = TimeSpan.FromSeconds(30);

    var output = (await client.GetFromJsonAsync<List<Commit>>(commits))![0]!;

    terms_of_service = terms_of_service.Replace("{sha}", output.SHA);
    privacy_policy = privacy_policy.Replace("{sha}", output.SHA);
  }

  public async Task<string> FetchTOS()
  {

    string tos = await client.GetStringAsync(terms_of_service);

    tos = tos.Replace("""
---
title: Terms and Conditions
sidebar: false
---
""", "").Replace("[Usage Guidelines](#usage-guidelines)", "**Usage Guidelines**").Replace("(/", "(https://ahqstore.github.io/");

    return tos;
  }

  public async Task<string> FetchPP()
  {
    string pp = await client.GetStringAsync(privacy_policy);

    pp = pp.Replace("""
---
title: Privacy Policy
sidebar: false
---
""", "").Replace("[Your Data Rights](#your-data-rights)", "**Your Data Rights**").Replace("(/", "(https://ahqstore.github.io/");

    return pp;
  }

  public async Task<Urls> GetMSIUrls()
  {
    Release? data = await client.GetFromJsonAsync<Release>(latest);

    Release? prereleases = (await client.GetFromJsonAsync<List<Release>>(releases))?.First();

    Predicate<Asset> reducer = (asset) =>
      {
        Architecture arch = RuntimeInformation.OSArchitecture;

        if (arch == Architecture.X64)
        {
          return asset.NAME.Contains("_x64_") && asset.NAME.EndsWith(".msi");
        }

        if (arch == Architecture.Arm64)
        {
          return asset.NAME.Contains("_arm64_") && asset.NAME.EndsWith(".msi");
        }

        return false;
      };

    Asset? asset = data?.Assets.Find(reducer);
    Asset? preasset = prereleases?.Assets.Find(reducer);

    Urls output = new Urls();

    if (asset != null)
    {
      output.Release = asset.Url;
    }

    if (preasset != null)
    {
      output.Prerelease = preasset.Url;
    }

    return output;
  }
}