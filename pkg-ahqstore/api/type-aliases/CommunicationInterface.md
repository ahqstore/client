[@ahqstore/plugin-api](../globals.md) / CommunicationInterface

# Type Alias: CommunicationInterface

> **CommunicationInterface** = \{ `data`: `unknown`; `event`: [`EventName`](../enumerations/EventName.md); `eventType`: [`Request`](../enumerations/EventType.md#request); `refId`: `number`; \} \| \{ `data`: `unknown`; `event`: [`EventName`](../enumerations/EventName.md); `eventType`: [`Event`](../enumerations/EventType.md#event); \} \| \{ `data`: `unknown`; `eventType`: [`Response`](../enumerations/EventType.md#response); `refId`: `number`; `status`: [`ResponseStatus`](../enumerations/ResponseStatus.md); \}

Defined in: [communication.ts:65](https://github.com/ahqstore/client/blob/9e99876bd0536b03acf88526963ab719b8250898/pkg-ahqstore/src/communication.ts#L65)

**`Experimental`**

This whole interface is also internally managed by our plugin

## Type Declaration

\{ `data`: `unknown`; `event`: [`EventName`](../enumerations/EventName.md); `eventType`: [`Request`](../enumerations/EventType.md#request); `refId`: `number`; \}

### data

> **data**: `unknown`

Data to be attested

### event

> **event**: [`EventName`](../enumerations/EventName.md)

Event Name [EventName](../enumerations/EventName.md)

### eventType

> **eventType**: [`Request`](../enumerations/EventType.md#request)

Event Type [EventType](../enumerations/EventType.md)

Set to [EventType.Request](../enumerations/EventType.md#request)

### refId

> **refId**: `number`

A reference id

This is the communication interface provided
if the event is a request

This gives a refId to use to fulfil the request from the
Plugin End.

\{ `data`: `unknown`; `event`: [`EventName`](../enumerations/EventName.md); `eventType`: [`Event`](../enumerations/EventType.md#event); \}

### data

> **data**: `unknown`

Data to be attested

### event

> **event**: [`EventName`](../enumerations/EventName.md)

Event Name [EventName](../enumerations/EventName.md)

### eventType

> **eventType**: [`Event`](../enumerations/EventType.md#event)

Event Type [EventType](../enumerations/EventType.md)

Set to [EventType.Request](../enumerations/EventType.md#request)

\{ `data`: `unknown`; `eventType`: [`Response`](../enumerations/EventType.md#response); `refId`: `number`; `status`: [`ResponseStatus`](../enumerations/ResponseStatus.md); \}

### data

> **data**: `unknown`

Data whose type is not yet known

### eventType

> **eventType**: [`Response`](../enumerations/EventType.md#response)

Event Type [EventType](../enumerations/EventType.md)

Set to [EventType.Response](../enumerations/EventType.md#response)

### refId

> **refId**: `number`

A reference id

### status

> **status**: [`ResponseStatus`](../enumerations/ResponseStatus.md)

This is the communication interface provided
if the event is a response

This gives an eventType and finally response data
