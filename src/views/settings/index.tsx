import { ConfigSelect } from "@/components/select";
import { Switch } from "@fluentui/react-components";

import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"

import { ZoomIn, Aperture } from "lucide-react";

import { useEffect, useMemo, useState } from "react";

import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart"
import { platform } from "@tauri-apps/plugin-os";

import { setScale } from "tauri-plugin-ahqstore-api"

export default function Settings() {
  const pc = useMemo(() => platform() != "android", []);
  const defaultZoom = useMemo(() => localStorage.getItem("defZoom") || "1", []);

  const [autostart, setAutoStart] = useState<null | boolean>(null);

  const loadAutostart = async () => {
    setAutoStart(await isEnabled());
  };

  // Init
  useEffect(() => {
    (async () => {
      if (pc) {
        await loadAutostart();
      }
    })()
  }, []);

  return <>
    <div className="flex flex-col gap-2 w-full justify-center items-center">
      {pc && <ConfigSelect
        title="Zoom"
        description="Set your zoom level"
        Icon={ZoomIn}
      >
        <Select
          defaultValue={defaultZoom}
          onValueChange={(val) => {
            localStorage.setItem("defZoom", val);
            setScale(Number(val));
          }}
        >
          <SelectTrigger className="w-32 md:w-48">
            <SelectValue placeholder="Select Zoom" />
          </SelectTrigger>
          <SelectContent className="dark:border-base-300 bg-accent">
            <SelectItem value="0.5">50%</SelectItem>
            <SelectItem value="0.75">75%</SelectItem>
            <SelectItem value="1">100%</SelectItem>
            <SelectItem value="1.25">125%</SelectItem>
            <SelectItem value="1.5">150%</SelectItem>
            <SelectItem value="1.75">175%</SelectItem>
            <SelectItem value="2">200%</SelectItem>
          </SelectContent>
        </Select>
        {/* <Select
          onClick={() => {
            setScale(1.25);
          }}
        >
          <option>125%</option>
          <option>100%</option>
          <option>75%</option>
        </Select> */}
      </ConfigSelect>}

      {/* Autostart is PC only setting */}
      {pc && <ConfigSelect
        title="Autostart"
        description="Start AHQ Store on startup (uses minimal resources)"
        Icon={Aperture}
      >
        {autostart == null ?
          <span className="loading loading-spinner text-primary"></span>
          :
          <Switch
            checked={autostart}
            onChange={async (_, data) => {
              const checked = data.checked;

              // Now it'll be set
              if (checked) {
                await enable()
              } else {
                await disable();
              }

              loadAutostart();
            }}
          />
        }
      </ConfigSelect>}

    </div >
  </>;
}