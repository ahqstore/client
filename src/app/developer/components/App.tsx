import { useEffect, useRef, useState } from "react";

//Icons
import { MdModeEdit } from "react-icons/md";
import { IoIosNotifications } from "react-icons/io";

//API
import { appData, getResource } from "../../resources/api/fetchApps";
import Toast from "../../resources/api/toast";
import { invoke } from "@tauri-apps/api/core";

export default function App({
  appInfo,
  dark,
  toast,
  lastIndex,
}: {
  appInfo: appData;
  dark: boolean;
  toast: typeof Toast;
  lastIndex: boolean;
}) {
  const updating = false;
  const data = useRef<HTMLDivElement>("" as any);

  const [icon, setIcon] = useState<string>();

  async function handleClick() {
    toast("Opened in browser", "success", 1);
    invoke("open", {
      url: "https://github.com/ahqstore/apps",
    });
  }

  useEffect(() => {
    (async () => {
      setIcon(await getResource(appInfo.appId, "0"));
    })();
  }, [appInfo.appId]);

  return (
    <div
      className={`mx-2 rounded-md my-1 flex min-h-[4.5rem] max-h-[4.5rem] max-w-[100%] ${
        dark ? "bg-gray-800 text-white" : "bg-gray-100 text-slate-800"
      } ${lastIndex ? "rounded-b-md" : ""} hover:shadow-xl pl-2 cursor-default`}
    >
      {icon ? (
        <img
          src={icon}
          alt={appInfo.appDisplayName}
          className={`mr-2`}
          draggable={false}
          style={{
            width: "60px",
            marginTop: "auto",
            marginBottom: "auto",
          }}
        />
      ) : (
        <div
          className={`dui-loading dui-loading-lg dui-loading-ring mt-5 mr-2 mb-[0.75rem] ${dark ? "text-white" : ""
            }`}
        />
      )}

      <div className="flex flex-col my-auto text-start">
        <h1 className={`flex ${dark ? "text-blue-400" : "text-blue-700"}`}>
          <span className="text-2xl">{appInfo.appDisplayName}</span>
          {updating ? (
            <div className={`${dark ? "text-yellow-500" : "text-yellow-900"}`}>
              <IoIosNotifications />
            </div>
          ) : (
            <></>
          )}
        </h1>
        <h2 className="block">
          {appInfo.description.substring(0, 64)}
          {appInfo.description.length > 64 ? "..." : ""}
        </h2>
      </div>

      {!updating ? (
        <div className="ml-auto mr-3 my-auto" ref={data}>
          <button
            className="flex min-w-[100%] p-4 min-h-[3rem] justify-center items-center text-center text-base-content hover:text-accent-content hover:bg-accent-foreground rounded-xl transition-all cursor-pointer"
            onClick={() => {
              handleClick();
            }}
          >
            <MdModeEdit size="1.5em" />
          </button>
        </div>
      ) : (
        <></>
      )}
    </div>
  );
}
