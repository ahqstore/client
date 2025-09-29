[@ahqstore/plugin-api](../globals.md) / HTTPOutput

# Class: HTTPOutput

Defined in: [index.ts:182](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L182)

## Implements

- [`HTTPOutputData`](../interfaces/HTTPOutputData.md)

## Constructors

### Constructor

> **new HTTPOutput**(`data`): `HTTPOutput`

Defined in: [index.ts:189](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L189)

#### Parameters

##### data

[`HTTPOutputData`](../interfaces/HTTPOutputData.md)

#### Returns

`HTTPOutput`

## Properties

### body

> **body**: `ArrayBuffer`

Defined in: [index.ts:187](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L187)

#### Implementation of

[`HTTPOutputData`](../interfaces/HTTPOutputData.md).[`body`](../interfaces/HTTPOutputData.md#body)

***

### headers

> **headers**: `Record`\<`string`, `string`\>

Defined in: [index.ts:186](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L186)

#### Implementation of

[`HTTPOutputData`](../interfaces/HTTPOutputData.md).[`headers`](../interfaces/HTTPOutputData.md#headers)

***

### ok

> **ok**: `boolean`

Defined in: [index.ts:183](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L183)

#### Implementation of

[`HTTPOutputData`](../interfaces/HTTPOutputData.md).[`ok`](../interfaces/HTTPOutputData.md#ok)

***

### status

> **status**: `number`

Defined in: [index.ts:184](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L184)

#### Implementation of

[`HTTPOutputData`](../interfaces/HTTPOutputData.md).[`status`](../interfaces/HTTPOutputData.md#status)

***

### statusText

> **statusText**: `string`

Defined in: [index.ts:185](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L185)

#### Implementation of

[`HTTPOutputData`](../interfaces/HTTPOutputData.md).[`statusText`](../interfaces/HTTPOutputData.md#statustext)

## Methods

### json()

> **json**\<`T`\>(`encoding`): `T`

Defined in: [index.ts:213](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L213)

Converts the data into json

#### Type Parameters

##### T

`T`

#### Parameters

##### encoding

`string` = `"utf-8"`

The encoding of the string, default `utf-8`

#### Returns

`T`

Type of data

***

### text()

> **text**(`encoding`): `string`

Defined in: [index.ts:202](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L202)

Converts the data into string

#### Parameters

##### encoding

`string` = `"utf-8"`

The encoding of the string, default `utf-8`

#### Returns

`string`

string output of the data

***

### toObjectURL()

> **toObjectURL**(): `string`

Defined in: [index.ts:221](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/index.ts#L221)

Converts the data to object URL

#### Returns

`string`

Object URL of the data
