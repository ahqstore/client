# AHQ Store Environment Types Package

This package provides a **type declaration file** for **AHQ Store Plugin** authors, enabling full IDE support for the global classes and types available in the AHQ Store's runtime environment.

This typeset is intended for use in the JavaScript and TypeScript code that is embedded within your `pluginUI.html` and `settings.html` files.

## What is injected?

The runtime used by AHQ Store Plugin pages have a injected script that adds a new class to the
**Window** object. We call this `AHQStore` class.

You can use the `AHQStore` like this

```ts
import type { AHQStore } from "ahqstore";

const ahqstore: AHQStore = window.AHQStore;
```

> NOTE
>
> the package `ahqstore` is a ghost module and does not exist in runtime. Hence you must
> use `import type` and not `import`
