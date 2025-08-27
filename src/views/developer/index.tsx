import { Category } from "@/components/category";
import { useAuth } from "@/lib/auth/provider";
import { UserCircle2, PackageCheckIcon } from "lucide-react";
import { useEffect, useState } from "react";

import { getDevData, getDevsApps, getApp, getAppAsset } from "tauri-plugin-ahqstore-api"

import type { AHQStoreApplication, DevData } from "src-ahqstore-types/pkg/ahqstore_types";

export default function DeveloperPage() {
  const auth = useAuth();

  const [dev, setDD] = useState<DevData | null>(null);
  const [apps, setDA] = useState<{
    app: AHQStoreApplication;
    icon: Uint8Array<ArrayBufferLike>;
  }[] | null>(null);
  const [ready, setReady] = useState(false);

  useEffect(() => {
    (async () => {
      if (auth) {
        const devid = auth.login == "ahqsoftwares" ? "1" : auth.login;

        // Correction for User Name
        const devdata = await getDevData(`a:${devid}`);

        setDD(devdata);

        const apps = await getDevsApps(`a:${devid}`);

        if (apps[0] != "404: Not Found") {
          const allApps = await Promise.all(apps.map(
            (appId) => {
              return (async () => {
                const app = await getApp(appId);
                const icon = await getAppAsset(appId, "0");

                return {
                  app, icon
                }
              })();
            }
          ));
          setDA(allApps);
        } else {
          setDA([]);
        }

        setReady(true);
      }
    })()
  }, [auth]);

  return (
    (ready && dev && apps && auth) ? <>
      <div className="flex gap-1">
        <h1 className="text-xl">Welcome, <strong className="text-base-300 dark:text-base-content">{dev?.name}</strong></h1>
        <img className="rounded-full my-auto w-[1.25rem] h-[1.25rem]" src={dev?.avatar_url} />
      </div>

      <Category
        title="Profile Details"
        description="Core profile details"
        Icon={UserCircle2}
        normallyOpen
      >
        <div className="flex flex-col space-y-2">

        </div>
      </Category>

      <Category
        title={`Published Applications (${apps.length})`}
        description="View your published applications"
        Icon={PackageCheckIcon}
        openable={apps.length != 0}
        normallyOpen={apps.length != 0}
      >
        <div className="flex flex-col space-y-2">

        </div>
      </Category>
    </> : <div className="w-full h-full flex flex-col gap-4 text-center justify-center items-center">
      <span className="block loading loading-spinner w-[calc(var(--size-selector,0.25rem)*15)]"></span>
      <span className="md:text-3xl sm:text-xl text-lg">Just a moment...</span>
    </div>
  );
}
