import { base, genUniquePluginId } from "@/api/plugin";
import { Category } from "@/components/category";
import { Input, Button } from "@fluentui/react-components";
import { PlugZapIcon, Zap } from "lucide-react";
import { toast } from "sonner";
import { Plugin } from "./plugin";
import { useMemo } from "react";
import { AStorePluginManager } from "@/api/plugin/mgnt";

export default function PluginPage() {
  // useEffect(() => {
  //   invoke("open_plugin", {
  //     plugin: "1",
  //     title: "AHQ Store Test",
  //     settings: true
  //   });
  // }, []);

  const plugins = useMemo(() => [...AStorePluginManager.getInstance().uiPlugins.keys()], []);

  const column1Plugins = plugins.filter((_, index) => index % 2 === 0); // 0, 2, 4...
  const column2Plugins = plugins.filter((_, index) => index % 2 !== 0); // 1, 3, 5...

  return <>
    <div className="flex flex-col w-full justify-center items-center">
      <h1 className="mr-auto text-xl">Plugins</h1>
      <span className="mr-auto mt-1 mb-2 text-base-content/80">This page lists the plugins that are enabled</span>

      <div role="alert" className="alert alert-error alert-soft w-full my-2 flex flex-col">
        <div className="w-full flex">
          <Zap className="size-8 mr-2" />
          <span className="text-xl">Danger</span>
        </div>
        <div className="w-full flex flex-col gap-2 text-lg">
          <span>Plugins are run in a sandboxed environment but they can have a lot of capabilities including <strong>HTTP, AppInstallation Provider</strong>. Only install plugins that you trust. </span>
          <span>Apart from that, plugins can also hang up your system causing instabilities.</span>
          <span className="text-base-content text-sm">Learn more about plugin architecture and developer guides <span className="text-blue-700 dark:text-blue-400 cursor-pointer underline" onClick={() => open("https://ahqstore.github.io/guide/plugins")}>by clicking here</span></span>
        </div>
      </div>

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

      <h1 className="mr-auto text-xl mt-2">Installed Plugins</h1>
      <span className="mr-auto mt-1 mb-2 text-base-content/80">All the plugins are listed here</span>

      <div className="mt-2 flex flex-col md:flex-row gap-2 w-full">
        <div className="w-full flex flex-col gap-2 flex-1">
          {column1Plugins.map((id) => <Plugin id={id} key={id} />)}
        </div>

        <div className="w-full flex flex-col gap-2 flex-1">
          {column2Plugins.map((id) => <Plugin id={id} key={id} />)}
        </div>
      </div>

      {/* <div className="mt-2 grid grid-cols-1 md:grid-cols-2 gap-2 w-full">
        {plugins.map((id) => <Plugin id={id} key={id} />)}
      </div> */}
    </div>
  </>;
}