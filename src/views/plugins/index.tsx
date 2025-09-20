import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";

export default function PluginPage() {
  useEffect(() => {
    invoke("open_plugin", {
      plugin: "1",
      title: "AHQ Store Test",
      settings: true
    });
  }, []);

  return <>

  </>;
}