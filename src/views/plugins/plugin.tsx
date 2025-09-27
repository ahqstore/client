import { AStorePluginManager, PluginFlags } from "@/api/plugin/mgnt";
import { Category } from "@/components/category"
import { Button } from "@/components/ui/button";
import { Switch } from "@fluentui/react-components";
import { invoke } from "@tauri-apps/api/core";
import { Puzzle, Settings, ExternalLink, AlertTriangle } from "lucide-react"

import { useMemo } from "react";

export interface PluginProps {
  id: string;
}
export function Plugin({ id }: PluginProps) {
  const { displayName, flags } = useMemo(() => AStorePluginManager.getInstance().uiPlugins.get(id)!!, [id]);
  const worker = useMemo(() => AStorePluginManager.getInstance().workerPlugins.get(id), [id]);

  const isWorker = flags.has(PluginFlags.WorkerPlugin);

  return <Category
    title={displayName}
    description={isWorker ? (worker ? "Enabled. Click to disable." : "Disabled. Click to enable.") : "Not worker plugin"}
    Icon={Puzzle}
    nearChevron={
      <div className="flex gap-1">
        <Button
          variant={"ghost"}
          disabled={!flags.has(PluginFlags.PluginUI)}
          onClick={() => {
            invoke("open_plugin", {
              plugin: id,
              title: `Plugin - ${displayName}`,
              settings: false
            });
          }}
        >
          <ExternalLink />
        </Button>
        <Button
          variant={"ghost"}
          disabled={!flags.has(PluginFlags.SettingsUI)}
          onClick={() => {
            invoke("open_plugin", {
              plugin: id,
              title: `Plugin Settings - ${displayName}`,
              settings: true
            });
          }}
        >
          <Settings />
        </Button>
        {isWorker &&
          <Switch
            defaultChecked={worker != null}
            onChange={async (_, data) => {
              let val = JSON.parse(localStorage.getItem("enabled-plugins")!!) as string[];

              if (data.checked) {
                val.push(id);
              } else {
                val = val.filter((x) => x != id);
              }

              localStorage.setItem("enabled-plugins", JSON.stringify(val));
              window.location.reload();
            }}
          />}
      </div>

    }
    normallyOpen
    forceOpen
    openable
  // pointer={flags.has(PluginFlags.PluginUI)}
  >
    <div className="w-full flex flex-col gap-2">
      {isWorker && <div role="alert" className="alert alert-warning alert-soft">
        <AlertTriangle className="size-6" />

        <span>
          {worker != null ? "Enabled worker plugin" : "This worker plugin is disabled."}
        </span>
      </div>}


    </div>
  </Category >
}