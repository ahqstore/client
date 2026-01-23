import { AHQStoreApplication } from "@ahqstore/core-types";
import { getCommitSmart } from "./commit";

interface MapData {
  data: [AHQStoreApplication, string];
  expires: number;
}

let cache: Map<String, MapData> = new Map();
let commit: string | undefined = undefined;

setInterval(() => {
  const now = Date.now();
  cache.forEach((val, key, map) => {
    if (now > val.expires) {
      map.delete(key);
    }
  });
}, 30 * 1000);

export async function getKeyFromCache(
  key: string,
): Promise<[AHQStoreApplication, string] | undefined> {
  const ahqstoreCommit = await getCommitSmart();

  if (commit != ahqstoreCommit) {
    commit = ahqstoreCommit;
    cache.clear();

    return undefined;
  }

  return cache.get(key)?.data;
}

export async function setKeyToCache(
  key: string,
  val: [AHQStoreApplication, string],
) {
  const ahqstoreCommit = await getCommitSmart();

  if (commit != ahqstoreCommit) {
    commit = ahqstoreCommit;
    cache.clear();
  }

  cache.set(key, {
    data: val,
    expires: Date.now() + 60 * 1000,
  });
}
