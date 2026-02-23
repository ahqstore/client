import { Store } from "../store";

export const devIdStore = new Store<string | undefined>(undefined);
export const backToPage = new Store<number | undefined>(undefined);