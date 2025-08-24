import { ConfigSelect } from "@/components/select";
import { Select, Switch } from "@fluentui/react-components";
import { ZoomIn, Aperture } from "lucide-react";

import { useEffect, useMemo, useState } from "react";

import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart"
import { platform } from "@tauri-apps/plugin-os";

import { setScale } from "tauri-plugin-ahqstore-api"

export default function Settings() {
  const pc = useMemo(() => platform() != "android", []);

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
      <ConfigSelect
        title="Zoom"
        description="Set your zoom level"
        Icon={ZoomIn}
      >
        {autostart == null ?
          <span className="loading loading-spinner text-primary"></span>
          :
          <Select
            onClick={() => {
              setScale(1.25);
            }}
          >
            <option>125%</option>
            <option>100%</option>
            <option>75%</option>
          </Select>
        }
      </ConfigSelect>

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

              if (checked) {
                await disable()
              } else {
                await enable();
              }

              loadAutostart();
            }}
          />
        }
      </ConfigSelect>}

    </div>
  </>;
}