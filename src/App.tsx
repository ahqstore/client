import "./App.css";

import { WindowTitlebar } from "./controls"
import ShowSpinner from "./views/spinner";

import { platform } from "@tauri-apps/plugin-os"

function App() {
  return (
    <>
      {platform() != "android" && <WindowTitlebar data-tauri-drag-region>
        <img src="/icon.png" className="ml-2 my-2 w-[26px] h-[26px]" />
        <h1 className="my-auto ml-2 text-md font-sans">AHQ Store</h1>
        <h1 className="my-auto ml-1 text-neutral-content text-md font-sans italic font-bold">NEO</h1>
      </WindowTitlebar>}

      <div className="content justify-center text-center items-center">
        <img src="/icon.png" className="w-[128px] h-[128px] my-[20vh]" />
        <ShowSpinner />
      </div>
    </>
  );
}

export default App;
