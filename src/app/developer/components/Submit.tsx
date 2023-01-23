//React
import { useEffect, useState } from "react";

//Tauri
import { open } from "@tauri-apps/api/dialog";

//Icons
import { BiArrowBack } from "react-icons/bi";
import { BsGithub } from "react-icons/bs";

//components
import icon from "../../resources/package.png";
import { readBinaryFile } from "@tauri-apps/api/fs";

//types
interface appData {
  name: string;
  longDescription: string;
  description: string;
  icon: string;
}

export default function Submit(props: { toggle: Function; dark: boolean }) {
  const { dark } = props;
  const [appData, setAppData] = useState<appData>({
    name: "",
    longDescription: "",
    description: "",
    icon,
  });

  useEffect(() => {
    const modal = document.getElementById("drop") as HTMLDivElement;

    modal.addEventListener("click", () => {});
  }, []);

  return (
    <div className="w-[100%] h-[100%]" style={{ overflowY: "hidden" }}>
      <button
        className={`p-1 ${
          props.dark
            ? "text-slate-200 hover:bg-gray-600"
            : "text-slate-800 hover:bg-gray-100"
        } rounded-md`}
        style={{ transition: "all 125ms linear" }}
        onClick={() => props.toggle()}
      >
        <BiArrowBack size="1.5em" />
      </button>
      <form
        className="w-[100%] min-h-auto h-[100%] pb-8 flex flex-col items-center space-y-3"
        style={{ overflowY: "scroll" }}
      >
        <div className="flex w-[100%]">
          <div className="flex flex-col w-[85%]">
            <input
              className={`style-input ${dark ? "style-input-d" : ""}`}
              placeholder="App Name"
              minLength={3}
              maxLength={30}
              style={{
                minWidth: "98%",
              }}
              required
              onChange={(event) => {
                setAppData((value) => {
                  return {
                    ...value,
                    name: (event.target as HTMLInputElement).value,
                  };
                });
              }}
            />
            <input
              className={`style-input mt-2 ${dark ? "style-input-d" : ""}`}
              placeholder="Description"
              minLength={16}
              maxLength={64}
              required
              style={{
                minWidth: "98%",
              }}
              onChange={(event) => {
                setAppData((value) => {
                  return {
                    ...value,
                    description: (event.target as HTMLInputElement).value,
                  };
                });
              }}
            />
            <textarea
              className={`mt-2 style-input ${dark ? "style-input-d" : ""}`}
              placeholder="App Description"
              rows={10}
              minLength={104 * 3}
              maxLength={104 * 10}
              required
              style={{
                resize: "none",
                minHeight: "263px",
                minWidth: "98%",
              }}
              onChange={(event) => {
                setAppData((value) => {
                  return {
                    ...value,
                    longDescription: (event.target as HTMLTextAreaElement)
                      .value,
                  };
                });
              }}
            />
          </div>
          <div className="flex flex-col items-center text-center justify-center mb-auto mt-3">
            <div
              className="img img-dev"
              onClick={() => {
                open({
                  multiple: false,
                  filters: [
                    {
                      name: "icon",
                      extensions: ["png"],
                    },
                  ],
                }).then(async (data) => {
                  const imageData = await readBinaryFile(data as string);

                  const blob = new Blob([imageData]);

                  const fs = new FileReader();

                  fs.readAsDataURL(blob);

                  fs.onload = () => {
                    setAppData((value) => {
                      return {
                        ...value,
                        icon: fs.result as string,
                      };
                    });
                  };
                });
              }}
            >
              <img
                src={appData.icon}
                alt="icon"
                width={"256px"}
                height={"256px"}
                style={{
                  minWidth: "256px",
                  minHeight: "256px",
                  maxWidth: "256px",
                  maxHeight: "256px",
                }}
              />
              <h1
                className={`mt-2 ${
                  dark ? "text-slate-300" : "text-slate-700"
                } font-bolder text-3xl`}
              >
                Application Icon
              </h1>
              <div
                className={`div div-m ${props.dark ? "" : "div-l"}`}
                id="drop"
              >
                <h1 className={`text ${props.dark ? "" : "text-l"}`}>Select</h1>
              </div>
            </div>
          </div>
        </div>
        <div className="w-[100%] flex flex-col text-center items-center">
          <h1
            className={`def ${dark ? "text-slate-200" : "text-slate-700"}`}
            style={{ fontWeight: "bolder", fontSize: "22.5px" }}
          >
            Installer Details
          </h1>
          <div className="w-[100%] flex flex-col justify-start text-start">
            <div className={`${dark ? "text-slate-100" : ""} flex mb-2`}>
              <BsGithub size={"32px"} />
              <h1
                className={`ml-2 text-2xl ${
                  dark ? "text-slate-300" : "text-slate-700"
                } flex text-center justify-center`}
                style={{
                  height: "32px",
                  minHeight: "32px",
                  maxHeight: "32px",
                }}
              >
                App Repo
              </h1>
            </div>
            <div className="flex w-[100%]">
              <input
                className={`git-input ${dark ? "style-input-d" : ""}`}
                autoComplete={"off"}
                autoCorrect={"off"}
                style={{
                  height: "47px",
                  minHeight: "47px",
                  maxHeight: "47px",
                }}
              />
              <h1
                className={`text-5xl ${
                  dark ? "text-gray-500" : "text-slate-700"
                } flex text-center justify-center`}
                style={{
                  height: "47px",
                  minHeight: "47px",
                  maxHeight: "47px",
                }}
              >
                /
              </h1>
              <input
                className={`git-input ${dark ? "style-input-d" : ""}`}
                autoComplete={"off"}
                autoCorrect={"off"}
                style={{
                  height: "47px",
                  minHeight: "47px",
                  maxHeight: "47px",
                }}
              />
            </div>
          </div>
        </div>
        <button
          className="button button-success"
          type="submit"
          onClick={(e) => {
            const { name, description, longDescription } = appData;
            if (
              name.length >= 3 &&
              description.length >= 64 &&
              longDescription.length >= 104 * 3
            ) {
              e.preventDefault();
            }
          }}
        >
          Submit
        </button>
      </form>
    </div>
  );
}
