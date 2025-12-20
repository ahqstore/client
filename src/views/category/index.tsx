import { useHome } from "@/lib/data";
import { categoryView } from "@/data/implementations/catView";

import { AppBox } from "../apps/appBox";
import { useStore } from "@/data/store";
import BackButton from "@/components/backButton";

export function CategoryView({ set }: { set: (_: number) => void }) {
  const home = useHome();

  const index = useStore(categoryView);

  return (
    <>
      <div className="w-full h-full overflow-hidden flex flex-col">
        {[home!![index!!]]?.map((apps) => (
          <div
            className="home_apps home_apps_override overflow-x-hidden overflow-y-scroll"
            key={apps[0]}
          >
            <div>
              <BackButton set={set} backTo={0} noRightRound />

              <span className="my-auto ml-2 mr-auto">{apps[0]}</span>
            </div>
            <div>
              <span>Explore all of the {apps[1].length} apps</span>
            </div>
            <div>
              {apps[1].slice(0, 4).map((app) => (
                <AppBox key={`${app}${apps[0]}`} appId={app} set={set} />
              ))}
            </div>
          </div>
        ))}
      </div>
    </>
  );
}
