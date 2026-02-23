import { openApplicationState } from "@/data/implementations/appView";

import { useStore } from "@/data/store";

import { useEffect, useState } from "react";

import "./app.css";
import { CompoundButton } from "@fluentui/react-components";
import { ArrowDownloadRegular } from "@fluentui/react-icons";
import { getAppWrapped } from "@/api/fetchApps";

export default function Application() {
  const appId = useStore(openApplicationState);

  const [, setLoading] = useState(true);

  console.log(appId);

  useEffect(() => {
    setLoading(true);

    if (appId) {
      getAppWrapped(appId);
    }
  }, [appId]);

  return <div className="w-full h-full flex flex-col">
    <div className="w-full">
      <CompoundButton
        appearance="primary"
        icon={<ArrowDownloadRegular />}
        size="large"
        shape="rounded"
        secondaryContent={"Installs the app for current user."}

      >
        Install (user)
      </CompoundButton>
    </div>

    {appId}
  </div>;
}