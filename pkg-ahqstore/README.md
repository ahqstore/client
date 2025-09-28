# AHQ Store Plugin API

This is an Plugin API Wrapper. As you'd know AHQ Store Worker Plugins run in a specialized containerized environment called AHQStoreJS.

This whole plugin is written using the runtimes and interfaces available in AHQStoreJS and will not work in environments like **NodeJS, Deno, Bun**.

You only have to import the `Plugin` interface from `@ahqstore/plugin-api`

## Scope

This part of the API exports the functions needed to integrate an **AHQ Store**
**`CORE PLUGIN WORKER`**

A core plugin worker basically extends the core of **AHQ Store** using the provided
api and api **conventions** that keep changing like the time (I bet it has changed
since you last read it)

```ts
import { Plugin } from "@ahqstore/plugin-api";

const plugin = new Plugin();
```
