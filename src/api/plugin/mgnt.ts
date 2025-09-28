import { Capability, CommunicationInterface, EventName, EventType, Metadata, ResponseStatus } from "@ahqstore/plugin-api"
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow"
import { existsUI, getAsset, meta } from ".";
import { toast } from "sonner";

const winda = getCurrentWebviewWindow();

export interface Manifest {
  displayName: string;
  capabilities: Capability[];
  worker: boolean;
}

export enum PluginFlags {
  WorkerPlugin = 0b0001,
  PluginUI = 0b0010,
  SettingsUI = 0b0100,
  Disabled = 0b1000
}

export class Flags {
  flags: number;

  constructor(flags: number) {
    this.flags = flags;
  }

  has(flag: PluginFlags) {
    return (this.flags & flag) > 0
  }

  add(flag: PluginFlags) {
    this.flags |= flag
  }

  remove(flag: PluginFlags) {
    this.flags &= ~flag;
  }

  toggle(flag: PluginFlags) {
    this.flags ^= flag;
  }
}

export interface PluginMngtData {
  man: Manifest;
  id: string;
  hInstance: AStorePlugin
  registersSource: boolean;
  sourceName: string | undefined;
}

export interface BasicPluginData {
  man: Manifest;
  flags: Flags;
  displayName: string;
}

export class AStorePluginManager {
  static #instance: AStorePluginManager | undefined;

  // Defines worker plugins that need special care
  workerPlugins: Map<string, PluginMngtData> = new Map();

  registeredSources: Map<string, string> = new Map();

  // Defines UI plugins
  // UI plugins can also be worker plugins
  uiPlugins: Map<string, BasicPluginData> = new Map();

  private constructor() {
    winda.listen<string>("state-update", (ev) => {
      const plugin = this.workerPlugins.get(ev.payload);

      if (plugin && plugin.hInstance.capability.includes(Capability.RequestsEvents)) {
        plugin.hInstance.emit(EventName.CommonStateUpdated);
      }
    });
  }

  static sendThemeUpdate() {
    try {
      const me = AStorePluginManager.getInstance(true);

      const plugin = [...me.workerPlugins.entries()];

      for (let i = 0; i < plugin.length; i++) {
        const [, mnt] = plugin[i];

        if (mnt.man.capabilities.includes(Capability.RequestsEvents)) {
          mnt.hInstance.emit(EventName.OnThemeUpdate);
        }
      }
    } catch (e) {
      // Ignore since that means plugins not enabled
    }
  }

  static getInstance(strict = false) {
    if (!AStorePluginManager.#instance) {
      if (strict) {
        throw new Error("Strict mode is on, instance not instantiated");
      }
      AStorePluginManager.#instance = new AStorePluginManager();
    }

    return AStorePluginManager.#instance;
  }

  static hasInstance() {
    return AStorePluginManager.#instance !== undefined;
  }

  static async create(plugins: string[]) {
    const me = AStorePluginManager.getInstance();

    const final: Promise<PluginMngtData | null>[] = [];
    const uiPlugins: Map<string, BasicPluginData> = new Map();
    const enabled = (() => {
      try {
        const data = JSON.parse(localStorage.getItem("enabled-plugins")!!);

        if (data == null) {
          throw new Error("");
        }

        return data as string[];
      } catch (e) {
        localStorage.setItem("enabled-plugins", "[]");
        return [];
      }
    })();

    for (let i = 0; i < plugins.length; i++) {
      const id = plugins[i];

      try {
        const data = await meta(id);

        const flags = new Flags(0);

        if (data && data.displayName && Array.isArray(data.capabilities)) {
          if (!enabled.includes(id)) {
            flags.add(PluginFlags.Disabled);
          }
          if (data.worker) {
            flags.flags |= PluginFlags.WorkerPlugin;
          }

          if (await existsUI(id, "pluginUI.html")) {
            flags.flags |= PluginFlags.PluginUI;
          }

          if (await existsUI(id, "settings.html")) {
            flags.flags |= PluginFlags.SettingsUI;
          }

          if (data.worker && enabled.includes(id)) {
            final.push((async () => {
              const inst = new AStorePlugin(data.capabilities);

              let manifest: {
                registersSource: boolean;
                sourceName: string | undefined;
              };
              try {
                manifest = await inst.getInstance(id);
              } catch (e) {
                console.log("---- ERROR ------");
                console.warn(e);
                console.log("-----------------");
                return null;
              }

              return {
                man: data,
                id,
                displayName: data.displayName,
                hInstance: inst,
                registersSource: manifest.registersSource,
                sourceName: manifest.sourceName
              } as PluginMngtData;
            })());
          }

          uiPlugins.set(id, {
            flags,
            man: data,
            displayName: data.displayName
          });
          console.log(uiPlugins);
        }
      } catch (e) {
        console.log(e);
      }
    }

    const finalResolved = await Promise.all(final);

    finalResolved
      .filter((x) => x != null)
      .forEach((d) => {
        me.workerPlugins.set(d.id, d);
      });
    me.uiPlugins = uiPlugins;
  }
}

const AHQSTORE_PLUGIN_API_CURRENT = 0 as const;

export class AStorePlugin {
  worker: Worker;
  capability: Capability[];
  sourceRepoName: string | undefined;

  id: string = "";
  registered = false;

  constructor(cap: Capability[]) {
    this.worker = new Worker(new URL("./worker.js", import.meta.url));
    this.capability = cap;
  }

  async getInstance(plugin: string): Promise<{ registersSource: boolean; sourceName: string | undefined; }> {
    // ArrayBuffer by design
    this.id = plugin;
    const data = await getAsset(plugin, "worker.js");

    return new Promise((res, rej) => {
      this.worker.postMessage(data, [data]);

      // Auto disable after 3 secs unless registered
      const closure = setTimeout(() => {
        this.worker.terminate();
        rej("Timed Out");
      }, 3 * 1000);

      this.worker.onmessage = (data: MessageEvent<CommunicationInterface>) => {
        const dat = data.data;

        // Type Request
        if (dat.eventType == EventType.Request) {
          // Handle Initialization
          if (dat.event == EventName.RequestInitialization && !this.registered) {
            const meta = dat.data as Metadata;

            console.log(meta);

            if (![...meta.capabilities].every((c) => this.capability.includes(c))) {
              this.worker.postMessage({
                eventType: EventType.Response,
                status: ResponseStatus.Unauthorized,
                data: "Invalid Capabilities",
                refId: dat.refId
              } as CommunicationInterface);
              return;
            }

            if (meta.capabilities.has(Capability.AppInstallationSource)) {
              this.sourceRepoName = meta.newSourceName;
            }

            // Change capability to what it registered as
            // To prevent possible issues down the line
            this.capability = [...meta.capabilities];

            this.worker.postMessage({
              eventType: EventType.Response,
              status: ResponseStatus.Ok,
              data: AHQSTORE_PLUGIN_API_CURRENT,
              refId: dat.refId
            } as CommunicationInterface);
            clearTimeout(closure);

            res({
              registersSource: this.sourceRepoName != null,
              sourceName: this.sourceRepoName
            });
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

          try {
            // Processing
            switch (dat.event) {
              case EventName.RequestInjectCSS:
                this.verify(Capability.UsesTheming, dat.refId);

                this.typeCheck(dat.data, "string", dat.refId);

                this.injectCss(dat.data as string);

                this.worker.postMessage({
                  eventType: EventType.Response,
                  status: ResponseStatus.Ok,
                  data: "OK",
                  refId: dat.refId
                } as CommunicationInterface);

                return;
              case EventName.RequestThemeData:
                this.verify(Capability.UsesTheming, dat.refId);

                this.worker.postMessage({
                  eventType: EventType.Response,
                  status: ResponseStatus.Ok,
                  // @ts-ignore
                  data: globalThis.themeData,
                  refId: dat.refId
                } as CommunicationInterface);

                return;
              case EventName.RequestRestart:
                this.verify(Capability.RequestClientRestart, dat.refId);

                toast(
                  "A plugin has requested you to restart AHQ Store. Would you like to restart?",
                  {
                    action: {
                      label: "Yes",
                      onClick: () => {
                        window.location.reload();
                      }
                    },
                    duration: 3000
                  }
                );

                return;
              case EventName.RequestState:
                this.verify(Capability.UsesState, dat.refId);

                // TODO: Soon

                return;
              case EventName.RequestFetch:
                return;
              default:
                return;
            }
          } catch (e) {
            console.warn(`Plugin tried to perform unauthorized action. ${e}`);
          }
        }
      };
    });
  }

  private verify(cap: Capability, refId: number) {
    if (!this.capability.includes(cap)) {
      this.worker.postMessage({
        eventType: EventType.Response,
        status: ResponseStatus.Unauthorized,
        data: "Unauthorized",
        refId
      } as CommunicationInterface);
      throw new Error("Capability Failure!");
    }
  }

  private typeCheck<T>(data: T, typeToCheck: "bigint" | "boolean" | "function" | "number" | "object" | "string" | "symbol" | "undefined", refId: number) {
    if (typeof (data) != typeToCheck) {
      this.worker.postMessage({
        eventType: EventType.Response,
        status: ResponseStatus.Unauthorized,
        data: "Unauthorized",
        refId
      } as CommunicationInterface);
      throw new Error("Typecheck Failure!");
    }
  }

  injectCss(css: string) {
    let e = document.getElementById(this.id);

    if (!e) {
      const node = document.createElement("style");

      node.id = this.id;

      e = document.head.appendChild(
        node
      );
    }

    e.textContent = css;
  }

  emit<T>(ev: EventName, data?: T) {
    this.worker.postMessage({
      eventType: EventType.Event,
      event: ev,
      data
    } as CommunicationInterface);
  }

  destroy() {
    this.worker.terminate();
  }
}