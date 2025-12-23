import { getAppWrapped, getRepo } from "@/api/fetchApps";
import { openApplicationState } from "@/data/implementations/appView";
import { categoryView } from "@/data/implementations/catView";
import { useHome } from "@/lib/data";
import {
  Dialog,
  DialogTrigger,
  DialogSurface,
  DialogTitle,
  DialogBody,
  DialogContent,
  DialogActions,
  Button,
  Card,
  CardHeader,
  Body1,
  Caption1,
} from "@fluentui/react-components";
import { useEffect, useState } from "react";
import { AHQStoreApplication } from "src-ahqstore-types/pkg/ahqstore_types";

import { Check, BookMarked, SearchAlert } from "lucide-react"

export interface DeepLinkMeta {
  isOpen: boolean;
  task: {
    type: "app",
    id: string
  } | {
    type: "category",
    id: number;
  };
}

export const close: DeepLinkMeta = {
  isOpen: false,
  task: {
    type: "category",
    id: 0
  }
};

interface Props {
  meta: DeepLinkMeta;
  setMeta: (_: DeepLinkMeta) => void;
  set: (_: number) => void;
}

export const DeepLink = ({ meta: { isOpen, task }, setMeta, set }: Props) => {
  const home = useHome();

  const [app, setApp] = useState<[AHQStoreApplication, string] | "unverified" | undefined>();

  useEffect(() => {
    if (task.type == "app") {
      setApp(undefined);

      (async () => {
        try {
          setApp(await getAppWrapped(task.id));
        } catch (e) {
          console.log(e);
          setApp("unverified");
        }
      })()
    }
  }, [task]);

  return (
    <Dialog
      open={isOpen}
    >
      {/* <DialogTrigger disableButtonEnhancement>
        <></>
      </DialogTrigger> */}
      <DialogSurface >
        <DialogBody className="!bg-transparent !max-h-[80vh]">
          <DialogTitle>Deep Link Triggered</DialogTitle>
          <DialogContent>
            {
              task.type == "category" &&
              <>
                <p>
                  A deep link has requested AHQ Store to navigate to home screen group named <code className="border border-foreground-content border-dashed px-1 rounded-md">{home!![task.id][0]}</code>.
                </p>

                <p className="mt-3 text-warning">
                  <strong>Would you like AHQ Store to acknowledge this interaction by performing the navigation action?</strong>
                </p>
              </>
            }

            {
              task.type == "app" &&
              <>
                <p>
                  A deep link has requested AHQ Store to open application with id <code className="border border-foreground-content border-dashed px-1 rounded-md">{task.id}</code>.
                </p>

                {app === "unverified" &&
                  <p className="alert alert-soft alert-error flex flex-col shadow-lg mt-2">
                    The application is non-existent. If you are experiencing such errors, it may be that someone is trying to hijack your AHQ Store experience by sending garbage data periodically. You should have a look at Task Manager or similar softwares to track the process or browser website trying to spam such invalid URLs.
                  </p>
                }

                <div className="flex flex-col w-full text-center items-center jusitfy-center gap-2 my-2">
                  {app === undefined && <span className="loading loading-spinner loading-lg mx-auto"></span>}

                  {app !== undefined &&
                    <Card appearance="filled-alternative" className="w-full dark:bg-neutral-content/10!">
                      <CardHeader
                        image={
                          <>
                            {app == "unverified" ? <SearchAlert className="size-16 min-size-16 max-size-16" /> : <img src={app[1]} alt="App Logo" className="size-16 min-size-16 max-size-16" />}
                          </>
                        }
                        header={
                          <div className="flex flex-col">
                            <Body1 className="flex! flex-row! w-full">
                              <b>{app == "unverified" ? "Unknown Application" : app[0].appDisplayName}</b>

                              {app != "unverified" && (
                                app[0].verified ? <div className="badge badge-soft badge-success px-2 ml-2 gap-1 flex"> <Check className="size-4 min-size-4 max-size-4" /> Verified</div> :
                                  <div className="badge px-2 ml-2 gap-1 flex"> <BookMarked className="size-4 min-size-4 max-size-4" /> {getRepo(app[0])}</div>
                              )
                              }
                            </Body1>

                            <Caption1>{task.id}</Caption1>
                          </div>
                        }
                      />

                      <div className="flex w-full h-full text-start items-start">
                        <span className="line-clamp-5 text-clip">{app == "unverified" ? "This application does not exist in AHQ Store" : app[0].description}</span>
                      </div>
                    </Card>
                  }
                </div>

                <p className="mt-3 text-warning">
                  <strong>Would you like AHQ Store to acknowledge this interaction by performing the navigation action?</strong>
                </p>
              </>
            }
          </DialogContent>
          <DialogActions>
            <DialogTrigger>
              <Button appearance="subtle" onClick={() => setMeta(close)}>No</Button>
            </DialogTrigger>
            <DialogTrigger disableButtonEnhancement>
              <Button disabled={task.type == "app" && !Array.isArray(app)} appearance="primary" onClick={() => {
                if (task.type == "category") {
                  categoryView.data = task.id;
                  set(12);
                } else {
                  openApplicationState.data = task.id;
                  set(10);
                }

                setMeta(close);
              }}>Yes</Button>
            </DialogTrigger>
          </DialogActions>
        </DialogBody>
      </DialogSurface>
    </Dialog>
  );
};