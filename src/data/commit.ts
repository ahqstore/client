import { getCommit } from "src-plugin/dist-js";

// Simple Memoization
let cachedCommit: string | null = null;
let clearCacheTimer: any = null;

export async function getCommitSmart() {
  if (cachedCommit) return cachedCommit;

  // Fetch from Rust
  cachedCommit = (await getCommit()).ahqstore;

  // Clear local cache after 50ms (enough time to render the whole list)
  if (!clearCacheTimer) {
    clearCacheTimer = setTimeout(() => {
      cachedCommit = null;
      clearCacheTimer = null;
    }, 50);
  }

  return cachedCommit;
}