import { invoke } from '@tauri-apps/api/core'
import { AHQStoreApplication, type SearchEntry } from "ahqstore-types"

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

export const search = async (query: string) => {
  return await invoke<SearchEntry[]>("plugin:get_all_search", { query });
}

export const getHome = async () => await invoke<[string, string[]][]>("plugin:get_home");
export const getApp = async (app: string) => await invoke<AHQStoreApplication>("plugin:get_app", { app });