import { base, dataPlugins, genUniquePluginId } from "@/api/plugin";
import { Category } from "@/components/category";
import { Input, Button } from "@fluentui/react-components";
import { PlugZapIcon } from "lucide-react";
import { toast } from "sonner";

export default function PluginPage() {
  // useEffect(() => {
  //   invoke("open_plugin", {
  //     plugin: "1",
  //     title: "AHQ Store Test",
  //     settings: true
  //   });
  // }, []);
  const data = dataPlugins;

  return <>
    <div className="flex flex-col w-full justify-center items-center">
      <h1 className="mr-auto text-xl">Plugins</h1>
      <span className="mr-auto mt-1 mb-2 text-base-content/80">This page lists the plugins that are enabled</span>

      <Category
        title={`Install Plugin`}
        description={`Click to install plugin`}
        openable={true}
        normallyOpen={true}
        Icon={PlugZapIcon}
      >
        <form
          onSubmit={(e) => {
            e.preventDefault();

            const put: HTMLInputElement = document.getElementById("anInput")!! as any;

            put.disabled = true;
            const path = put.value;

            (async () => {
              const data = `${genUniquePluginId()}}::{${path}`;

              const val = await fetch(`${base}/inst/${data}`).then((d) => d.text());

              if (val == "NOK") {
                toast("Something went wrong while installating the plugin...");
              } else {
                window.location.reload();
              }

              put.disabled = false;
            })()
          }}
          className="flex w-full"
        >
          <Input
            className="w-full"
            placeholder="Path to plugin .zip file"
            minLength={5}
            id="anInput"
          />
          <Button appearance="secondary" type="submit">Install</Button>
        </form>
      </Category>
    </div>
  </>;
}