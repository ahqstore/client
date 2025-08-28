import {
  Dialog,
  DialogTrigger,
  DialogSurface,
  DialogTitle,
  DialogBody,
  DialogContent,
  DialogActions,
  Button,
} from "@fluentui/react-components";

import { open } from "tauri-plugin-ahqstore-api"

export const Disclaimer = () => {
  return (
    <Dialog defaultOpen={true}>
      {/* <DialogTrigger disableButtonEnhancement>
        <></>
      </DialogTrigger> */}
      <DialogSurface >
        <DialogBody className="!bg-transparent !max-h-[80vh]">
          <DialogTitle>Please be informed</DialogTitle>
          <DialogContent>
            <p>
              AHQ Store distributes applications from the sources like
              <strong> Winget, FDroid, AppImageHub, AHQStore Repo </strong> this means that
              you should install the applications at your own discretion.
              Even though AHQ Store explicitly <strong>scans</strong> application with Windows Defender
              on Windows client, and <strong>uses ClamAV in its workflows during app submission. </strong>
              We absolutely provide no guarantee and request your own discretion.
            </p>
            <p className="mt-4">
              Learn more about the security measures used by us <span className="underline text-blue-600 cursor-pointer" onClick={() => open("https://ahqstore.github.io/security")}>here</span>
            </p>
            <p className="mt-4">
              AHQ Store does not collect any data. No data is collected by
              AHQ Store whatsoever. Logging into an account is absolutely not
              necessary or even required for the basic functionality with a few
              exceptions (App Reporting, Issue Reporting)
            </p>
          </DialogContent>
          <DialogActions>
            <DialogTrigger disableButtonEnhancement>
              <Button appearance="primary">Got it</Button>
            </DialogTrigger>
          </DialogActions>
        </DialogBody>
      </DialogSurface>
    </Dialog>
  );
};
