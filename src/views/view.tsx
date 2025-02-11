import { useMediaQuery } from "@/hooks/use-media-query";

import {
  AppsRegular,
  AppsFilled,

  AppsListRegular,
  AppsListFilled,

  SettingsRegular,
  SettingsFilled,

  PersonRegular,
  PersonFilled
} from "@fluentui/react-icons";

import { platform } from "@tauri-apps/plugin-os";
import { Library, LayoutGrid, Settings, User, LibraryBig } from "lucide-react";

import { useState } from "react";
import NavigationSidebar from "./nav";

export const items:
  ({
    name: string;
    id: number;
    icon: JSX.Element,
    iconFilled: JSX.Element,
    iconMobile: JSX.Element,
    iconMobileFilled: JSX.Element,
    hidden?: boolean
  })[] = [
    {
      name: "Apps",
      id: 0,
      icon: <AppsRegular className="size-[1.5em]" />,
      iconFilled: <AppsFilled className="size-[1.5em]" style={{ color: "#ff0000" }} />,
      iconMobile: <LayoutGrid size="1.5em" />,
      iconMobileFilled: <LayoutGrid fill="currentcolor" size="1.5em" />,
    },
    {
      name: "Library",
      id: 1,
      icon: <AppsListRegular className="size-[1.5em]" />,
      iconFilled: <AppsListFilled className="size-[1.5em]" style={{ color: "#ff0000" }} />,
      iconMobile: <Library size="1.5em" />,
      iconMobileFilled: <LibraryBig size="1.5em" />,
    },
    {
      name: "Profile",
      id: 2,
      hidden: true,
      icon: <PersonRegular className="size-[1.5em]" />,
      iconFilled: <PersonFilled className="size-[1.5em]" style={{ color: "#ff0000" }} />,
      iconMobile: <User size="1.5em" />,
      iconMobileFilled: <User fill="currentcolor" size="1.5em" />,
    },
    {
      name: "Settings",
      id: 3,
      icon: <SettingsRegular className="size-[1.5em]" />,
      iconFilled: <SettingsFilled className="size-[1.5em]" style={{ color: "#ff0000" }} />,
      iconMobile: <Settings size="1.5em" />,
      iconMobileFilled: <Settings className="rotate-12" size="1.5em" />,
    }
  ];

export function ApplicationView() {
  const [item, setItem] = useState(0);

  const desktop = useMediaQuery("(min-width: 640px)");

  if (desktop) {
    return <div className="mt-2 w-full h-full flex">
      <div className="h-full w-20 flex flex-col gap-2 px-2 pb-2 items-center text-center">
        <NavigationSidebar item={item} setItem={setItem} />
      </div>
      <div className="w-full h-full rounded-tl-xl p-3 bg-neutral/50">
        <h1>Hello World</h1>
      </div>
    </div>;
  }

  return <div className="w-full h-full flex flex-col">
    <BottomNavigation item={item} setItem={setItem} />
  </div>;
}

function BottomNavigation({ item, setItem }: { item: number, setItem: (_: number) => void }) {
  return <div className="dock dock-xl bg-neutral/30">
    {
      items.map((s) =>
        <button key={`${s.id}`} className={s.id === item ? "dock-active transition-all" : "transition-all"} onClick={() => setItem(s.id)}>
          {
            platform() == "android" ?
              s.id === item ? s.iconMobileFilled : s.iconMobile
              :
              s.id === item ? s.iconFilled : s.icon
          }
          <span className="dock-label">{s.name}</span>
        </button>
      )
    }
  </div>;
}