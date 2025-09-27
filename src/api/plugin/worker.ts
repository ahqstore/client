import "ses";

const MAX_LOGS_OVERALL = 8000;

lockdown();

let logs = 0;
let lastLogAt = 0;

const log = (c: any) => {
  const now = Date.now();

  // Max 1 log every 200ms
  if (lastLogAt > (now + 200)) {
    return;
  }

  if (logs > MAX_LOGS_OVERALL) {
    return;
  }

  logs += 1;
  lastLogAt = now;

  console.log(c);
}

const c = new Compartment({
  // Critical
  Math,
  Date,
  console: {
    log: log.bind(self),
    error: log.bind(self),
    warn: log.bind(self)
  },

  // Timings
  setTimeout,
  clearTimeout,
  Promise,

  // Structs & Utilities
  Map,
  Set,
  JSON,
  RegExp,

  // Buffer
  ArrayBuffer,
  DataView,
  Blob,
  Uint8Array,
  Int8Array,
  Uint16Array,
  Int32Array,
  Float32Array,

  // Crypto
  crypto: self.crypto,

  // Core
  postMessage: self.postMessage.bind(self),
  atob,
  btoa
});

c.evaluate(`
  globalThis.self = globalThis;

  Object.assign(globalThis, { onmessage, postMessage, atob, btoa });
`);

// main worker (host)
const handlePluginMessage = (dat: any) => {
  const data = JSON.stringify(dat);

  c.evaluate(`self.onmessage && self.onmessage(${data})`);
};

self.onmessage = (d: MessageEvent<ArrayBuffer>) => {
  const code = new TextDecoder("utf-8").decode(d.data);

  c.evaluate(code);

  self.onmessage = (e) => {
    handlePluginMessage({ data: e.data });
  };
}
