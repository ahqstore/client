[@ahqstore/plugin-api](../globals.md) / Plugin

# Class: Plugin

Defined in: [index.ts:253](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L253)

This is the global instance of an `AHQStore` Plugin

```ts
import { Plugin } from "@ahqstore/plugin-api"

(async() => {
 const api = new Plugin(
  {
    capabilities: []
  }
 );
 
 // Must call after creation
 // AHQ Store has a timeout after which
 // it'll otherwise terminate your plugin
 await api.initialize();
})()
```

## Constructors

### Constructor

> **new Plugin**(`meta`): `Plugin`

Defined in: [index.ts:289](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L289)

This abstracts away the complexities of the AHQStore Plugin api

This constructor also handles the IPC communication and gives you a quick way to community
via async wrappers

Use [registerSearchFn](#registersearchfn), [registerAppFetchFn](#registerappfetchfn), [registerAppAssetFetchFn](#registerappassetfetchfn), [registerAppVersionFetchFn](#registerappversionfetchfn) to register
handlers for the capability

## You must call [initialize](#initialize) afterwards

#### Parameters

##### meta

[`Metadata`](../interfaces/Metadata.md)

Defines the metadata for your plugin

#### Returns

`Plugin`

## Methods

### fetch()

> **fetch**(`data`): `Promise`\<[`HTTPOutput`](HTTPOutput.md)\>

Defined in: [index.ts:597](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L597)

Performs an HTTP request like the `fetch` api

The maximum size of the body is 50MB

## Note
- This function consumes the body and you don't get the array buffer back

#### Parameters

##### data

[`FetchOptions`](../interfaces/FetchOptions.md)

Please read the information at [FetchOptions](../interfaces/FetchOptions.md)

#### Returns

`Promise`\<[`HTTPOutput`](HTTPOutput.md)\>

the [HTTPOutput](HTTPOutput.md) data type

***

### getCapabilities()

> **getCapabilities**(): `Set`\<[`Capability`](../enumerations/Capability.md)\>

Defined in: [index.ts:454](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L454)

Gets the capabilities that your worker plugin is registered with

This requires you to get the plugin initialized

#### Returns

`Set`\<[`Capability`](../enumerations/Capability.md)\>

A list of capabilities

***

### getHostApi()

> **getHostApi**(): `number`

Defined in: [index.ts:467](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L467)

Gets the api version that AHQ Store Supports

This requires you to get the plugin initialized

#### Returns

`number`

API Version as number

***

### getState()

> **getState**(`stateId`): `Promise`\<`string`\>

Defined in: [index.ts:509](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L509)

Gets the state under the stateId specified

#### Parameters

##### stateId

`string`

ID (file of the state)

#### Returns

`Promise`\<`string`\>

The string data of the state

***

### getTargetApi()

> **getTargetApi**(): `number`

Defined in: [index.ts:477](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L477)

Gets the target api that the plugin is **intended** to work with.

#### Returns

`number`

API Version as number

***

### getThemeData()

> **getThemeData**(): `Promise`\<[`ThemeData`](../interfaces/ThemeData.md)\>

Defined in: [index.ts:551](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L551)

Gets the AHQ Store Theme Data

## NOTE
The returned data does not automatically
update with theme changes

#### Returns

`Promise`\<[`ThemeData`](../interfaces/ThemeData.md)\>

The theme data [ThemeData](../interfaces/ThemeData.md)

***

### hasCapability()

> **hasCapability**(`capability`): `boolean`

Defined in: [index.ts:750](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L750)

#### Parameters

##### capability

[`Capability`](../enumerations/Capability.md)

#### Returns

`boolean`

True is the capability is present

***

### initialize()

> **initialize**(): `Promise`\<`void`\>

Defined in: [index.ts:427](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L427)

Initializes the AHQ Store Plugin

If you don't call it, the plugin manager will kill your process

#### Returns

`Promise`\<`void`\>

#### Throws

If it failed to initialize

***

### injectCustomCss()

> **injectCustomCss**(`css`): `Promise`\<`void`\>

Defined in: [index.ts:492](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L492)

Injects the provided css into the client gui application

## NOTE
This modifies the already injected css to the new css data

Please note the above

#### Parameters

##### css

`string`

The css string to inject

#### Returns

`Promise`\<`void`\>

***

### on()

#### Call Signature

> **on**(`event`, `handler`): [`UnregisterFn`](../type-aliases/UnregisterFn.md)

Defined in: [index.ts:754](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L754)

##### Parameters

###### event

`"themeUpdate"`

###### handler

(`data`) => `void`

##### Returns

[`UnregisterFn`](../type-aliases/UnregisterFn.md)

#### Call Signature

> **on**(`event`, `handler`): [`UnregisterFn`](../type-aliases/UnregisterFn.md)

Defined in: [index.ts:755](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L755)

##### Parameters

###### event

`"commonStateUpdate"`

###### handler

(`data`) => `void`

##### Returns

[`UnregisterFn`](../type-aliases/UnregisterFn.md)

***

### registerAppAssetFetchFn()

> **registerAppAssetFetchFn**(`fn`): `void`

Defined in: [index.ts:738](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L738)

Register a function and overrides the current function meant to provide application metadata

#### Parameters

##### fn

[`GetApplicationAssetFn`](../type-aliases/GetApplicationAssetFn.md)

The fn itself

#### Returns

`void`

***

### registerAppFetchFn()

> **registerAppFetchFn**(`fn`): `void`

Defined in: [index.ts:716](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L716)

Register a function and overrides the current function meant to provide application metadata

#### Parameters

##### fn

[`GetApplicationFn`](../type-aliases/GetApplicationFn.md)

The fn itself

#### Returns

`void`

***

### registerAppVersionFetchFn()

> **registerAppVersionFetchFn**(`fn`): `void`

Defined in: [index.ts:727](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L727)

Register a function and overrides the current function meant to provide application versions

#### Parameters

##### fn

[`GetApplicationVersionsFn`](../type-aliases/GetApplicationVersionsFn.md)

The fn itself

#### Returns

`void`

***

### registerSearchFn()

> **registerSearchFn**(`fn`): `void`

Defined in: [index.ts:705](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L705)

Register the search function and overrides the current search function if present

You can only register it if you have [Capability.AppInstallationSource](../enumerations/Capability.md#appinstallationsource)

#### Parameters

##### fn

[`SearchFn`](../type-aliases/SearchFn.md)

The search fn itself

#### Returns

`void`

***

### requestRestart()

> **requestRestart**(`desc`): `Promise`\<`void`\>

Defined in: [index.ts:572](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L572)

Requests the user to restart

## NOTE
This does not guarantee restart

#### Parameters

##### desc

`string` = `"A restart is required to apply custom theme data. Are you ready?"`

Explain why you would like to restart (optional, a template is already provided)

#### Returns

`Promise`\<`void`\>

***

### setState()

> **setState**(`stateId`, `stateData`): `Promise`\<`string`\>

Defined in: [index.ts:527](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L527)

Sets the state under the stateId specified

#### Parameters

##### stateId

`string`

ID (file of the state)

##### stateData

`string`

Data of the state to update it with

#### Returns

`Promise`\<`string`\>

The string data of the state

***

### ~~unsafeAddCustomOnMessage()~~

> **unsafeAddCustomOnMessage**(`fn`): `void`

Defined in: [index.ts:641](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L641)

**`Experimental`**

Add your own custom handler to the OnMessage

#### Parameters

##### fn

[`OnMessageCallback`](../type-aliases/OnMessageCallback.md)

#### Returns

`void`

#### Deprecated

This is unsafe and hence marked deprecated
 This is unsafe and hence marked so

***

### getInstance()

> `static` **getInstance**(): `undefined` \| `Plugin`

Defined in: [index.ts:693](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L693)

Returns the single instance of the Plugin.
If the instance does not exist, it returns nothing.

#### Returns

`undefined` \| `Plugin`

The single Plugin instance.
