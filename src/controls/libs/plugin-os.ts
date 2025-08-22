import type { OsType } from "@tauri-apps/plugin-os";
import { type } from "@tauri-apps/plugin-os";

const osType = type();

// A helper function to get the OS type, which returns a Promise
export function getOsType(): Promise<OsType> {
  return new Promise((r) => r(osType));
}
