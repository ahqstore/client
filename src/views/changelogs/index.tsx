import { Category } from "@/components/category";
import { Megaphone, ScrollText } from "lucide-react";

import { open } from "tauri-plugin-ahqstore-api"

export default function Changelog() {
  return (
    <>
      <Category
        title="AHQ Store v0.0.1"
        description="Learn what's new in this version of AHQ Store"
        Icon={Megaphone}
        normallyOpen={true}
      >
        <div className="readd flex flex-col space-y-2">
          <div>
            <h1>UI Redesign</h1>
            <span>
              We're experimenting with the Sidebar using a design language
              similar to <strong>"WinUI 3"</strong>
            </span>
            <span>
              We're slowly starting to build the pages, starting with this one!
            </span>
            <span>
              1st class support for android!
            </span>
          </div>
        </div>
      </Category>

      <Category
        title="Application Site is out"
        description="Hurray! The applications site is now out!"
        Icon={ScrollText}
      >
        <div className="readd flex flex-col space-y-2">
          <button className="mr-auto w-16 cursor-pointer underline" onClick={() => open("https://ahqstore.github.io/applications")}>Click here</button>
        </div>
      </Category>
    </>
  );
}
