import { Channel, invoke } from '@tauri-apps/api/core'
import { AHQStoreApplication, Commit, DevData, type SearchEntry } from "ahqstore-types"

export type DownloadEvent = {
  event: "started";
  data: {
    length: number
  }
} | {
  event: "progress";
  data: {
    progress: number;
  }
} | {
  event: "finished";
  data: {}
};

export async function getWindows(): Promise<string> {
  return await invoke<string>('plugin:ahqstore|get_windows', {})
    .catch((_) => "");
}

export async function getLinuxDistro(): Promise<string> {
  return await invoke<string>('plugin:ahqstore|get_linux_distro', {})
    .catch((_) => "");
}

export async function isWindows11(): Promise<boolean> {
  return await invoke<boolean>('plugin:ahqstore|is_windows11', {})
    .catch((_) => false);
}

export async function download(
  url: string,
  name: string,
  destDir: string,
  progressUpdate: (event: DownloadEvent) => void
) {

  const channel = new Channel<DownloadEvent>();

  channel.onmessage = (resp) => progressUpdate(resp);

  return invoke<void>("plugin:ahqstore|download", {
    url,
    name,
    path: destDir,
    channel
  });
}

export const encrypt = async (payload: string) => invoke<number[]>("plugin:ahqstore|encrypt", { payload });
export const decrypt = async (encrypted: number[]) => invoke<string>("plugin:ahqstore|decrypt", { encrypted });

export async function open(url: string) {
  return await invoke<void>('plugin:ahqstore|open', { url });
}

export async function setProgress(state: number, c?: number, t?: number) {
  return await invoke<void>("plugin:ahqstore|set_progress", { state, c, t });
}

export async function isDev() {
  return await invoke<boolean>("plugin:ahqstore|is_sevelopment");
}

export async function showCode(code: string) {
  return await invoke<void>("plugin:ahqstore|show_code", { code });
}

export async function removeCode() {
  return await invoke<void>("plugin:ahqstore|rem_code", {});
}

export async function hashUsername(username: string) {
  return await invoke<string>("plugin:ahqstore|hash_username", { username });
}

export async function setScale(scale: number) {
  return await invoke<void>("plugin:ahqstore|set_scale", { scale });
}

export async function refreshCommit() {
  return await invoke<void>("plugin:ahqstore|refresh_commit");
}

export const getCommit = async () => await invoke<Commit>("plugin:ahqstore|get_commit");

export const search = async (query: string) => {
  return await invoke<SearchEntry[]>("plugin:ahqstore|get_all_search", { query });
}

export const getHome = async () => await invoke<[string, string[]][]>("plugin:ahqstore|get_home");
export const getApp = async (app: string) => await invoke<AHQStoreApplication>("plugin:ahqstore|get_app", { app });
export const getDevData = async (dev: string) => invoke<DevData>("plugin:ahqstore|get_dev_data", { dev });
export const getAppAsset = async (app: string, asset: string) => invoke<Uint8Array>("plugin:ahqstore|get_app_asset", { app, asset });
export const getDevsApps = async (dev: string) => invoke<string[]>("plugin:ahqstore|get_devs_apps", { dev });

export const getArch = async () => invoke<string>("plugin:ahqstore|get_arch");