import { ApplicationData } from "../api/fetchApps";
import { WebSocketMessage, sendWsRequest } from "./handler";
import fetch from "./http";
import { Downloaded, ListedApps } from "./structs";

let sha = "";

const totalUrl = "https://rawcdn.githack.com/ahqstore/apps/{sha}/db/total";
const homeUrl = "https://rawcdn.githack.com/ahqstore/apps/{sha}/db/home.json";
const appUrl =
  "https://rawcdn.githack.com/ahqstore/apps/{sha}/db/apps/{app}.json";
const mapUrl =
  "https://rawcdn.githack.com/ahqstore/apps/{sha}/db/map/{id}.json";
const searchUrl =
  "https://rawcdn.githack.com/ahqstore/apps/{sha}/db/search/{id}.json";
const appsUserUrl =
  "https://rawcdn.githack.com/ahqstore/apps/{sha}/db/dev/{dev}";

export async function get_devs_apps(devId: string) {
  if (sha == "") {
    await get_sha();
  }
  const url = appsUserUrl.replace("{sha}", sha).replace("{dev}", devId);

  const { ok, text } = await fetch(url, {
    method: "GET",
  });
  const data = await text();

  const apps: string[] = ok ? data.split("\n") : [""];

  apps.pop();

  return apps;
}

export async function get_total() {
  if (sha == "") {
    await get_sha();
  }
  return Number(
    (
      await fetch(totalUrl.replace("{sha}", sha), {
        method: "GET"
      }).then(async (res) => ({ data: await res.text() }))
    ).data,
  );
}

export async function get_home() {
  if (sha == "") {
    await get_sha();
  }

  const url = homeUrl.replace("{sha}", sha);

  const { data } = await fetch(url, {
    method: "GET"
  }).then(async (d) => ({ ...d, data: await d.json() }));

  return data;
}

export async function get_search_data<T>() {
  if (sha == "") {
    await get_sha();
  }
  const map = [];

  const total = await get_total();

  for (let i = 1; i <= total; i++) {
    const url = searchUrl.replace("{sha}", sha).replace("{id}", i.toString());

    const val = await fetch(url, {
      method: "GET"
    }).then(async (r) => ({ ...r, data: await r.json() }));

    map.push(...val.data);
  }

  console.log(map);
  return map as unknown as any as T;
}

export async function get_map<T>(): Promise<T> {
  if (sha == "") {
    await get_sha();
  }
  const map = {};

  const total = await get_total();

  for (let i = 1; i <= total; i++) {
    const url = mapUrl.replace("{sha}", sha).replace("{id}", i.toString());

    const val = await fetch(url, {
      method: "GET"
    }).then(async (r) => ({ ...r, json: await r.json() }));

    console.log(val);
  }

  console.log(map);
  return map as unknown as any as T;
}

export function get_sha() {
  console.log("Sent");
  return new Promise((resolve) => {
    sendWsRequest(WebSocketMessage.GetSha(), (val) => {
      console.log(val);
      if (val.method == "SHAId") {
        console.log("Got Sha", val.data);
        sha = val.data as string;
        resolve(undefined);
      }
    });
  });
}

export async function get_app(app: string): Promise<ApplicationData> {
  const { data } = await fetch(
    appUrl.replace("{sha}", sha).replace("{app}", app),
    {
      method: "GET"
    },
  ).then(async (d) => ({ ...d, json: await d.json() }));

  const appData: ApplicationData = {
    ...data,
    icon: `data:image;base64,${data.icon}`,
  };

  return appData;
}

export function install_app(
  app: string,
  status_update: (data: Downloaded) => void,
): Promise<boolean> {
  return new Promise((resolve) => {
    sendWsRequest(WebSocketMessage.InstallApp(app), (val) => {
      switch (val.method) {
        case "DownloadProgress":
          status_update(val.data as Downloaded);
          break;
        case "Installing":
          status_update({
            c: 10000,
            t: 0,
          });
          break;
        case "Installed":
          resolve(true);
          break;
        case "Error":
          resolve(false);
          break;
        default:
          break;
      }
    });
  });
}

export function list_apps(): Promise<{ id: string; version: string }[]> {
  return new Promise((resolve) => {
    sendWsRequest(WebSocketMessage.ListApps(), (val) => {
      if (val.method == "ListApps") {
        resolve(
          (val.data as ListedApps).map(([id, version]) => ({
            id,
            version,
          })),
        );
      }
    });
  });
}

interface Prefs {
  launch_app: boolean;
  install_apps: boolean;
}

export type { Prefs };

let accessPrefs: Prefs | undefined;

export async function get_access_perfs(): Promise<Prefs> {
  if (accessPrefs != undefined) {
    return accessPrefs;
  }

  return new Promise((resolve) => {
    sendWsRequest(WebSocketMessage.GetPrefs(), (val) => {
      if (val.method == "Prefs") {
        accessPrefs = val.data as Prefs;
        resolve(val.data as Prefs);
      }
    });
  });
}

export async function set_access_prefs(prefs: Prefs): Promise<void> {
  accessPrefs = undefined;
  return new Promise((resolve) => {
    sendWsRequest(WebSocketMessage.SetPrefs(prefs), (val) => {
      if (val.method == "PrefsSet") {
        resolve();
      }
    });
  });
}

export function un_install(app: string): Promise<void> {
  return new Promise((resolve, reject) => {
    sendWsRequest(WebSocketMessage.UninstallApp(app), (val) => {
      if (val.method == "Uninstalled") {
        resolve();
      } else if (val.method == "Error") {
        reject();
      }
    });
  });
}
