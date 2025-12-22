import { getAppWrapped } from "@/api/fetchApps";
import { openApplicationState } from "@/data/implementations/appView";
import { memo, useEffect, useState } from "react";

import { useInView } from "react-intersection-observer";
import { AHQStoreApplication } from "src-ahqstore-types/pkg/ahqstore_types";

export interface PanelProps {
  appId: string;
  set: (_: number) => void;
}

const MobileVerticalPanel = memo(function MobileVerticalPanel({ appId, set }: PanelProps) {
  const { ref, inView } = useInView({
    threshold: 0,
    triggerOnce: true
  });

  const [[app, icon], setApp] = useState<[AHQStoreApplication, string] | [undefined, undefined]>([undefined, undefined]);

  const launch = () => {
    openApplicationState.data = appId;
    set(10);
  };

  useEffect(() => {
    if (inView) {
      (async () => {
        const [app, icon] = await getAppWrapped(appId);

        setApp([app, icon]);
      })();
    }
  }, [inView]);

  if (app == undefined) {
    return <div ref={ref} className="flex! flex-row! justify-center! items-center! text-center!">
      <span className="loading loading-spinner loading-xl" />
    </div>
  }

  return <div ref={ref} className="flex! flex-row! overflow-hidden gap-2" onClick={() => launch()}>
    <img className="size-auto m-0 border-none rounded-xl" src={icon} />
    <div className="w-full h-full flex flex-col overflow-hidden gap-1 sm:gap-2">
      <h1 className="mt-1 sm:mt-2 sm:mb-1 mr-auto sm:m-0 text-sm sm:text-lg font-medium sm:font-bold font-sans line-clamp-2">{app.appDisplayName}</h1>
      <h2 className="overflow-hidden line-clamp-1">{app.description || "Click to view more"}</h2>
    </div>
  </div>;
});

export default MobileVerticalPanel;