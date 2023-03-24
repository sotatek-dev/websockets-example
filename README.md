# Websockets

You can build the example locally with:

```
$ cargo install wasm-pack
$ wasm-pack build --target web
```

Then serve the statics and navigate to `host:port`

```
# static server from https://crates.io/crates/https
http

# or use python
python2 -m SimpleHTTPServer
python3 -m http.server
```

## Others

- https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
- https://rustwasm.github.io/wasm-bindgen/examples/websockets.html
- https://github.com/3Hren/msgpack-rust
