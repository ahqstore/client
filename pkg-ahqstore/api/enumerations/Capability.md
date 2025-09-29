[@ahqstore/plugin-api](../globals.md) / Capability

# Enumeration: Capability

Defined in: [index.ts:15](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L15)

An enum that defines capabilities that 
your plugin possesses

## Enumeration Members

### AppInstallationSource

> **AppInstallationSource**: `1`

Defined in: [index.ts:35](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L35)

This defines that this plugin also acts as an external
app installation source.

You are requried to register `search` and `appGet`

### WARNING ⚠️
Please note that apps installed from external sources will never
be updated by AHQ Store

***

### HTTP

> **HTTP**: `5`

Defined in: [index.ts:68](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L68)

This gives access to the fetch api (no any other fancy api)

You can `HTTP` fetch any url, provided its https://

***

### InfiniteTimeout

> **InfiniteTimeout**: `6`

Defined in: [index.ts:74](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L74)

Allows HTTP Request to set timeouts longer than `10_000`ms

#### Requires

API v1 else its a NOOP

***

### RequestClientRestart

> **RequestClientRestart**: `4`

Defined in: [index.ts:62](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L62)

This allows your application to request Client (i.e. Application)
frontend restart.

This is useful for cases like CSS Theming, or CSS Injection
You can restart the client and then request `CSS Injection`

***

### RequestsEvents

> **RequestsEvents**: `0`

Defined in: [index.ts:24](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L24)

This allows your app to get events related to
lifecycle of your plugin.

The events are :-
 - **CommonStateUpdated:** Emitted when the common state gets updated
 - **ThemeUpdate:** Emitted when the theme updates

***

### UpdatesState

> **UpdatesState**: `7`

Defined in: [index.ts:80](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L80)

Allows to update the Plugin State.

#### Requires

API v1 else its a NOOP

***

### UsesState

> **UsesState**: `2`

Defined in: [index.ts:43](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L43)

This means that the application is allowed to use state data that
is set by the `pluginUI.html` or `settings.html`

Generally Worker Plugins aren't allowed to access this state. But this
capability allows them to

***

### UsesTheming

> **UsesTheming**: `3`

Defined in: [index.ts:54](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L54)

This allows the app to inject custom css
like theme updates

This allows you to style the UI

This also allows you to get the current **Theme** preference

like, if its dark mode or light mode, if vibrant visuals (transparency) is enabled etc
