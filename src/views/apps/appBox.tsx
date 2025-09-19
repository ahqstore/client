import { useEffect, useState } from "react"
import ShowSpinner from "../spinner";
import { AHQStoreApplication } from "src-ahqstore-types/pkg/ahqstore_types";
import { openApplicationState } from "@/data/implementations/appView";
import { getAppWrapped } from "@/api/fetchApps";

export function AppBox({ appId, set }: { appId: string, set: (_: number) => void }) {
  const [application, setApp] = useState<"loading" | AHQStoreApplication>("loading")
  const [img, setImg] = useState<string | undefined>(undefined);

  useEffect(() => {
    (async () => {
      setApp("loading");

      const [a, img] = await getAppWrapped(appId);

      setApp(a);
      setImg(img);
    })()
  }, [appId]);

  if (application == "loading" || img == undefined) {
    return <div className="w-full h-full flex items-center justify-center text-center">
      <ShowSpinner />
    </div>;
  }

  return <div className="w-full h-full flex gap-1 sm:gap-2" onClick={() => {
    openApplicationState.data = appId;
    set(10);
  }}>
    <img className="m-0 size-auto border-none rounded-xl" src={img} />
    <div className="w-full h-full flex flex-col overflow-hidden gap-1 sm:gap-2">
      <h1 className="mt-1 sm:mt-2 sm:mb-1 mr-auto sm:m-0 text-sm sm:text-lg md:text-xl font-medium sm:font-bold font-sans">{application.appDisplayName}</h1>
      <h2 className="overflow-hidden line-clamp-1 sm:line-clamp-2">{application.description || "Click to view more"}</h2>
    </div>
  </div>
}