declare module "ahqstore" {
    export class AHQStore {
        constructor();
        /**
         * Gets the version of the current AHQ Store Runtime
         * @returns {string}
         */
        getVersion(): string;
        /**
         * Returns the `sha` that the current AHQ Store session is using
         *
         * ## NOTE
         * This version might change after a while so call it and immediately
         * use it
         *
         * Do not store it since it might get outdated
         *
         * @returns {Promise<string>}
         */
        getCurrentAHQStoreCommit(): Promise<string>;
    }
}
declare module "types" {
    import { AHQStore } from "ahqstore";
    global {
        interface Window {
            AHQStore: AHQStore;
        }
    }
}
