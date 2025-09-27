import { invoke } from "@tauri-apps/api/core";

export interface Commits {
  ahqstore: string,
  alt: string,
}

export class AHQStore {
  constructor() {
    console.log("AHQStore instance created!");
  }

  /**
   * Gets the version of the current AHQ Store Runtime
   * @returns {string}
   */
  getVersion(): string {
    return "1.0.0";
  }

  /**
   * Returns the `sha` that the current AHQ Store session is using
   * 
   * ## NOTE
   * This version might change after a while so call it and immediately
   * use it
   * 
   * Do not store it since it might get outdated
   * 
   * @returns {Promise<Commits>}
   */
  async getCurrentAHQStoreCommit(): Promise<Commits> {
    const data = await invoke<ArrayBuffer>("plugin:ahqstore|get_commit")

    return JSON.parse((new TextDecoder()).decode(data));
  }
}
