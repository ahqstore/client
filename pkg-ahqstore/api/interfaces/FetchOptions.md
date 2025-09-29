[@ahqstore/plugin-api](../globals.md) / FetchOptions

# Interface: FetchOptions

Defined in: [index.ts:141](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L141)

## Properties

### body?

> `optional` **body**: `ArrayBuffer`

Defined in: [index.ts:157](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L157)

# WARNING 🚨
- We recommend a body smaller than `20MB`
- Our wrapper will exclipitly deny a body longer than `50MB`

Both of them are a recommendation can is not enforced from the ahq store side

## Why?
The usecase of plugins should be thought of. They are to provide search and
manifests. Which means from the request side you'll need to send minimal data.

If the server sends too much data, we'll 100% get it back to you.

***

### cache?

> `optional` **cache**: `"default"` \| `"no-store"` \| `"reload"` \| `"no-cache"` \| `"force-cache"`

Defined in: [index.ts:158](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L158)

***

### headers?

> `optional` **headers**: `Record`\<`string`, `string`\>

Defined in: [index.ts:159](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L159)

***

### method

> **method**: `"GET"` \| `"HEAD"` \| `"OPTIONS"` \| `"TRACE"` \| `"PUT"` \| `"DELETE"` \| `"POST"` \| `"PATCH"` \| `"CONNECT"`

Defined in: [index.ts:143](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L143)

***

### redirect?

> `optional` **redirect**: `"follow"` \| `"error"` \| `"manual"`

Defined in: [index.ts:160](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L160)

***

### timeout?

> `optional` **timeout**: `number`

Defined in: [index.ts:171](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L171)

The time (in milliseconds) to wait before timing out the whole request

If omitted, defaults to `5000`ms
If set to `Infinity`, waits for the whole request to respond no matter how long it takes

Must have `InfiniteTimeout` capability to set this to a number greater than `10_000`ms

#### Requires

API v1 or else its NOOP and ignored

***

### url

> **url**: `string`

Defined in: [index.ts:142](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L142)
