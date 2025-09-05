import { fetch } from "@tauri-apps/plugin-http";
import { Auth, User } from ".";
import { decrypt, encrypt } from "tauri-plugin-ahqstore-api";

import { verifyDevExists, generateGHUserHash } from "./hash";

export function onAuthChange(auth: Auth, callback: (auth?: User) => void) {
  auth.onAuthChange.push(callback);
}

export async function tryAutoLogin(auth: Auth) {
  try {
    const rawToken = JSON.parse(
      localStorage.getItem("token") || "[]",
    ) as number[];

    const token = await decrypt(rawToken);

    await login(auth, token);
  } catch (_) { }
}

export async function login(auth: Auth, auth_tok: string): Promise<boolean> {
  const { ok, data } = await fetch(`https://api.github.com/user`, {
    method: "GET",
    headers: {
      Authorization: `Bearer ${auth_tok}`,
    },
    connectTimeout: 100_000,
  }).then(async (d) => ({ ...d, ok: d.ok, data: await d.json() }));

  if (ok) {
    const hash = await generateGHUserHash(data.login);

    auth.currentUser = {
      ...data,
      dev: await verifyDevExists(hash),
    };
    auth.loggedIn = true;

    localStorage.setItem("token", JSON.stringify(await encrypt(auth_tok)));

    auth.onAuthChange.forEach((cb) => cb(auth.currentUser));
  } else {
    auth.onAuthChange.forEach((cb) => cb(undefined));
  }

  return ok;
}
