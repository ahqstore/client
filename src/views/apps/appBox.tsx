import { useEffect, useState } from "react"
import { getApp, getAppAsset } from "tauri-plugin-ahqstore-api";
import ShowSpinner from "../spinner";
import { AHQStoreApplication } from "src-ahqstore-types/pkg/ahqstore_types";
import { getKeyFromCache, setKeyToCache } from "@/data/appCache";
import { openApplicationState } from "@/data/implementations/appView";

export function AppBox({ appId, set }: { appId: string, set: (_: number) => void }) {
  const [application, setApp] = useState<"loading" | AHQStoreApplication>("loading")
  const [img, setImg] = useState<string | undefined>(undefined);

  useEffect(() => {
    (async () => {
      setApp("loading");

      const data = await getKeyFromCache(appId);

      if (data) {
        setApp(data[0]);
        setImg(data[1]);
        return;
      }

      const [app, img] = await Promise.all([
        getApp(appId),
        getAppAsset(appId, "0")
      ]);

      setApp(app);

      const blob = new Blob([img as unknown as any]);
      const uri = URL.createObjectURL(blob);

      setImg(uri);

      await setKeyToCache(appId, [app, uri]);
    })()
  }, [appId]);

  if (application == "loading" || img == undefined) {
    return <div className="w-full h-full flex items-center justify-center text-center">
      <ShowSpinner />
    </div>;
  }

  return <div className="w-full h-full flex flex-col sm:flex-row sm:gap-2" onClick={() => {
    openApplicationState.data = appId;
    set(10);
  }}>
    <img className="mx-auto sm:m-0 size-16 sm:size-auto" src={img} />
    <div className="w-full h-full flex flex-col overflow-hidden gap-2">
      <h1 className="mt-auto mx-auto sm:m-0 text-xl font-bold font-sans">{application.appDisplayName}</h1>
      <h2 className="hidden sm:block">{application.description || "Click to view more"}</h2>
    </div>
  </div>
}