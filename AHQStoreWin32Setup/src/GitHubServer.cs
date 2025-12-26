using System.Net.Http;
using System.Net.Http.Json;
using System.Text.Json.Serialization;

namespace AHQStoreWin32Setup;

public class Commit
{
  [JsonPropertyName("sha")]
  public string SHA { get; set; } = string.Empty;
}

public class GitHubService
{
  string commits = "https://api.github.com/repos/ahqstore/ahqstore.github.io/commits";

  string terms_of_service = "https://cdn.jsdelivr.net/gh/ahqstore/client@{sha}/docs/tos.mdx";

  string privacy_policy = "https://cdn.jsdelivr.net/gh/ahqstore/client@{sha}/docs/privacy.mdx";


  HttpClient client = new HttpClient();

  public async Task Setup()
  {
    client.DefaultRequestHeaders.UserAgent.ParseAdd("AHQStore");

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
}