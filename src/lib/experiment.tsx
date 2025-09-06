import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";

const Experiment = createContext(false);

export const useExperiment = () => useContext(Experiment);

export function ExperimentProvider({ children }: { children: ReactNode }) {
  const [val, setVal] = useState(false);
  useEffect(() => {
    setVal(localStorage.getItem("always-show-exp") == "true");

    window.addEventListener("keydown", (e) => {
      if (e.ctrlKey && e.key == "E") {
        setVal((d) => !d);
      }
    });
  }, []);

  return <Experiment.Provider value={val}>{children}</Experiment.Provider>;
}
