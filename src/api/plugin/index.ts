import { platform } from "@tauri-apps/plugin-os";

export const root = () => {
  if (platform() == "windows") {
    return "http://ahqstoreplugin.localhost";
  } else {
    return "ahqstoreplugin:/";
  }
}

export const base = root();

const chars = 'abcdefghijklmnopqrstuvwxyz0123456789'.split("");
export function genUniquePluginId() {
  let id = ''

  for (let i = 0; i < 64; i++) {
    id += chars[Math.floor(Math.random() * chars.length)];
  }

  return id;
}

export async function plugins(): Promise<string[]> {
  const data = await fetch(`${base}/plug/`)
    .then((d) => d.json());

  return data;
}

export let dataPlugins: string[] = [];

export async function initPluginDaemonService() {
  if (platform() != "android") {
    const plug = await plugins();

    dataPlugins = plug;
    console.log(dataPlugins);
  }
}