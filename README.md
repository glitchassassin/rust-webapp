# rust-webapp

An experimental full-stack webapp in Rust

## Purpose

To experiment with the [WebAssembly use case](https://webassembly.org/docs/use-cases/) "Main frame in WebAssembly, but the UI is in JavaScript / HTML."

## Architecture

This will be a simple chat app, with a backend Websocket server that forwards all messages. No real attention is paid to security, so this ought not to be used in production.

### Websocket server

```
cd server
cargo run
```

The Warp server will run on port 8000, and will proxy any requests (aside from the websocket) to the Perseus server on port 8080.

### Frontend

In this experiment, we're using the Perseus front-end framework to do DOM manipulation.

For setup details, see [wasm-client/README.md](wasm-client/README.md).

```
cd wasm-client
perseus serve
```

## Results

Once set up, you should be able to open a pair of browser windows at `http://localhost:8000/` and interact with a very simplistic IRC-like chat interface!