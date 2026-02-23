import { Category } from "@/components/category";
import { ConfigSelect } from "@/components/select";
import { Button } from "@/components/ui/button";

import { Repeat2, CheckCircle2, TriangleAlert, Package, PackageOpen } from "lucide-react"

import { useState } from "react";

export default function LibraryPage() {
  const [state, setState] = useState("done");

  return <>
    <div className="flex flex-col gap-2 w-full justify-center items-center">
      <ConfigSelect
        title="Updates"
        description={
          state == "done" ? "You're up to date" :
            state == "pending" ? "Pending updates are available" : "Checking for updates, please wait..."
        }
        Icon={state == "pending" ? TriangleAlert : state == "done" ? CheckCircle2 : Repeat2}
      >
        <Button
          className="cursor-pointer"
          onClick={() => {
            setState("");
          }}
          disabled={state == ""}
        >
          Check
        </Button>
      </ConfigSelect>

      <Category
        title="Pending Updates (0)"
        description="Lists the Updates Pending"
        Icon={PackageOpen}
      >
        <div className="w-full h-full items-center text-center justify-center flex flex-col space-y-3">
          <span className="block loading loading-spinner w-[calc(var(--size-selector,0.25rem)*10)]"></span>
          <h2 className="text-lg">Hold tight!</h2>
        </div>
      </Category>

      <Category
        title="Installed Applications (0)"
        description="Shows the applications installed and managed by AHQ Store"
        Icon={Package}
        normallyOpen
      >
        <div className="w-full h-full items-center text-center justify-center flex flex-col space-y-3">
          <span className="block loading loading-spinner w-[calc(var(--size-selector,0.25rem)*10)]"></span>
          <h2 className="text-lg">Loading app list...</h2>
        </div>
      </Category>
    </div>
  </>;
}