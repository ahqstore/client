import { Dialog, DialogSurface, DialogTitle, MenuDivider, MenuItem, MenuList } from "@fluentui/react-components";
import "./App.css";

import { WindowTitlebar } from "./controls";
import ShowSpinner from "./views/spinner";

import { platform } from "@tauri-apps/plugin-os";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "./components/ui/popover";

import { LogOutIcon } from "lucide-react";
import {
  PersonAddRegular,
  PersonRegular,
  ToolboxRegular,
  BugArrowCounterclockwiseRegular,
} from "@fluentui/react-icons";

import { open, setScale } from "tauri-plugin-ahqstore-api";
import { startLogin } from "./lib/auth/github";
import { authObject, useAuth } from "./lib/auth/provider";
import { logOut } from "./lib/auth";
import { useExperiment } from "./lib/experiment";
import { useHome } from "./lib/data";
import { ApplicationView } from "./views/view";
import { useMediaQuery } from "./hooks/use-media-query";
import { useEffect, useState } from "react";
import { ApplyAsDeveloper } from "./views/developer/applySelf";
import { initPluginDaemonService } from "./api/plugin";
import { useExperiments } from "./lib/experiments";

function App() {
  const auth = useAuth();
  const experiment = useExperiment();
  const experiments = useExperiments();

  useEffect(() => {
    if (experiments.plugins) {
      initPluginDaemonService();
    }
  }, []);

  const [isOpen, setOpen] = useState(false);

  useEffect(() => {
    const zoom = localStorage.getItem("defZoom");

    if (zoom) setScale(Number(zoom));
  }, []);

  return (
    <>
      {platform() != "android" && (
        <WindowTitlebar
          data-tauri-drag-region
          right={
            <>
              <Popover>
                <PopoverTrigger className="border-none mx-3 cursor-pointer font-mono flex text-center items-center justify-center transition-all hover:scale-125 active:scale-90">
                  {auth?.avatar_url ? (
                    <img
                      src={auth?.avatar_url}
                      className="mr-1 my-1 size-6 avatar rounded-xl"
                    />
                  ) : (
                    <PersonRegular className="mr-1 my-1 p-1 size-6 avatar rounded-full border border-neutral-content" />
                  )}
                </PopoverTrigger>

                <PopoverContent
                  className={`!bg-accent shadow-sm w-auto h-auto p-2 backdrop-blur-3xl stylish rounded-lg border border-neutral-content/20`}
                >
                  <MenuList className="bg-transparent">
                    <MenuItem
                      style={{ background: "transparent" }}
                      onClick={() => {
                        if (auth) {
                          open(`https://github.com/${auth.login}`);
                        }
                      }}
                    >
                      <div className="flex">
                        {auth?.avatar_url ? (
                          <img
                            src={auth?.avatar_url}
                            className="h-12 avatar rounded-xl"
                          />
                        ) : (
                          <PersonRegular className="size-12 p-2 border border-neutral-content rounded-full" />
                        )}

                        <div className="ml-3 flex flex-col">
                          {auth ? (
                            <span>
                              Hello, <strong>{auth.name || auth.login}</strong>
                            </span>
                          ) : (
                            <span>
                              Hello, <strong>Guest</strong>
                            </span>
                          )}
                          {auth && <span>@{auth.login}</span>}
                        </div>
                      </div>
                    </MenuItem>

                    <MenuDivider />
                    <MenuItem
                      icon={<BugArrowCounterclockwiseRegular />}
                      style={{ background: "transparent" }}
                      subText={experiment ? "Enabled" : "Disabled"}
                    >
                      Experimental Features
                    </MenuItem>
                    {auth && (
                      <MenuItem
                        icon={<ToolboxRegular />}
                        subText={
                          auth.dev ? "Enabled" : "Register for developer mode"
                        }
                        style={{ background: "transparent" }}
                        onClick={() => {
                          if (!auth.dev) {
                            setOpen(true);
                          }
                        }}
                      >
                        Developer Mode
                      </MenuItem>
                    )}
                    <MenuItem
                      icon={
                        auth == undefined ? (
                          <PersonAddRegular />
                        ) : (
                          <LogOutIcon />
                        )
                      }
                      subText={
                        auth == undefined
                          ? "Login using GitHub authentication"
                          : "Logout from this device"
                      }
                      onClick={
                        auth == undefined
                          ? () => startLogin(authObject)
                          : () => logOut(authObject)
                      }
                      className="w-72"
                      style={{ background: "transparent" }}
                    >
                      {auth == undefined ? "Login" : "Logout"}
                    </MenuItem>
                  </MenuList>
                </PopoverContent>
              </Popover>
            </>
          }
        >
          <img
            data-tauri-drag-region
            src="/icon.png"
            className="ml-1 my-1 w-6 h-6"
          />
          <h1 data-tauri-drag-region className="my-auto ml-2 text-md font-sans">
            AHQ Store
          </h1>
          {useMediaQuery("(min-width: 400px)") && (
            <h1
              data-tauri-drag-region
              className="my-auto ml-1 text-black dark:!text-base-content text-md font-sans italic font-bold"
            >
              NEO
            </h1>
          )}
        </WindowTitlebar>
      )}

      <Dialog
        open={isOpen}
        modalType="non-modal"
        inertTrapFocus
        onOpenChange={() =>
          setOpen(false)
        }
      >
        <DialogSurface>
          <DialogTitle>Apply as a developer</DialogTitle>
          <ApplyAsDeveloper />
        </DialogSurface>
      </Dialog>
      {useHome() == undefined ? <Loading /> : <ApplicationView />}
    </>
  );

  function Loading() {
    return (
      <div className="content justify-center text-center items-center">
        <img src="/icon.png" className="w-[128px] h-[128px] my-[20vh]" />
        <ShowSpinner />
      </div>
    );
  }
}

export default App;
