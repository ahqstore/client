import BackButton from "@/components/backButton";
import { searchQueryData } from "@/data/implementations/searchData";
import { useStore } from "@/data/store";
import { useMediaQuery } from "@/hooks/use-media-query";
import { Search } from "lucide-react";
import { useEffect, useRef } from "react";
import { search } from "src-plugin/dist-js";

export default function SearchInterface({ set }: { set: (_: number) => void }) {
  const value = useStore(searchQueryData);

  const isTablet = useMediaQuery("(min-width: 50rem)");

  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    (async () => {
      if (value) {
        console.log("Searching");
        console.log(await search(value));
      }
    })()
  }, [value]);

  return <div className="w-full h-full overflow-hidden flex flex-col">
    <div className="flex w-full justify-center text-center items-center">
      <BackButton set={set} backTo={0} noRightRound />

      <div className="relative w-full max-w-xl">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            const val = inputRef.current!!.value;

            searchQueryData.data = val;
          }}
        >
          <input
            type="text"
            placeholder="Search for apps, games, and more"
            defaultValue={value}
            ref={inputRef}
            className="w-full py-3 pl-9 pr-3 bg-primary/10 rounded-xl rounded-l-none md:rounded-l-xl border border-border focus:outline-none focus:ring-1 focus:ring-muted transition duration-300"
          />
          <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground h-5 w-5" />
        </form>
      </div>

      <div className="hidden md:block w-11 ml-auto" />
    </div>

    <div className="mt-5 w-full h-full flex flex-col">
      <h1 className="text-xl font-sans mb-5">"{value?.substring(0, isTablet ? 30 : 10).trim()}{(value?.length || 0) > (isTablet ? 30 : 10) ? "..." : ""}"</h1>

      <div className="w-full h-full">

      </div>
    </div>
  </div>;
}