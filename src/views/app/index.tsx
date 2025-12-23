import { openApplicationState } from "@/data/implementations/appView";

import { useStore } from "@/data/store";

import { useEffect, useState } from "react";

import "./app.css";

export default function Application() {
  const appId = useStore(openApplicationState);

  const [loading, setLoading] = useState(true);

  console.log(appId);

  useEffect(() => {
    setLoading(true);
  }, [appId]);

  return <div className="w-full h-full flex flex-col">

    {appId}
  </div>;
}