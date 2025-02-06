import { invoke } from '@tauri-apps/api/core'
import { AHQStoreApplication, type SearchEntry } from "ahqstore-types"

export async function getWindows(): Promise<string> {
  return await invoke<string>('plugin:ahqstore|getWindows', {})
    .catch((_) => "");
}

export async function getLinuxDistro(): Promise<string> {
  return await invoke<string>('plugin:ahqstore|getLinuxDistro', {})
    .catch((_) => "");
}

export async function isWindows11(): Promise<boolean> {
  return await invoke<boolean>('plugin:ahqstore|isWindows11', {})
    .catch((_) => false);
}

export async function open(url: string) {
  return await invoke<void>('plugin:ahqstore|open', { url });
}

export async function setProgress(state: number, c?: number, t?: number) {
  return await invoke<void>("plugin:ahqstore|setProgress", { state, c, t });
}

export async function isDev() {
  return await invoke<boolean>("plugin:ahqstore|isDevelopment");
}

export async function showCode(code: string) {
  return await invoke<void>("plugin:ahqstore|showCode", { code });
}

export async function removeCode() {
  return await invoke<void>("plugin:ahqstore|remCode", {});
}

export async function hashUsername(username: string) {
  return await invoke<string>("plugin:ahqstore|hashUsername", { username });
}

export async function setScale(scale: number) {
  return await invoke<void>("plugin:ahqstore|setScale", { scale });
}

export async function refreshCommit() {
  return await invoke<void>("plugin:ahqstore|refreshCommit");
}

export const search = async (query: string) => {
  return await invoke<SearchEntry[]>("plugin:getAllSearch", { query });
}

export const getHome = async () => await invoke<[string, string[]][]>("plugin:getHome");
export const getApp = async (app: string) => await invoke<AHQStoreApplication>("plugin:getApp", { app });