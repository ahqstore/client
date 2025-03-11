import { useMediaQuery } from "@/hooks/use-media-query";

import {
  AppGenericRegular as AppsRegular,
  AppGenericFilled as AppsFilled,

  SettingsRegular,
  SettingsFilled,

  PersonRegular,
  PersonFilled,

  LibraryRegular,
  LibraryFilled,

  ToolboxRegular,
  ToolboxFilled
} from "@fluentui/react-icons";

import { platform } from "@tauri-apps/plugin-os";
import { Library, LayoutGrid, Settings, User, LibraryBig } from "lucide-react";

import { useState } from "react";
import NavigationSidebar from "./nav";
import { useExperiment } from "@/lib/experiment";
import { useAuth } from "@/lib/auth/provider";

export const items:
  ({
    name: string;
    id: number;
    icon: JSX.Element,
    iconFilled: JSX.Element,
    iconMobile: JSX.Element,
    iconMobileFilled: JSX.Element,
    hidden?: () => boolean
  })[] = [
    {
      name: "Apps",
      id: 0,
      icon: <AppsRegular className="size-[1.5em]" />,
      iconFilled: <AppsFilled className="size-[1.5em]" style={{ color: "var(--win32-accent)" }} />,
      iconMobile: <LayoutGrid size="1.5em" />,
      iconMobileFilled: <LayoutGrid fill="currentcolor" size="1.5em" />,
    },
    {
      name: "Library",
      id: 1,
      icon: <LibraryRegular className="size-[1.5em]" />,
      iconFilled: <LibraryFilled className="size-[1.5em]" style={{ color: "var(--win32-accent)" }} />,
      iconMobile: <Library size="1.5em" />,
      iconMobileFilled: <LibraryBig size="1.5em" />,
    },
    {
      name: "Profile",
      id: 2,
      hidden: () => platform() != "android",
      icon: <PersonRegular className="size-[1.5em]" />,
      iconFilled: <PersonFilled className="size-[1.5em]" style={{ color: "var(--win32-accent)" }} />,
      iconMobile: <User size="1.5em" />,
      iconMobileFilled: <User fill="currentcolor" size="1.5em" />,
    },
    {
      name: "Dev",
      id: 3,
      hidden: () => useAuth()?.dev || true,
      icon: <ToolboxRegular className="size-[1.5em]" />,
      iconFilled: <ToolboxFilled className="size-[1.5em]" style={{ color: "var(--win32-accent)" }} />,
      iconMobile: <ToolboxRegular className="size-[1.5em]" />,
      iconMobileFilled: <ToolboxFilled className="size-[1.5em]" />,
    },
    {
      name: "Lab",
      id: 6,
      hidden: () => !useExperiment(),
      icon: <ToolboxRegular className="size-[1.5em]" />,
      iconFilled: <ToolboxFilled className="size-[1.5em]" style={{ color: "var(--win32-accent)" }} />,
      iconMobile: <ToolboxRegular className="size-[1.5em]" />,
      iconMobileFilled: <ToolboxFilled className="size-[1.5em]" />,
    },
    {
      name: "Settings",
      id: 7,
      icon: <SettingsRegular className="size-[1.5em]" />,
      iconFilled: <SettingsFilled className="size-[1.5em]" style={{ color: "var(--win32-accent)" }} />,
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
      <div className="w-full h-full rounded-tl-xl p-3 bg-accent/50">
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
      items
        .filter((s) => !(s.hidden && s.hidden()))
        .map((s) =>
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