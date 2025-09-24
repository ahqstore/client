import { Capability, CommunicationInterface, EventName, EventType, Metadata, ResponseStatus } from "@ahqstore/plugin-api"

import { getAsset, meta } from ".";

export interface Manifest {
  displayName: string;
  capabilities: Capability[];
  worker: boolean;
}

export interface PluginMngtData {
  man: Manifest;
  id: string;
}

interface PluginInnerData {
  displayName: string;
  hInstance: AStorePlugin
}

export class AStorePluginManager {
  static #instance: AStorePluginManager | undefined;

  map: Map<string, PluginInnerData> = new Map();
  others: Set<string> = new Set();

  private constructor(plugins: PluginMngtData[], others: Set<string>) {
    this.others = others;

    console.log(plugins, others);

    AStorePluginManager.#instance = this;
  }

  static getInstance() {
    return AStorePluginManager.#instance!!;
  }

  static async create(plugins: string[]) {
    const final: PluginMngtData[] = [];
    const normalPlugins: Set<string> = new Set();

    for (let i = 0; i < plugins.length; i++) {
      const id = plugins[i];

      try {
        const data = await meta(id);

        if (data && data.displayName && Array.isArray(data.capabilities)) {
          if (data.worker) {
            final.push({
              man: data,
              id
            });
          } else {
            normalPlugins.add(id);
          }
          console.log(data);
        }
      } catch (e) {

      }
    }

    return new AStorePluginManager(final, normalPlugins);
  }
}

export class AStorePlugin {
  worker: Worker;
  capability: Capability[];
  enabled: boolean = false;
  sourceRepoName: string | undefined;

  registered = false;

  constructor(cap: Capability[]) {
    this.worker = new Worker(new URL("./worker.js", import.meta.url));
    this.capability = cap;
  }

  async getInstance(plugin: string) {
    this.enabled = true;

    const data = await getAsset(plugin, "worker.js");

    this.worker.postMessage(data, [data]);

    // Auto disable after 3 secs unless registered
    const closer = setTimeout(() => {
      this.worker.terminate();
    }, 3 * 1000);

    this.worker.onmessage = (data: MessageEvent<CommunicationInterface>) => {
      const dat = data.data;

      if (dat.eventType == EventType.Request) {
        if (dat.event == EventName.RequestInitialization && !this.registered) {
          const meta = dat.data as Metadata;

          if (!meta.capabilities.every((c) => this.capability.includes(c))) {
            this.worker.postMessage({
              eventType: EventType.Response,
              status: ResponseStatus.Unauthorized,
              data: "Invalid Capabilities",
              refId: dat.refId
            } as CommunicationInterface);
            return;
          }

          if (meta.capabilities.includes(Capability.AppInstallationSource)) {
            this.sourceRepoName = meta.newSourceName;
          }

          this.worker.postMessage({
            eventType: EventType.Response,
            status: ResponseStatus.Unauthorized,
            data: "Invalid Capabilities",
            refId: dat.refId
          } as CommunicationInterface);
          return;
        }

        if (!this.registered) {
          this.worker.postMessage({
            eventType: EventType.Response,
            status: ResponseStatus.Unauthorized,
            data: "Register",
            refId: dat.refId
          } as CommunicationInterface);
          return;
        }
      }
    };
  }
}