import { platform } from "@tauri-apps/plugin-os";
import { AStorePluginManager, Manifest } from "./mgnt";

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

export async function meta(plugin: string): Promise<Manifest> {
  const data = await fetch(`${base}/meta/${plugin}`)
    .then((d) => d.text())
    .then((txt) => {
      if (txt == "") {
        throw new Error("Invalid");
      }

      return JSON.parse(txt);
    });

  return data;
}

/**
 * Only string assets
 * @param plugin 
 * @param asset 
 * @returns 
 */
export async function getAsset(plugin: string, asset: string): Promise<ArrayBuffer> {
  const data = await fetch(`${base}/asst/${plugin}}::{${asset}`)
    .then((d) => d.arrayBuffer());

  return data;
}

/**
 * Checks if the file exists
 * @param plugin 
 * @param asset 
 * @returns 
 */
export async function existsUI(plugin: string, asset: string): Promise<boolean> {
  const data = await fetch(`${base}/asst/${plugin}}::{${asset}`)
    .then((d) => d.ok);

  return data;
}

export async function initPluginDaemonService() {
  if (platform() != "android") {
    if (!AStorePluginManager.hasInstance()) {
      console.log("Creating instance");
      const plug = await plugins();

      await AStorePluginManager.create(plug);
    }
  }
}