import { getAppWrapped } from "@/api/fetchApps";
import { openApplicationState } from "@/data/implementations/appView";
import { memo, useEffect, useState } from "react";

import { useInView } from "react-intersection-observer";
import type { AHQStoreApplication } from "src-ahqstore-types/pkg/ahqstore_types";
import { getAppAsset } from "src-plugin/dist-js";

export interface PanelProps {
  appId: string;
  set: (_: number) => void;
}

const DesktopVerticalPanel = memo(function DesktopVerticalPanel({ appId, set }: PanelProps) {
  const { ref, inView } = useInView({
    threshold: 0,
    triggerOnce: true
  });

  const [[app, icon], setApp] = useState<[AHQStoreApplication, string] | [undefined, undefined]>([undefined, undefined]);

  const [image, setImage] = useState("loading");

  const launch = () => {
    openApplicationState.data = appId;
    set(10);
  };

  useEffect(() => {
    if (inView) {
      (async () => {
        const [app, icon] = await getAppWrapped(appId);

        if (app.displayImages.length != 0) {
          const iconId = app.displayImages[0];

          const img = await getAppAsset(appId, iconId.toString());
          const blob = new Blob([img as unknown as any]);

          let uri = URL.createObjectURL(blob);
          if (appId.startsWith("f:")) {
            const data = await fetch(await blob.text())
            const b = await data.blob();

            uri = URL.createObjectURL(b);
          }

          setImage(uri);
        } else {
          setImage("null");
        }

        setApp([app, icon]);
      })();
    }
  }, [inView]);

  if (app == undefined) {
    return <div ref={ref} className="flex! flex-row! justify-center! items-center! text-center!">
      <span className="loading loading-spinner loading-xl" />
    </div>
  }

  return <div ref={ref} className="flex flex-col overflow-hidden gap-2" onClick={() => launch()}>
    <div className="w-full flex gap-1 sm:gap-2">
      <img className="size-24 min-size-24 max-size-24 m-0 border-none rounded-xl" src={icon} />
      <div className="w-full h-full flex flex-col overflow-hidden gap-1 sm:gap-2">
        <h1 className="mt-1 sm:mt-2 sm:mb-1 mr-auto sm:m-0 text-sm sm:text-lg font-medium sm:font-bold font-sans line-clamp-2">{app.appDisplayName}</h1>
        {image != "null" && <h2 className="overflow-hidden line-clamp-1">{app.description || "Click to view more"}</h2>}
      </div>
    </div>

    {image == "null" && <div className="h-full w-full overflow-clip text-clip">
      {app.description || "Click to view more"}
    </div>}
    {image != "null" && <img className="h-full w-full overflow-hidden border-0 rounded-xl dark:bg-neutral-content/10" src={image} />}
  </div>
});

export default DesktopVerticalPanel;