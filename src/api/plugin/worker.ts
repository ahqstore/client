// @ts-expect-error Security
delete self.fetch;
// @ts-expect-error Security
delete self.XMLHttpRequest;
// @ts-expect-error Security
delete self.indexedDB;
// @ts-expect-error Security
delete self.localStorage; // Not available in workers, but good practice
// @ts-expect-error Security
delete self.sessionStorage; // Not available in workers, but good practice

// WebSockets for real-time communication
// @ts-expect-error Security
delete self.WebSocket;
// @ts-expect-error Security
delete self.WebTransport;

// Other network/data APIs
// @ts-expect-error Security
delete self.EventSource; // For Server-Sent Events
// @ts-expect-error Security
delete self.navigator.sendBeacon; // For non-blocking requests
// @ts-expect-error Security
delete self.caches; // For the Cache API
// @ts-expect-error Security
delete self.FileReader; // For reading file data

// WebRTC APIs for peer-to-peer data channels
// @ts-expect-error Security
delete self.RTCPeerConnection;
// @ts-expect-error Security
delete self.RTCDataChannel;
// @ts-expect-error Security
delete self.RTCDtlsTransport;

// Worker and Service Worker APIs
// These are crucial for preventing the plugin from
// spawning new threads or controlling network traffic.
// @ts-expect-error Security
delete self.Worker;
// @ts-expect-error Security
delete self.SharedWorker;
// @ts-expect-error Security
delete self.importScripts;
// @ts-expect-error Security
delete self.ServiceWorker;

self.onmessage = (d: MessageEvent<ArrayBuffer>) => {
  const code = new TextDecoder("utf-8").decode(d.data);
  self.onmessage = null;
  // Execute plugin
  eval(code);
}