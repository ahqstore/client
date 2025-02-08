import { createContext, ReactNode, use, useEffect, useState } from "react";
import { teamsDarkTheme, teamsLightTheme, FluentProvider } from "@fluentui/react-components"
import { type } from "@tauri-apps/plugin-os";

const def = String(window.matchMedia("(prefers-color-scheme: dark)").matches);

export default function fnTheme() {
  const dark = (localStorage.getItem("dark") || def) == "true";

  document.querySelector("html")?.classList.toggle("dark", dark);
  document.querySelector("html")?.classList.toggle("not-win", type() != "windows");
}

const ThemeContext = createContext(false);
export let setUITheme = (_: boolean) => { };

export const useUITheme = () => {
  if (use(ThemeContext)) {
    return teamsDarkTheme;
  } else {
    return teamsLightTheme;
  }
}

export function ThemeProvider({ children }: { children: ReactNode }) {
  fnTheme();

  const [dark, setDark] = useState(true);
  const [theme, setTheme] = useState(teamsDarkTheme);

  setUITheme = (theme: boolean) => setDark(theme);

  useEffect(() => {
    localStorage.setItem("dark", String(dark));

    if (dark) {
      setTheme(teamsDarkTheme);
    } else {
      setTheme(teamsLightTheme);
    }

    fnTheme();
  }, [dark]);

  return <ThemeContext.Provider value={dark} >
    <FluentProvider theme={theme}>
      <div className="content">
        {children}
      </div>
    </FluentProvider>
  </ThemeContext.Provider>
}