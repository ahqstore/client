import "./App.css";

import { WindowTitlebar } from "./controls"
import ShowSpinner from "./spinner";

function App() {
  return (
    <>
      <WindowTitlebar data-tauri-drag-region>
      </WindowTitlebar>

      <div className="content justify-center text-center items-center">
        <img src="/icon.png" className="w-[128px] h-[128px] my-[20vh]" />
        <ShowSpinner />
      </div>
    </>
  );
}

export default App;
