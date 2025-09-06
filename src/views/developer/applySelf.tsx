import { useEffect, useState } from "react";
import ShowSpinner from "../spinner";
import { Checkbox, Button, Input, DialogActions, Label } from "@fluentui/react-components";
import { WindowDevEditFilled } from "@fluentui/react-icons";

import { open } from "tauri-plugin-ahqstore-api"
import { Separator } from "@/components/ui/separator";

import { applyAsDeveloper } from "@/lib/auth/apply";

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

export function ApplyAsDeveloper({ edit = false }: { edit?: boolean }) {
  const [step, setStep] = useState<PageState>(PageState.Loading);

  const [tos, setTos] = useState(false);
  const [coc, setCoc] = useState(false);

  const [name, setName] = useState("");
  const [err, setErr] = useState("Please fill your new account name");

  useEffect(() => {
    setTimeout(() => {
      if (!edit) {
        setStep(PageState.TOS);
      } else {
        setStep(PageState.Details);
      }
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
          You're about to apply for developer for the <strong>AHQ Store Community
            Repository</strong>. We would like to remind you to check out our <strong><span className="text-blue-700 dark:text-blue-400 cursor-pointer" onClick={() => open("https://ahqstore.github.io/tos")}>Terms
              of Service</span> and <span className="text-blue-700 dark:text-blue-400 cursor-pointer" onClick={() => open("https://ahqstore.github.io/privacy")}>Privacy Policy</span></strong> if you have not yet. Continuing means
          that you are agreeing to both of them and hence you are being guided
          by them.
        </p>
        <p>
          Since applying for developer also means <strong>contributing</strong> to our
          repository, you must also agree to our <span className="text-blue-700 dark:text-blue-400 cursor-pointer" onClick={() => open("https://github.com/ahqstore/client?tab=coc-ov-file#readme")}>code of conduct</span>.
        </p>
        <p>
          You must also be aware that all the data is <strong>public</strong> and can be viewed by anyone.
          This means that your account metadata, your application metadata, your avatar url will
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

          <Label>Enter your Developer Account Name (it can be different from your AHQ Store Account name)</Label>
          <Input
            contentBefore={<WindowDevEditFilled />}
            className="w-full"
            minLength={5}
            maxLength={64}
            onChange={(ev) => {
              const text = ev.target.value;

              if (text.length < 5) {
                setErr("Too Small");
              } else if (text.length > 64) {
                setErr("Too Long");
              } else {
                setErr("");
                setName(text);
              }
            }}
          />

          {err != "" && <div role="alert" className="alert alert-error alert-soft flex">
            <span>{err}</span>
          </div>}
        </div>

        <DialogActions className="w-full flex justify-end">
          <Button
            appearance="subtle"
            disabled={err != ""}
            onClick={() => {
              applyAsDeveloper(name);

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
        a GitHub issue. You can check the repository for the commits and see if your user metadata
        has been committed to the repository. It will show that you have contributed to our repository.
      </p>
      <p>
        Once committed, your account metadata will slowly become available to all. You can restart
        your computer to see the new <strong className="border border-base-content px-1 py-[1px]">Developer</strong> tab in your <strong>AHQ Store</strong> application.
        Alternatively, you can open <strong className="border border-base-content px-1 py-[1px]">Settings</strong> page and click <strong className="border border-base-content px-1 py-[1px]">Refetch Data</strong>
        if you see the commit in the repository.
      </p>
      <p>
        Have a fabulous day ahead 👋!
      </p>
    </>}
  </div>;
}