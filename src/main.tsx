import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import { getCurrentWindow } from "@tauri-apps/api/window"
import { ThemeProvider } from "./lib";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { Secondary } from "./Secondary";
import { Toaster } from "./components/ui/sonner";
import { AuthProvider } from "./lib/auth/provider";

export const window = getCurrentWindow();
(async () => {
  try {
    window.emit("loaded", "");
  } catch (_) { }
})();

if (getCurrentWebviewWindow().label == "main") {
  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
      <AuthProvider>
        <ThemeProvider>
          <App />
          <Toaster />
        </ThemeProvider>
      </AuthProvider>
    </React.StrictMode>,
  );
} else {
  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
      <ThemeProvider>
        <Secondary />
      </ThemeProvider>
    </React.StrictMode>,
  );
}