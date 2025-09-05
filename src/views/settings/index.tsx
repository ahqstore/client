import { ConfigSelect } from "@/components/select";
import { Label, Switch } from "@fluentui/react-components";

import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"

import { ZoomIn, Aperture, Moon, Sparkles, DatabaseBackup } from "lucide-react";

import { useContext, useEffect, useMemo, useState } from "react";

import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart"
import { platform } from "@tauri-apps/plugin-os";

import { setScale, refreshCommit } from "tauri-plugin-ahqstore-api"
import { setUITheme, ThemeContext, VibrantWindows } from "@/lib";
import { Separator } from "@/components/ui/separator";
import { Category } from "@/components/category";

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
      <ConfigSelect
        title="Dark"
        description="Use the dark theme"
        Icon={Moon}
      >
        <Switch
          defaultChecked={useContext(ThemeContext)}
          onChange={async (_, data) => {
            const checked = data.checked;

            setUITheme(checked);
          }}
        />
      </ConfigSelect>

      {
        useContext(VibrantWindows) &&
        <ConfigSelect
          title="Vibrant UI"
          description="Congrats! You're experiencing the vibrant version of AHQ Store UI (Exclusive to Win11)"
          Icon={Sparkles}
        >

        </ConfigSelect>
      }

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
          <SelectContent className="border-base-300 dark:border-base-300 bg-accent">
            <SelectItem value="0.5">50%</SelectItem>
            <SelectItem value="0.75">75%</SelectItem>
            <SelectItem value="0.8">80%</SelectItem>
            <SelectItem value="0.9">90%</SelectItem>
            <SelectItem value="1">100%</SelectItem>
            <SelectItem value="1.25">125%</SelectItem>
            <SelectItem value="1.5">150%</SelectItem>
            <SelectItem value="1.75">175%</SelectItem>
            <SelectItem value="2">200%</SelectItem>
          </SelectContent>
        </Select>
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

      <Separator />

      <Label className="mr-auto !text-xl">Advanced</Label>

      <ConfigSelect
        title="Refetch Data"
        description="Click this button to reload and refetch full data"
        Icon={DatabaseBackup}
        pointer
        onClick={() => {
          (async () => {
            await refreshCommit();
            window.location.reload();
          })()
        }}
      >
      </ConfigSelect>

      <Separator />

      <Label className="mr-auto !text-xl">Attributions</Label>

      <Category
        title="About Us"
        description="Making App Distribution Accessible to Everyone"
        Icon={function (props: { size: string; }) {
          return <img src="/icon.png" style={{ width: props.size, height: props.size }} />;
        } as any}
      >
        <></>
      </Category>
    </div >
  </>;
}