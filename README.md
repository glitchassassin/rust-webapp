# rust-webapp

An experimental webapp with a WASM core and JS/HTML UI

## Purpose

To experiment with the [WebAssembly use case](https://webassembly.org/docs/use-cases/) "Main frame in WebAssembly, but the UI is in JavaScript / HTML."

## Architecture

This will be a simple chat app, with a backend Websocket server that forwards all messages. No real attention is paid to security, so this ought not to be used in production.

### Websocket server

https://blog.logrocket.com/how-to-build-a-websocket-server-with-rust/

### Frontend

The UI integration can happen at any level, from DOM manipulation to a high-level API called by something like React.

In this experiment, we're using the Perseus front-end framework to do DOM manipulation.