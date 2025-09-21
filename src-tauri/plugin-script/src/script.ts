import { AHQStore } from "./ahqstore.js";
import "./types"

if (typeof window !== 'undefined') {
  window.AHQStore = new AHQStore();
}