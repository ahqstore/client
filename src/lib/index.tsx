import { createContext, ReactNode, useContext, useEffect, useState } from "react";
import { teamsDarkTheme, teamsLightTheme, FluentProvider } from "@fluentui/react-components"
import { isWindows11 } from "src-plugin/dist-js";

const def = String(window.matchMedia("(prefers-color-scheme: dark)").matches);

export default function fnTheme(windows: boolean) {
  const dark = (localStorage.getItem("dark") || def) == "true";

  document.querySelector("html")?.classList.toggle("dark", dark);

  if (windows) {
    document.querySelector("html")?.classList.remove("not-win");
  } else {
    document.querySelector("html")?.classList.add("not-win");
  }
}

const ThemeContext = createContext(false);
export let setUITheme = (_: boolean) => { };

export const useUITheme = () => {
  if (useContext(ThemeContext)) {
    return teamsDarkTheme;
  } else {
    return teamsLightTheme;
  }
}

export function ThemeProvider({ children }: { children: ReactNode }) {
  const [dark, setDark] = useState(true);
  const [theme, setTheme] = useState(teamsDarkTheme);
  const [windows, setWindows] = useState(false);

  setUITheme = (theme: boolean) => setDark(theme);

  useEffect(() => {
    document.querySelector("html")!!.style.setProperty("--win32-accent", window.accent);

    isWindows11().then(setWindows).catch(console.error);
  }, []);

  useEffect(() => {
    if (windows) {
      setDark(def == "true");
    }
  }, [windows]);

  useEffect(() => {
    localStorage.setItem("dark", String(dark));

    if (dark) {
      setTheme(teamsDarkTheme);
    } else {
      setTheme(teamsLightTheme);
    }

    fnTheme(windows);
  }, [dark, windows]);

  return <ThemeContext.Provider value={dark} >
    <FluentProvider theme={theme}>
      <div className="content">
        {children}
      </div>
    </FluentProvider>
  </ThemeContext.Provider>
}