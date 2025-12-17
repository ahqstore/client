import { getAppWrapped } from "@/api/fetchApps";
import { useEffect, useState } from "react";

import { useInView } from "react-intersection-observer";
import { AHQStoreApplication } from "src-ahqstore-types/pkg/ahqstore_types";

export interface PanelProps {
  appId: string;
  set: (_: number) => void;
}

export default function DesktopVerticalPanel({ appId }: PanelProps) {
  const { ref, inView } = useInView({
    threshold: 0,
    triggerOnce: true
  });

  const [app, setApp] = useState<[AHQStoreApplication, string] | undefined>();

  useEffect(() => {
    if (inView) {
      getAppWrapped(appId).then(setApp);
    }
  }, [inView]);

  return <div ref={ref} className="border border-white">
    <h2>{`Header inside viewport ${inView}.`}</h2>

    Desktop {app?.[0].appDisplayName}
  </div>
}