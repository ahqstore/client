import { Bug, Cable, ToolboxIcon, SearchIcon } from "lucide-react";
import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useMemo,
} from "react";

export interface Experiments {
  toolbox?: boolean;
  plugins?: boolean;
  forceVibrant?: boolean;
}

export const docs: { [key: string]: { docs: string, title: string, Icon: typeof SearchIcon } } = {
  toolbox: {
    docs: "Enables developer toolbox (FPS + heatmap) in AHQ Store",
    title: "Enable Toolbox",
    Icon: ToolboxIcon
  },
  plugins: {
    docs: "Enables plugins for Desktop Build for AHQ Store",
    title: "Enable Plugins",
    Icon: Cable
  },
  forceVibrant: {
    docs: "Forces Vibrant UI in any OS (danger: May lead to very broken UI)",
    title: "Force Vibrant UI",
    Icon: Bug
  }
}

const Experiments = createContext<Experiments>({});

export const useExperiments = () => useContext(Experiments);

export let setExperiment: (n: Experiments) => void;

export function ExperimentsProvider({ children }: { children: ReactNode }) {
  // const [val, setVal] = useState<Experiments>({});
  const val = useMemo(() => {
    try {
      const item = localStorage.getItem("experiments")!;
      const d = JSON.parse(item);

      if (d != null) {
        return d;
      }
    } catch (_) { }

    return {}
  }, []);

  useEffect(() => {
    setExperiment = (d) => {
      localStorage.setItem("experiments", JSON.stringify(d));
    };
  }, []);

  return <Experiments.Provider value={val}>{children}</Experiments.Provider>;
}
