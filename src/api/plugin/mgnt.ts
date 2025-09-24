import { Capability } from "@ahqstore/plugin-api"

export interface Manifest {
  capabilities: Capability[]
}

export interface PluginMeta {

}

export class AStorePluginManager {
  constructor(plugins: PluginMeta[]) {

  }
}

export class AStorePlugin {
  constructor(id: string) {

  }
}