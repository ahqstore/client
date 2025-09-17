import { useMediaQuery } from "@/hooks/use-media-query";

import {
  AppsRegular,
  AppsFilled,
  SettingsRegular,
  SettingsFilled,
  // @ts-ignore
  PersonRegular,
  // @ts-ignore
  PersonFilled,
  LibraryRegular,
  LibraryFilled,
  WindowDevToolsFilled,
  WindowDevToolsRegular,
  MegaphoneLoudRegular,
  MegaphoneLoudFilled,
  ToolboxRegular,
  ToolboxFilled,
} from "@fluentui/react-icons";

import { platform } from "@tauri-apps/plugin-os";
import {
  Library,
  LayoutGrid,
  Settings,
  // @ts-ignore
  User,
  LibraryBig,
  Code2Icon,
} from "lucide-react";

import { useMemo, useState } from "react";
import NavigationSidebar from "./nav";
import { useExperiment } from "@/lib/experiment";
import { useAuth } from "@/lib/auth/provider";

import { AppsHome } from "./apps";

import Changelog from "./changelogs";
import SettingsPage from "./settings";
import LibraryPage from "./library";
import DeveloperPage from "./developer";
import LabPage from "./lab";
import { Disclaimer } from "./disclaimer";
import Application from "./app";
import SearchInterface from "./search";

export const items: {
  name: string;
  id: number;
  icon: JSX.Element;
  iconFilled: JSX.Element;
  iconMobile: JSX.Element;
  iconMobileFilled: JSX.Element;
  active?: number[];
  hidden?: () => boolean;
}[] = [
    {
      name: "Apps",
      id: 0,
      icon: <AppsRegular className="size-[1.5em]" />,
      iconFilled: (
        <AppsFilled
          className="size-[1.5em]"
          style={{ color: "var(--win32-accent)" }}
        />
      ),
      active: [9, 10, 11, 12],
      iconMobile: <LayoutGrid size="1.5em" />,
      iconMobileFilled: <LayoutGrid fill="currentcolor" size="1.5em" />,
    },
    {
      name: "Library",
      id: 1,
      icon: <LibraryRegular className="size-[1.5em]" />,
      iconFilled: (
        <LibraryFilled
          className="size-[1.5em]"
          style={{ color: "var(--win32-accent)" }}
        />
      ),
      iconMobile: <Library size="1.5em" />,
      iconMobileFilled: <LibraryBig size="1.5em" />,
    },
    // {
    //   name: "Profile",
    //   id: 2,
    //   hidden: () => platform() != "android",
    //   icon: <PersonRegular className="size-[1.5em]" />,
    //   iconFilled: (
    //     <PersonFilled
    //       className="size-[1.5em]"
    //       style={{ color: "var(--win32-accent)" }}
    //     />
    //   ),
    //   iconMobile: <User size="1.5em" />,
    //   iconMobileFilled: <User fill="currentcolor" size="1.5em" />,
    // },
    {
      name: "Developer",
      id: 3,
      hidden: () => !(useAuth()?.dev || false),
      icon: <ToolboxRegular className="size-[1.5em]" />,
      iconFilled: (
        <ToolboxFilled
          className="size-[1.5em]"
          style={{ color: "var(--win32-accent)" }}
        />
      ),
      iconMobile: <ToolboxRegular className="size-[1.5em]" />,
      iconMobileFilled: <ToolboxFilled className="size-[1.5em]" />,
    },
    {
      name: "Lab",
      id: 6,
      hidden: () => !useExperiment(),
      icon: <WindowDevToolsRegular className="size-[1.5em]" />,
      iconFilled: (
        <WindowDevToolsFilled
          className="size-[1.5em]"
          style={{ color: "var(--win32-accent)" }}
        />
      ),
      iconMobile: <Code2Icon className="size-[1.5em]" />,
      iconMobileFilled: <Code2Icon className="size-[1.5em]" />,
    },
    {
      name: "Updates",
      id: 7,
      hidden: () => !useMediaQuery("(min-width: 640px)"),
      icon: <MegaphoneLoudRegular className="size-[1.5em]" />,
      iconFilled: (
        <MegaphoneLoudFilled
          className="size-[1.5em]"
          style={{ color: "var(--win32-accent)" }}
        />
      ),
      iconMobile: <Settings size="1.5em" />,
      iconMobileFilled: <Settings className="rotate-12" size="1.5em" />,
    },
    {
      name: "Settings",
      id: 8,
      icon: <SettingsRegular className="size-[1.5em]" />,
      iconFilled: (
        <SettingsFilled
          className="size-[1.5em]"
          style={{ color: "var(--win32-accent)" }}
        />
      ),
      iconMobile: <Settings size="1.5em" />,
      iconMobileFilled: <Settings className="rotate-12" size="1.5em" />,
    },
  ];

export function ApplicationView() {
  const [item, setItem] = useState(0);

  const desktop = useMediaQuery("(min-width: 640px)");

  const ui = useMemo(() => <GetJsx item={item} setItem={setItem} />, [item, setItem]);

  if (desktop) {
    return (
      <div className="mt-2 w-full h-full flex overflow-hidden">
        <div className="animate h-full w-24 flex flex-col gap-2 px-1 pb-2 items-center text-center overflow-hidden overflow-y-scroll">
          <NavigationSidebar item={item} setItem={setItem} />
        </div>
        <Disclaimer />
        <div className="w-full h-full flex flex-col space-y-2 rounded-tl-xl p-3 bg-muted/30 border border-muted dark:border-none border-b-0 border-r-0 overflow-y-scroll">
          {ui}
        </div>
      </div>
    );
  }

  return (
    <div className="w-full h-full flex flex-col overflow-hidden">
      <Disclaimer />
      <div className="h-full w-full flex flex-col space-y-2 p-2 overflow-scroll">
        {ui}
      </div>
      <BottomNavigation item={item} setItem={setItem} />
    </div>
  );
}

interface Props {
  item: number;
  setItem: React.Dispatch<React.SetStateAction<number>>;
}

function GetJsx({ item, setItem }: Props) {
  switch (item) {
    case 0:
      return <AppsHome set={(s) => setItem(s)} />;
    case 1:
      return <LibraryPage />;
    case 2:
      return <>Profile</>;
    case 3:
      return <DeveloperPage />;
    case 6:
      return <LabPage />;
    case 7:
      return <Changelog />;
    case 8:
      return <SettingsPage />;
    case 9:
      // Search
      return <SearchInterface set={setItem} />;
    case 10:
      // AppList
      return <Application />;
    case 11:
      // DevInfo
      return <SettingsPage />;
    case 12:
      // CategoryView
      return <Application />
    default:
      return <>Not Found</>;
  }
}

function BottomNavigation({
  item,
  setItem,
}: {
  item: number;
  setItem: (_: number) => void;
}) {
  return (
    <div className="dock dock-xl bg-neutral/30" style={{ position: "initial" }}>
      {items
        .filter((s) => !(s.hidden && s.hidden()))
        .map((s) => {
          const isActive = s.id == item || (s.active && s.active.includes(item));

          return <button
            key={`${s.id}`}
            className={
              isActive ? "dock-active transition-all" : "transition-all"
            }
            onClick={() => setItem(s.id)}
          >
            {platform() == "android"
              ? isActive
                ? s.iconMobileFilled
                : s.iconMobile
              : isActive
                ? s.iconFilled
                : s.icon}
            <span className="dock-label">{s.name}</span>
          </button>
        })
      }
    </div >
  );
}
