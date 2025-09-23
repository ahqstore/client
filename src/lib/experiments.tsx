import { Bug, Cable, SearchIcon } from "lucide-react";
import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";

export interface Experiments {
  plugins?: boolean;
  forceVibrant?: boolean;
}

export const docs: { [key: string]: { docs: string, title: string, Icon: typeof SearchIcon } } = {
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
  const [val, setVal] = useState<Experiments>({});

  useEffect(() => {
    try {
      const item = localStorage.getItem("experiments")!;
      const d = JSON.parse(item);

      if (d != null) {
        setVal(d);
      }
    } catch (_) { }

    setExperiment = (d) => {
      setVal(d)
      localStorage.setItem("experiments", JSON.stringify(d));
    };
  }, []);

  return <Experiments.Provider value={val}>{children}</Experiments.Provider>;
}
