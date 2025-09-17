import { AHQStoreApplication } from "ahqstore-types";
import { getCommit } from "tauri-plugin-ahqstore-api"

let cache: Map<String, [AHQStoreApplication, string]> = new Map();
let commit: string | undefined = undefined;

export async function getKeyFromCache(key: string): Promise<[AHQStoreApplication, string] | undefined> {
  const ahqstoreCommit = (await getCommit()).ahqstore;

  if (commit != ahqstoreCommit) {
    commit = ahqstoreCommit;
    cache.clear();

    return undefined;
  }

  return cache.get(key);
}

export async function setKeyToCache(key: string, val: [AHQStoreApplication, string]) {
  const ahqstoreCommit = (await getCommit()).ahqstore;

  if (commit != ahqstoreCommit) {
    commit = ahqstoreCommit;
    cache.clear();

    return undefined;
  }

  cache.set(key, val);
}