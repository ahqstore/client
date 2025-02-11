import { useMediaQuery } from "@/hooks/use-media-query";

import {
  DrawerBody,
  DrawerHeader,
  DrawerHeaderTitle,
  Drawer,
  Button,
  useRestoreFocusSource,
} from "@fluentui/react-components";

import {
  Dismiss24Regular,

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

const items = [
  {
    name: "Apps",
    id: 0,
    icon: <AppsRegular className="size-[1.5em]" />,
    iconFilled: <AppsFilled className="size-[1.5em]" />,
    iconMobile: <LayoutGrid size="1.5em" />,
    iconMobileFilled: <LayoutGrid fill="currentcolor" size="1.5em" />,
  },
  {
    name: "Library",
    id: 1,
    icon: <AppsListRegular className="size-[1.5em]" />,
    iconFilled: <AppsListFilled className="size-[1.5em]" />,
    iconMobile: <Library size="1.5em" />,
    iconMobileFilled: <LibraryBig size="1.5em" />,
  },
  {
    name: "Profile",
    id: 2,
    icon: <PersonRegular className="size-[1.5em]" />,
    iconFilled: <PersonFilled className="size-[1.5em]" />,
    iconMobile: <User size="1.5em" />,
    iconMobileFilled: <User fill="currentcolor" size="1.5em" />,
  },
  {
    name: "Settings",
    id: 3,
    icon: <SettingsRegular className="size-[1.5em]" />,
    iconFilled: <SettingsFilled className="size-[1.5em]" />,
    iconMobile: <Settings size="1.5em" />,
    iconMobileFilled: <Settings className="rotate-12" size="1.5em" />,
  }
];

export function ApplicationView() {
  const [isOpen, setIsOpen] = useState(true);

  const [item, setItem] = useState(0);

  const tablet = useMediaQuery("(min-width: 640px)");
  const desktop = useMediaQuery("(min-width: 1024px)");

  const restoreFocusSourceAttributes = useRestoreFocusSource();

  if (desktop) {
    return <div className="mt-2 w-full h-full flex">
      <div className="h-full w-64 flex flex-col">
        <NavigationSidebar />
      </div>
      <div className="w-full h-full rounded-tl-xl p-3 bg-neutral/50">
        <h1>Hello World</h1>
      </div>
    </div>;
  }

  if (tablet) {
    return <>
      <div className="h-full w-full bg-black flex">
        <h1 onClick={() => setIsOpen(true)}>Drawer</h1>
      </div>
      <Drawer
        {...restoreFocusSourceAttributes}
        type={"overlay"}
        separator
        className="drawer-tablet"
        open={isOpen}
        onOpenChange={(_, { open }) => setIsOpen(open)}
      >
        <DrawerHeader>
          <DrawerHeaderTitle
            action={
              <Button
                appearance="subtle"
                aria-label="Close"
                icon={<Dismiss24Regular />}
                onClick={() => setIsOpen(false)}
              />
            }
          >

            Navigation
          </DrawerHeaderTitle>
        </DrawerHeader>

        <DrawerBody>
          <p>Drawer content</p>
        </DrawerBody>
      </Drawer>
    </>;
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