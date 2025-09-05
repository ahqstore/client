import { decrypt, open } from "tauri-plugin-ahqstore-api";

export interface CommentResp {
  html_url: string;
}

export async function applyForAppLaunch(manifest: string) {
  const rawToken = JSON.parse(
    localStorage.getItem("token") || "[]",
  ) as number[];

  const token = await decrypt(rawToken);

  try {
    const out: CommentResp = await fetch(
      `https://api.github.com/repos/ahqstore/repo_community/issues/29/comments`,
      {
        headers: {
          Accept: "application/vnd.github+json",
          "User-Agent": "AHQ Store Automation",
          Authorization: `Bearer ${token}`
        },
        method: "POST",
        body: JSON.stringify({
          body: `/store set ${manifest}`
        })
      }
    ).then((r) => {
      if (!r.ok) {
        throw new Error("Oopsie");
      }

      return r.json();
    });

    await open(out.html_url);
  } catch (_) {
    await open("https://ahqstore.github.io/submissionFailed");
  }
}

export async function applyAsDeveloper(name: string) {
  const rawToken = JSON.parse(
    localStorage.getItem("token") || "[]",
  ) as number[];

  const token = await decrypt(rawToken);

  const link = btoa(
    encodeURIComponent(
      JSON.stringify({
        name
      })
    )
  );

  try {
    const out: CommentResp = await fetch(
      `https://api.github.com/repos/ahqstore/repo_community/issues/29/comments`,
      {
        headers: {
          Accept: "application/vnd.github+json",
          "User-Agent": "AHQ Store Automation",
          Authorization: `Bearer ${token}`
        },
        method: "POST",
        body: JSON.stringify({
          body: `/account create base64:${link}`
        })
      }
    ).then((r) => {
      if (!r.ok) {
        throw new Error("Oopsie");
      }

      return r.json();
    });

    await open(out.html_url);
  } catch (_) {
    await open("https://ahqstore.github.io/applyFailed");
  }
}