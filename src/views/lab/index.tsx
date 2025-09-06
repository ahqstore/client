import { Category } from "@/components/category";
import { ConfigSelect } from "@/components/select";
import { docs, setExperiment, useExperiments } from "@/lib/experiments";
import { Button, Switch } from "@fluentui/react-components";

import { FlaskRound, ListRestart, ActivitySquareIcon } from "lucide-react";

export default function LabPage() {
  const exp = useExperiments();
  const total = Object.entries(docs).length;

  return <div className="flex flex-col w-full justify-center items-center">
    <h1 className="mr-auto text-xl">Experiments</h1>
    <span className="mr-auto mt-1 mb-2 text-base-content/80">Each experiment may or may not end up in the release of AHQ Store</span>

    <Category
      title={`Available Experiments: ${total}`}
      description={`There are currently ${total} experiments available`}
      openable={false}
      Icon={FlaskRound}
    >
      <></>
    </Category>

    <div className="w-full mt-2 h-auto flex flex-col justify-center items-center gap-2">
      <ConfigSelect
        title={`Always show this tab`}
        description={`Restart to notice the change`}
        Icon={ActivitySquareIcon}
      >
        <Switch
          // @ts-ignore
          defaultChecked={localStorage.getItem("always-show-exp") == "true"}
          onChange={async (_, data) => {
            localStorage.setItem("always-show-exp", String(data.checked));
          }}
        />
      </ConfigSelect>

      <ConfigSelect
        title={`Restart`}
        description={`Restart to notice any changes done from here`}
        Icon={ListRestart}
      >
        <Button size="medium" appearance="subtle" onClick={() => window.location.reload()}>Restart</Button>
      </ConfigSelect>

      <span className="text-lg">Configuration</span>

      {
        Object.entries(docs)
          .map(([key, doc], index) =>
            <ConfigSelect
              title={doc.title}
              description={doc.docs}
              Icon={doc.Icon}
              key={`${key}-${index}`}
            >
              <Switch
                // @ts-ignore
                defaultChecked={exp[key]}
                onChange={async (_, data) => {
                  const checked = data.checked;

                  // @ts-ignore
                  exp[key] = checked;

                  setExperiment(exp)
                }}
              />
            </ConfigSelect>
          )
      }
    </div>
  </div>;
}