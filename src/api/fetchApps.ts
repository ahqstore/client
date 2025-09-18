import { getKeyFromCache, setKeyToCache } from "@/data/appCache";
import { fetch } from "@tauri-apps/plugin-http";
import { AHQStoreApplication } from "ahqstore-types";
import { getApp, getAppAsset } from "tauri-plugin-ahqstore-api";

export async function getAppWrapped(appId: string): Promise<[AHQStoreApplication, string]> {
  const data = await getKeyFromCache(appId);

  if (data) {
    return data;
  }

  const [app, img] = await Promise.all([
    getApp(appId),
    getAppAsset(appId, "0")
  ]);

  const blob = new Blob([img as unknown as any]);

  let uri = URL.createObjectURL(blob);
  if (appId.startsWith("f:")) {
    const data = await fetch(await blob.text())
    const b = await data.blob();

    uri = URL.createObjectURL(b);
  }


  await setKeyToCache(appId, [app, uri]);

  return [app, uri];
}