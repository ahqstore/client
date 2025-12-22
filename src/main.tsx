import { scan } from "react-scan/all-environments";

try {
  const item = localStorage.getItem("experiments")!;
  const d = JSON.parse(item);

  if (d.toolbox) {
    scan({
      enabled: true
    });
  }
} catch (_) { }


import ReactDOM from "react-dom/client";
import App from "./App";

import { getCurrentWindow } from "@tauri-apps/api/window";
import { ThemeProvider } from "./lib";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { Secondary } from "./Secondary";
import { Toaster } from "./components/ui/sonner";
import { AuthProvider } from "./lib/auth/provider";
import { ExperimentProvider } from "./lib/experiment";
import { HomeProvider } from "./lib/data";
import { ExperimentsProvider } from "./lib/experiments";

declare global {
  interface Window {
    accent: string;
  }
}

export const window = getCurrentWindow();
(async () => {
  try {
    window.emit("loaded", "");
  } catch (_) { }
})();

if (getCurrentWebviewWindow().label == "main") {
  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <ExperimentProvider>
      <ExperimentsProvider>
        <HomeProvider>
          <AuthProvider>
            <ThemeProvider>
              <App />
              <Toaster />
            </ThemeProvider>
          </AuthProvider>
        </HomeProvider>
      </ExperimentsProvider>
    </ExperimentProvider>,
  );
} else {
  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <ThemeProvider>
      <Secondary />
    </ThemeProvider>,
  );
}
