import { Repo } from "@/views/developer/applyApp";
import { fetch } from "@tauri-apps/plugin-http";
import { decrypt } from "tauri-plugin-ahqstore-api";

interface RepoData {
  private: boolean
}

interface ReleasesData {
  assets: {
    name: string;
    browser_download_url: string;
  }[]
}

export async function verifyRepo(repo: Repo): Promise<string> {
  const rawToken = JSON.parse(
    localStorage.getItem("token") || "[]",
  ) as number[];

  const token = await decrypt(rawToken);

  try {
    const output: RepoData = await fetch(
      `https://api.github.com/repos/${repo.author}/${repo.repo}`,
      {
        headers: {
          "User-Agent": "AHQ Store Submissions",
          Authorization: `Bearer ${token}`
        }
      }
    ).then((r) => {
      if (!r.ok) {
        throw new Error("Failed to fetch repo")
      }

      return r.json();
    });

    if (output.private) {
      return "The repository is private and hence can't be added to AHQ Store Community Repository"
    } else {
      return ""
    }
  } catch (_) {
    return "Unable to fetch repository. The repository probably does not exist or is private."
  }
}

export interface Output {
  error: boolean;
  output: string;
}

export async function getAHQStoreManifest(repo: Repo): Promise<Output> {
  const rawToken = JSON.parse(
    localStorage.getItem("token") || "[]",
  ) as number[];

  const token = await decrypt(rawToken);

  try {
    const output: ReleasesData = await fetch(
      `https://api.github.com/repos/${repo.author}/${repo.repo}/releases/latest`,
      {
        headers: {
          "User-Agent": "AHQ Store Submissions",
          Authorization: `Bearer ${token}`
        }
      }
    ).then((r) => {
      if (!r.ok) {
        throw new Error("Failed to fetch repo")
      }

      return r.json();
    });

    const file = output.assets.find((x) => x.name == "ahqstore.json");

    if (!file) {
      return {
        error: true,
        output: "`ahqstore.json` is not present in the `latest` release assets"
      }
    }

    return {
      error: false,
      output: file.browser_download_url
    }
  } catch (_) { }

  return {
    error: true,
    output: "Unable to fetch `ahqstore.json` from your GitHub Releases"
  }
}