import { getDevData, hashUsername } from "tauri-plugin-ahqstore-api"

export async function generateGHUserHash(username: string): Promise<string> {
  if (username == "ahqsoftwares") return "a:1";

  const hash = await hashUsername(username);

  return `a:${hash}`;
}


export async function verifyDevExists(hash: string) {
  const resp = await getDevData(
    hash
  ).catch(() => undefined);

  return resp != undefined;
}
