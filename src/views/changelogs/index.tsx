import { Category } from "@/components/category";
import { Megaphone, ScrollText } from "lucide-react";

export default function Changelog() {
  return (
    <>
      <Category
        title="This Update"
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
          </div>
        </div>
      </Category>

      <Category
        title="Mentionable Changes"
        description="Learn what's changed over the updates"
        Icon={ScrollText}
      >
        <div className="readd flex flex-col space-y-2">
          <div>
            <h1>2025.03.11</h1>
            <span>UI redesign</span>
          </div>
          <div>
            <h1>2025.01</h1>
            <span>Initial Build</span>
          </div>
        </div>
      </Category>
    </>
  );
}
