# AHQ Store Plugin API

This is an Plugin API Wrapper

Traditionally, AHQ Store Plugin API is based on **Web Workers** messages which are tedious to parse and the format keeps changing over time.

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
