import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";
import {
  teamsDarkTheme,
  teamsLightTheme,
  FluentProvider,
} from "@fluentui/react-components";
import { isWindows11 } from "src-plugin/dist-js";
import { useExperiments } from "./experiments";

const def = String(window.matchMedia("(prefers-color-scheme: dark)").matches);

export const VibrantWindows = createContext(false);

export default function fnTheme(windows: boolean, micaApplied: (_: boolean) => void, alwaysVibrant?: boolean) {
  const dark = (localStorage.getItem("dark") || def) == "true";

  document.querySelector("html")?.classList.toggle("dark", dark);

  if (alwaysVibrant || (windows && ((def == "true") == dark))) {
    document.querySelector("html")?.classList.remove("not-win");
    micaApplied(true);
  } else {
    document.querySelector("html")?.classList.add("not-win");
    micaApplied(false);
  }
}

export const ThemeContext = createContext(false);
export let setUITheme = (_: boolean) => { };

export const useUITheme = () => {
  if (useContext(ThemeContext)) {
    return teamsDarkTheme;
  } else {
    return teamsLightTheme;
  }
};

export function ThemeProvider({ children }: { children: ReactNode }) {
  const [dark, setDark] = useState(true);
  const [theme, setTheme] = useState(teamsDarkTheme);
  const [windows, setWindows] = useState(false);
  const [win32, setMICAApplied] = useState(false);

  const exp = useExperiments();

  setUITheme = (theme: boolean) => setDark(theme);

  useEffect(() => {
    document
      .querySelector("html")!!
      .style.setProperty("--win32-accent", window.accent);
  }, []);

  useEffect(() => {
    isWindows11()
      .then((d) => setWindows(d && dark == (def == "true")))
      .catch(console.error);
  }, [dark]);

  useEffect(() => {
    const dark = localStorage.getItem("dark");

    if (typeof dark == "string") setDark(dark == "true");
  }, []);

  useEffect(() => {
    localStorage.setItem("dark", dark ? "true" : "false");

    if (dark) {
      setTheme(teamsDarkTheme);
    } else {
      setTheme(teamsLightTheme);
    }

    fnTheme(windows, (value) => setMICAApplied(value), exp.forceVibrant);
  }, [exp, dark, windows]);

  return (
    <ThemeContext.Provider value={dark}>
      <VibrantWindows.Provider value={win32}>
        <FluentProvider theme={theme}>
          <div className="content">{children}</div>
        </FluentProvider>
      </VibrantWindows.Provider>
    </ThemeContext.Provider>
  );
}
