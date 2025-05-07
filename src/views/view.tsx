import { useMediaQuery } from "@/hooks/use-media-query";

import {
  AppsRegular,
  AppsFilled,

  SettingsRegular,
  SettingsFilled,

  PersonRegular,
  PersonFilled,

  LibraryRegular,
  LibraryFilled,

  WindowDevToolsFilled,
  WindowDevToolsRegular,

  MegaphoneLoudRegular,
  MegaphoneLoudFilled,

  ToolboxRegular,
  ToolboxFilled
} from "@fluentui/react-icons";

import { platform } from "@tauri-apps/plugin-os";
import { Library, LayoutGrid, Settings, User, LibraryBig, Code2Icon } from "lucide-react";

import { useMemo, useState } from "react";
import NavigationSidebar from "./nav";
import { useExperiment } from "@/lib/experiment";
import { useAuth } from "@/lib/auth/provider";
import Changelog from "./changelogs";
import { AppsHome } from "./apps";

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
      name: "Developer",
      id: 3,
      hidden: () => !(useAuth()?.dev || false),
      icon: <ToolboxRegular className="size-[1.5em]" />,
      iconFilled: <ToolboxFilled className="size-[1.5em]" style={{ color: "var(--win32-accent)" }} />,
      iconMobile: <ToolboxRegular className="size-[1.5em]" />,
      iconMobileFilled: <ToolboxFilled className="size-[1.5em]" />,
    },
    {
      name: "Lab",
      id: 6,
      hidden: () => !useExperiment(),
      icon: <WindowDevToolsRegular className="size-[1.5em]" />,
      iconFilled: <WindowDevToolsFilled className="size-[1.5em]" style={{ color: "var(--win32-accent)" }} />,
      iconMobile: <Code2Icon className="size-[1.5em]" />,
      iconMobileFilled: <Code2Icon className="size-[1.5em]" />,
    },
    {
      name: "Updates",
      id: 7,
      hidden: () => !useMediaQuery("(min-width: 640px)"),
      icon: <MegaphoneLoudRegular className="size-[1.5em]" />,
      iconFilled: <MegaphoneLoudFilled className="size-[1.5em]" style={{ color: "var(--win32-accent)" }} />,
      iconMobile: <Settings size="1.5em" />,
      iconMobileFilled: <Settings className="rotate-12" size="1.5em" />,
    },
    {
      name: "Settings",
      id: 8,
      icon: <SettingsRegular className="size-[1.5em]" />,
      iconFilled: <SettingsFilled className="size-[1.5em]" style={{ color: "var(--win32-accent)" }} />,
      iconMobile: <Settings size="1.5em" />,
      iconMobileFilled: <Settings className="rotate-12" size="1.5em" />,
    }
  ];

export function ApplicationView() {
  const [item, setItem] = useState(0);

  const desktop = useMediaQuery("(min-width: 640px)");

  const ui = useMemo(() => <GetJsx item={item} />, [item]);

  if (desktop) {
    return <div className="mt-2 w-full h-full flex overflow-hidden">
      <div className="animate h-full w-24 flex flex-col gap-2 px-1 pb-2 items-center text-center overflow-hidden overflow-y-scroll">
        <NavigationSidebar item={item} setItem={setItem} />
      </div>
      <div className="w-full h-full flex flex-col space-y-2 rounded-tl-xl p-3 bg-white/80 dark:bg-accent/50 border border-muted dark:border-none border-b-0 border-r-0 overflow-y-scroll">
        {ui}
      </div>
    </div>;
  }

  return <div className="w-full h-full flex flex-col overflow-hidden">
    <div className="h-full w-full flex flex-col space-y-2 p-2 overflow-scroll">
      {ui}
    </div>
    <BottomNavigation item={item} setItem={setItem} />
  </div>;
}


interface Props {
  item: number;
}

function GetJsx({ item }: Props) {
  switch (item) {
    case 0:
      return <AppsHome />
    case 1:
      return <>Library</>
    case 2:
      return <>Profile</>
    case 3:
      return <>Developer</>
    case 6:
      return <>Lab</>
    case 7:
      return <Changelog />
    case 8:
      return <>Settings</>
    default:
      return <>Not Found</>
  }
}

function BottomNavigation({ item, setItem }: { item: number, setItem: (_: number) => void }) {
  return <div className="dock dock-xl bg-neutral/30" style={{ position: "initial" }}>
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