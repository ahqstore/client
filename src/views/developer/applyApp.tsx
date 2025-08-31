import { useEffect, useState } from "react";
import ShowSpinner from "../spinner";
import { Checkbox, Button, Input, DialogActions, Label } from "@fluentui/react-components";
import { DatabaseRegular } from "@fluentui/react-icons";

import { open } from "tauri-plugin-ahqstore-api"
import { Separator } from "@/components/ui/separator";
import { getAHQStoreManifest, verifyRepo } from "@/lib/auth/verification";
import { applyForAppLaunch } from "@/lib/auth/apply";

export enum PageState {
  Loading,
  TOS,
  Details,
  Done
}

export interface Repo {
  author: string;
  repo: string;
}

export function ApplyForUpload() {
  const [step, setStep] = useState<PageState>(PageState.Loading);

  const [tos, setTos] = useState(false);
  const [coc, setCoc] = useState(false);

  const [repo, setRepo] = useState<Repo>();

  const [err, setErr] = useState("Please fill the GitHub Repository");
  const [verified, setVerified] = useState(false);
  const [manifest, setManifestUrl] = useState("");

  useEffect(() => {
    setTimeout(() => {
      setStep(PageState.TOS);
    }, 1000);
  }, []);

  return <div className="w-full flex flex-col my-3 space-y-5">
    <ul className="steps steps-vertical lg:steps-horizontal w-full">
      <li className={`step ${step >= PageState.TOS ? "step-primary" : ""}`}>Review & Agree</li>
      <li className={`step ${step >= PageState.Details ? "step-primary" : ""}`}>Provide Details</li>
      <li className={`step ${step >= PageState.Done ? "step-primary" : ""}`}>Submitted</li>
    </ul>

    {step == PageState.Loading && <ShowSpinner />}
    {
      step == PageState.TOS &&
      <div className="flex flex-col space-y-3">
        <p>
          You're about to submit an application to the <strong>AHQ Store Community
            Repository</strong>. We would like to remind you to check out our <strong><span className="text-blue-700 dark:text-blue-400 cursor-pointer" onClick={() => open("https://ahqstore.github.io/tos")}>Terms
              of Service</span> and <span className="text-blue-700 dark:text-blue-400 cursor-pointer" onClick={() => open("https://ahqstore.github.io/privacy")}>Privacy Policy</span></strong> if you have not yet. Continuing means
          that you are agreeing to both of them and hence you are being guided
          by them.
        </p>
        <p>
          Since submitting an application also means <strong>contributing</strong> to our
          repository, you must also agree to our <span className="text-blue-700 dark:text-blue-400 cursor-pointer" onClick={() => open("https://github.com/ahqstore/client?tab=coc-ov-file#readme")}>code of conduct</span>.
        </p>
        <p>
          You must also be aware that all the data is <strong>public</strong> and can be viewed by anyone.
          This means that your account metadata, your application metadata, your icons will
          be stored in our community repository and will remain public until requested for
          deletion
        </p>
        <p>
          <strong>Transparency</strong> is the key to the best interaction. These notices ensure
          a smooth and transparent conversation to ensure a long-lasting and thriving community.
          We are not frightening you. Instead, we're empowering you with how we function
        </p>
        <div className="w-full flex flex-col">
          <Checkbox
            checked={tos}
            onChange={() => setTos((t) => !t)}
            label="Yes, I agree to the Disclaimer, Terms and Service and Privacy Policy"
          />
          <Checkbox
            checked={coc}
            onChange={() => setCoc((t) => !t)}
            label="I agree to the Code of Conduct"
          />
        </div>
        <DialogActions className="w-full flex justify-end">
          <Button
            disabled={!(tos && coc)}
            appearance="primary"
            onClick={() =>
              setStep(PageState.Details)
            }
          >
            Continue
          </Button>
        </DialogActions>
      </div>
    }

    {
      step == PageState.Details &&

      <>
        <div className="w-full flex flex-col space-y-3">
          <Separator className="text-base-content" />

          <p>
            AHQ Store automatically fetches your <strong>ahqstore.json</strong> file from
            your provided repository. You are supposed to set up
            <strong> GitHub Releases</strong> and we'll automatically fetch
            the data and create the app submission request
          </p>

          <p>
            If you have not yet set up your <strong>open sourced</strong> repository for <strong>AHQ Store Integration.</strong>
            <span className="text-blue-700 dark:text-blue-400 cursor-pointer" onClick={() => open("https://ahqstore.github.io/guide/integrate")}> Click here to set up</span>
          </p>

          <div className="divider"></div>


          <Label>Enter your repository (owner/repo) format</Label>
          <Input
            contentBefore={<DatabaseRegular />}
            className="w-full"
            disabled={err == "Verifying..." || verified}
            onChange={(ev) => {
              const input = ev.target.value;

              const splits = input.split("/");

              if (splits.length == 2) {
                const [author, repo] = splits;

                setErr("");
                setRepo({
                  author,
                  repo
                });
              } else {
                setErr("Malformed Repository");
              }
            }}
          />

          {err != "" && <div role="alert" className="alert alert-warning alert-dash flex">
            {err == "Verifying..." && <ShowSpinner />}
            <span>{err}</span>
          </div>}
        </div>

        <DialogActions className="w-full flex justify-end">
          {repo && !verified && <Button
            disabled={err == "Verifying..."}
            appearance="subtle"
            onClick={() => {
              setErr("Verifying...");

              (async () => {
                const err = await verifyRepo(repo);

                if (err == "") {
                  const { error, output } = await getAHQStoreManifest(repo);

                  if (error) {
                    setErr(output);
                  } else {
                    setErr("");
                    setVerified(true);
                    setManifestUrl(output);
                  }
                } else {
                  setErr(err);
                }
              })()
            }}
          >
            Verify
          </Button>}
          <Button
            disabled={!verified}
            appearance="subtle"
            onClick={() => {
              applyForAppLaunch(manifest);
              setStep(PageState.Done);
            }}
          >
            Apply
          </Button>
        </DialogActions>
      </>
    }

    {step == PageState.Done && <>
      <p>
        Your request has been successfully submitted. A browser window will open shortly with
        a GitHub issue. Please wait for any output from there patiently, or you can check for
        the commits and see if your app metadata has been committed to the repository.
      </p>
      <p>
        Once committed, your app will slowly become available to users. You can restart your
        computer to see the application in your <strong>AHQ Store</strong> application.
      </p>
      <p>
        Have a fabulous day ahead 👋!
      </p>
      <div role="alert" className="alert alert-success alert-dash flex">
        <span>Tired of manually submitting apps and app updates? Learn to automate <span className="text-blue-700 dark:text-blue-400 cursor-pointer underline" onClick={() => open("https://ahqstore.github.io/guide/automate")}>by clicking here</span></span>
      </div>
    </>}
  </div>;
}