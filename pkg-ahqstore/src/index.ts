import { EventType, ResponseStatus, EventName, type CommunicationInterface } from "./communication.js";
import type { AHQStoreApplication, RefId } from "ahqstore-types";

/**
 * This declares the IPC version that this api is compatible with
 * 
 * If this does not match with the one being emitted from the AHQ Store app
 * 
 * It'll error out
 */
export const PLUGIN_IPC_INTERFACE_VERSION = 0 as const;

/**
 * An enum that defines capabilities that 
 * your plugin possesses
 */
export enum Capability {
  /**
   * This allows your app to get events related to
   * lifecycle of your plugin.
   * 
   * The events are :-
   *  - **CommonStateUpdated:** Emitted when the common state gets updated
   *  - **ThemeUpdate:** Emitted when the theme updates
   */
  RequestsEvents,
  /**
   * This defines that this plugin also acts as an external
   * app installation source.
   * 
   * You are requried to register `search` and `appGet`
   * 
   * ### WARNING ⚠️
   * Please note that apps installed from external sources will never
   * be updated by AHQ Store
   */
  AppInstallationSource,
  /**
   * This means that the application is allowed to use state data that
   * is set by the `pluginUI.html` or `settings.html`
   * 
   * Generally Worker Plugins aren't allowed to access this state. But this
   * capability allows them to
   */
  UsesState,
  /**
   * This allows the app to inject custom css
   * like theme updates
   * 
   * This allows you to style the UI
   * 
   * This also allows you to get the current **Theme** preference
   * 
   * like, if its dark mode or light mode, if vibrant visuals (transparency) is enabled etc
   */
  UsesTheming,
  /**
   * This allows your application to request Client (i.e. Application)
   * frontend restart.
   * 
   * This is useful for cases like CSS Theming, or CSS Injection
   * You can restart the client and then request `CSS Injection`
   */
  RequestClientRestart,
  /**
   * This gives access to the fetch api (no any other fancy api)
   * 
   * You can `HTTP` fetch any url, provided its https://
   */
  HTTP
}

/**
 * Application ID
 * 
 * In AHQ Store these are just `string`s
 */
export type AppId = string;

/**
 * Application Interface
 * 
 * This is just a reexport from `ahqstore-types`
 * 
 * Feel free to omit the `free` functions from here
 */
export type App = AHQStoreApplication;

/**
 * The function takes the user's query as string and returns a list of AppId
 */
export type SearchFn = (term: String) => Promise<AppId[]>;

/**
 * The function takes an appId to return the `Application` manifest that AHQ Store Uses
 */
export type GetApplicationFn = (appId: AppId) => Promise<App>;

/**
 * The function takes the user's query as string and returns a list of AppId
 * 
 * This should return an Uint8Array as directed in the function declaration
 */
export type GetApplicationAssetFn = (appId: AppId, assetId: string) => Promise<Uint8Array>;

/**
 * This is an internal function
 */
export type InternalCallback = (data: CommunicationInterface) => void;

export type OnMessageCallback = ((data: MessageEvent<CommunicationInterface>) => void)
  | ((data: MessageEvent<CommunicationInterface>) => Promise<void>);

export interface ThemeData {
  dark: boolean;
  vibrant: boolean;
}

export interface FetchOptions {
  url: string;
  method: "GET" | "HEAD" | "OPTIONS" | "TRACE" | "PUT" | "DELETE" | "POST" | "PATCH" | "CONNECT";
  /**
   * # WARNING 🚨
   * - We recommend a body smaller than `20MB`
   * - Our wrapper will exclipitly deny a body longer than `50MB`
   * 
   * Both of them are a recommendation can is not enforced from the ahq store side
   * 
   * ## Why?
   * The usecase of plugins should be thought of. They are to provide search and
   * manifests. Which means from the request side you'll need to send minimal data.
   * 
   * If the server sends too much data, we'll 100% get it back to you.
   */
  body?: ArrayBuffer;
  cache?: "default" | "no-store" | "reload" | "no-cache" | "force-cache";
  headers?: Headers;
  redirect?: "follow" | "error" | "manual";
}

export interface HTTPOutputData {
  ok: boolean;
  status: number;
  statusText: number;
  body: ArrayBuffer;
}

export class HTTPOutput implements HTTPOutputData {
  ok: boolean;
  status: number;
  statusText: number;
  body: ArrayBuffer;

  constructor(data: HTTPOutputData) {
    this.ok = data.ok;
    this.status = data.status;
    this.statusText = data.statusText;
    this.body = data.body;
  }

  /**
   * Converts the data into string
   * @param encoding The encoding of the string, default `utf-8`
   * @returns string output of the data
   */
  text(encoding: string = "utf-8") {
    const decoder = new TextDecoder(encoding);

    return decoder.decode(this.body)
  }

  /**
   * Converts the data into json
   * @param encoding The encoding of the string, default `utf-8`
   * @returns Type of data
   */
  json<T>(encoding: string = "utf-8"): T {
    return JSON.parse(this.text(encoding))
  }

  /**
   * Converts the data to object URL
   * @returns Object URL of the data
   */
  toObjectURL(): string {
    return URL.createObjectURL(
      new Blob([this.body])
    );
  }
}

export interface Metadata {
  capabilities: Set<Capability>,
  newSourceName?: string;
}

/**
 * This is the global instance of an `AHQStore` Plugin
 * 
 * ```ts
 * import { Plugin } from "@ahqstore/plugin-api"
 * 
 * (async() => {
 *  const api = new Plugin(
 *   {
 *     capabilities: []
 *   }
 *  );
 *  
 *  // Must call after creation
 *  // AHQ Store has a timeout after which
 *  // it'll otherwise terminate your plugin
 *  await api.initialize();
 * })()
 * ```
 */
export class Plugin {
  static #instance: Plugin;
  static #constructed: boolean = false;
  static #registered = false;
  static #counter = 0;

  private emitters: {
    [key: string]: ((data: any) => void)[]
  } = {};
  private fetchSizeLimitInBytes = 50 * 1024 * 1024;

  private capabilities: Set<Capability> = new Set();
  private newSourceName?: string;
  private onMessage?: OnMessageCallback;

  private responseHandlingQueue: Map<RefId, InternalCallback> = new Map();

  private search?: SearchFn;
  private getApp?: GetApplicationFn;
  private getAppAsset?: GetApplicationAssetFn;

  /**
   * This abstracts away the complexities of the AHQStore Plugin api
   * 
   * This constructor also handles the IPC communication and gives you a quick way to community
   * via async wrappers
   * 
   * Use {@link registerSearchFn}, {@link registerAppFetchFn}, {@link registerAppAssetFetchFn} to register
   * handlers for the capability
   * 
   * ## You must call {@link initialize} afterwards
   * 
   * @param meta Defines the metadata for your plugin
   */
  constructor(
    meta: Metadata
  ) {
    if (Plugin.#constructed) {
      throw new Error("Cannot reconstruct the Plugin constructor multiple times.");
    }

    meta.capabilities.forEach((cap) =>
      this.capabilities.add(cap)
    );

    meta.newSourceName && (this.newSourceName = meta.newSourceName);

    if (this.capabilities.has(Capability.AppInstallationSource) && !this.newSourceName) {
      throw new Error("Please provide the `newSourceName` of your plugin or remove `Capability.AppInstallationSource`");
    }

    Plugin.#constructed = true;

    Plugin.#instance = this;

    // Take ownership of message channel
    self.onmessage = (data: MessageEvent<CommunicationInterface>) => {
      this.onMessage && this.onMessage(data);

      const payload = data.data;

      if (payload.eventType == EventType.Response) {
        const ref = this.responseHandlingQueue.get(payload.refId);

        typeof (ref) == "function" && (
          ref(payload)
        )

        this.responseHandlingQueue.delete(payload.refId);
      } else if (payload.eventType == EventType.Event) {
        const ev: EmittedEvent = (() => {
          switch (payload.event) {
            case EventName.CommonStateUpdated:
              return "commonStateUpdate";
            case EventName.CommonStateUpdated:
              return "themeUpdate";
            default:
              throw new Error("Impossible");
          }
        })();

        this.emit(ev, payload.data);
      } else {
        const promise = (() => {
          switch (payload.event) {
            case EventName.Search:
              return this.nonNullPromise(this.search)(payload.data as string)
            case EventName.AppFetch:
              return this.nonNullPromise(this.getApp)(payload.data as string)
            case EventName.AppAssetFetch:
              const data: [string, string] = payload.data as any;

              return this.nonNullPromise(this.getAppAsset)(data[0] as string, data[1] as string)
            default:
              break;
          }
        })();


        promise && promise.then((output) => {
          this.sendResponse({
            data: output,
            eventType: EventType.Response,
            refId: payload.refId,
            status: ResponseStatus.Ok
          });
        })
          .catch((e) => {
            console.error(e);
            this.sendResponse({
              data: null,
              eventType: EventType.Response,
              refId: payload.refId,
              status: ResponseStatus.Error_Terminate
            });
          });
      }
    }
  }

  private sendRequest(data: CommunicationInterface, callback: InternalCallback, transfer: Transferable[] = []) {
    Plugin.#counter += 1;

    const refId = Plugin.#counter;

    self.postMessage({
      ...data,
      refId
    } as CommunicationInterface, {
      transfer
    });

    this.responseHandlingQueue.set(refId, callback);
  }

  private sendAsyncRequest<T = unknown>(data: CommunicationInterface, transfer: Transferable[] = []): Promise<T> {
    return new Promise((resolve, reject) => {
      this.sendRequest(data, (response) => {
        if (response.eventType == EventType.Response) {
          if (response.status == ResponseStatus.Ok) {
            resolve(data.data as unknown as T);
          } else {
            reject(`Error: ${response.status}. Outputs: ${response.data}`);
          }
        } else {
          reject("Unknown response");
        }
      });
    });
  }

  private sendResponse(data: CommunicationInterface) {
    self.postMessage({
      ...data,
      eventType: EventType.Response,
    } as CommunicationInterface);
  }

  /**
   * Initializes the AHQ Store Plugin
   * 
   * If you don't call it, the plugin manager will kill your process
   * 
   * @throws If it failed to initialize
   */
  async initialize() {
    await this.sendAsyncRequest({
      eventType: EventType.Request,
      event: EventName.RequestInitialization,
      refId: 0,
      data: {
        capabilities: this.capabilities,
        newSourceName: this.newSourceName
      }
    });

    Plugin.#registered = true;
  }

  /**
   * Injects the provided css into the client gui application
   * 
   * ## NOTE
   * This modifies the already injected css to the new css data
   *
   * Please note the above
   * 
   * @param css The css string to inject
   */
  async injectCustomCss(css: string) {
    this.ensure([Capability.UsesTheming]);

    await this.sendAsyncRequest({
      eventType: EventType.Request,
      event: EventName.RequestInjectCSS,
      data: css,
      refId: 0
    });
  }

  /**
   * Gets the AHQ Store Theme Data
   * 
   * ## NOTE
   * The returned data does not automatically
   * update with theme changes
   * 
   * @returns The theme data {@link ThemeData}
   */
  async getThemeData(): Promise<ThemeData> {
    this.ensure([Capability.UsesTheming]);

    return await this.sendAsyncRequest({
      eventType: EventType.Request,
      event: EventName.RequestThemeData,
      data: null,
      refId: 0
    });
  }

  /**
  * Requests the user to restart
  * 
  * ## NOTE
  * This does not guarantee restart
  * 
  * @param desc Explain why you would like to restart (optional, a template is already provided)
  */
  async requestRestart(desc: string = "A restart is required to apply custom theme data. Are you ready?") {
    this.ensure([Capability.RequestClientRestart]);

    await this.sendAsyncRequest({
      eventType: EventType.Request,
      event: EventName.RequestRestart,
      data: desc,
      refId: 0
    });
  }


  /**
   * Performs an HTTP request like the `fetch` api
   * 
   * The maximum size of the body is 50MB
   * 
   * @param data Please read the information at {@link FetchOptions}
   * @returns the {@link HTTPOutput} data type
   */
  async fetch(data: FetchOptions) {
    const tooBigErr = "The provided body is too big. The hard limit is 50MB";

    if (data.body) {
      const d = data.body;

      if (d instanceof ArrayBuffer) {
        if (d.byteLength > this.fetchSizeLimitInBytes) {
          throw new Error(tooBigErr);
        }
      } else {
        throw new Error("Invalid data type provided. Expected ArrayBuffer");
      }
    }

    return new HTTPOutput(
      await this.sendAsyncRequest({
        eventType: EventType.Request,
        event: EventName.RequestFetch,
        data,
        refId: 0
      }, data.body ? [data.body] : [])
    );
  }

  /**
   * 
   * Add your own custom handler to the OnMessage
   * @deprecated This is unsafe and hence marked deprecated
   * @experimental This is unsafe and hence marked so
   */
  unsafeAddCustomOnMessage(fn: OnMessageCallback) {
    this.onMessage = fn;
  }

  private nonNullPromise<T>(c?: T): T {
    if (!c) {
      return ((..._data: any[]) => {
        return new Promise((_resolve, reject) => {
          reject("Runtime Exiting... Null data encountered");
        });
      }) as unknown as T;
    }

    return c!!;
  }

  private ensure(c: Capability[]) {
    if (!Plugin.#registered) {
      throw new Error(`Please register your plugin before you run any functions`);
    }

    const unsatisfied = c.filter((cap) => !this.capabilities.has(cap));

    let errors: string[] = [];

    unsatisfied.forEach((cap) => {
      errors.push(`\`${cap}\``)
    });

    const error = errors.join(", ");

    throw new Error(`${error} is not provided`)
  }

  /**
   * 
   * Returns the single instance of the Plugin.
   * If the instance does not exist, it returns nothing.
   *
   * @returns The single Plugin instance.
   */
  static getInstance(): Plugin | undefined {
    if (Plugin.#instance) {
      return Plugin.#instance;
    }
  }

  /**
   * Register the search function and overrides the current search function if present
   * 
   * You can only register it if you have {@link Capability.AppInstallationSource}
   * @param fn The search fn itself
   */
  registerSearchFn(fn: SearchFn) {
    this.ensure([Capability.AppInstallationSource]);

    this.search = fn;
  }

  /**
   * Register a function and overrides the current function meant to prove application metadata
   * @param fn The fn itself
   */
  registerAppFetchFn(fn: GetApplicationFn) {
    this.ensure([Capability.AppInstallationSource]);

    this.getApp = fn;
  }

  /**
   * Register a function and overrides the current function meant to prove application metadata
   * @param fn The fn itself
   */
  registerAppAssetFetchFn(fn: GetApplicationAssetFn) {
    this.ensure([Capability.AppInstallationSource]);

    this.getAppAsset = fn;
  }

  /**
   * 
   * @param capability 
   * @returns True is the capability is present
   */
  hasCapability(capability: Capability): boolean {
    return this.capabilities.has(capability)
  }

  on<T>(event: EmittedEvent, handler: (data: T) => {}): UnregisterFn {
    this.ensure([Capability.RequestsEvents]);

    if (!this.emitters[event]) {
      this.emitters[event] = [];
    }

    this.emitters[event].push(handler);

    return () => {
      this.emitters[event] = this.emitters[event]!!.filter((d) => d != handler);
    }
  }

  private emit<T>(event: EmittedEvent, data: T) {
    if (!this.emitters[event]) {
      this.emitters[event] = [];
    }

    this.emitters[event].forEach((f) => f(data));
  }
}

export type EmittedEvent = "themeUpdate" | "commonStateUpdate";
export type UnregisterFn = () => void;

export { EventType, ResponseStatus, type CommunicationInterface, EventName }