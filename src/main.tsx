import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import { getCurrentWindow } from "@tauri-apps/api/window"
import { ThemeProvider } from "./lib";

export const window = getCurrentWindow();
(async () => {
  try {
    window.emit("loaded", "");
  } catch (_) { }
})();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider>
      <App />
    </ThemeProvider>
  </React.StrictMode>,
);
