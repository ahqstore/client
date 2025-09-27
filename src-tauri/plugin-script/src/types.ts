import { AHQStore } from "./ahqstore.js";

declare global {
  interface Window {
    AHQStore: AHQStore;
  }
}

