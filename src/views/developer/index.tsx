import { Category } from "@/components/category";
import { useAuth } from "@/lib/auth/provider";
import { UserCircle2, PackageCheckIcon } from "lucide-react";
import { useEffect, useState } from "react";

import { getDevData, getDevsApps, open } from "tauri-plugin-ahqstore-api"

import type { DevData } from "src-ahqstore-types/pkg/ahqstore_types";
export default function DeveloperPage() {
  const auth = useAuth();

  const [dev, setDD] = useState<DevData | null>(null);
  const [apps, setDA] = useState<string[] | null>(null);
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
          setDA(apps);
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
        <div className="flex space-y-1 space-x-4 text-md">
          {/* Fields */}
          <div className="flex flex-col space-y-2">
            <span>Developer Name</span>
            <span>Developer ID</span>
            <span>GitHub Username</span>
            <span>GitHub Profile</span>
            <span>Profile Picture</span>
          </div>

          {/* Values */}
          <div className="flex flex-col space-y-2 text-gray-700 dark:!text-gray-200/80">
            <span>{dev.name}</span>
            <span>{dev.id}</span>
            <span>{dev.github}</span>
            <span role="button" onClick={() => open(`https://github.com/${dev.github}`)} className="w-fit-content cursor-pointer underline">Open</span>
            <img role="button" aria-description="Open in browser" onClick={() => open(dev.avatar_url)} className="rounded-full my-auto w-2 cursor-pointer" src={dev?.avatar_url} />
          </div>
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
          {/* Fields */}
          <div className="flex flex-col space-y-2">
            <span>Developer Name</span>
            <span>Developer ID</span>
            <span>GitHub Username</span>
          </div>

          {/* Values */}
          <div className="flex flex-col space-y-2">
            <span>{dev.name}</span>
            <span>{dev.id}</span>
            <span>{dev.github}</span>
          </div>
        </div>
      </Category>
    </> : <div className="w-full h-full flex flex-col gap-4 text-center justify-center items-center">
      <span className="block loading loading-spinner w-[calc(var(--size-selector,0.25rem)*15)]"></span>
      <span className="md:text-3xl sm:text-xl text-lg">Just a moment...</span>
    </div>
  );
}
