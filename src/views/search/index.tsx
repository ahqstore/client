import BackButton from "@/components/backButton";
import { searchQueryData } from "@/data/implementations/searchData";
import { useStore } from "@/data/store";
import { Search } from "lucide-react";

export default function SearchInterface({ set }: { set: (_: number) => void }) {
  const value = useStore(searchQueryData);

  return <div className="w-full h-full overflow-hidden flex flex-col">
    <div className="flex w-full justify-center text-center items-center">
      <BackButton set={set} backTo={0} />

      <div className="relative w-full max-w-xl">
        <form>
          <input
            type="text"
            placeholder="Search for apps, games, and more"
            defaultValue={value}
            className="w-full py-3 pl-9 pr-3 bg-primary/10 rounded-xl border border-border focus:outline-none focus:ring-1 focus:ring-muted transition duration-300"
          />
          <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground h-5 w-5" />
        </form>
      </div>

      <div className="w-11 ml-auto" />
    </div>
  </div>;
}