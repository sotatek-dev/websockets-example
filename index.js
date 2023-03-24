import init, { register } from "./pkg/websockets.js";

init().then(() => {
  register("ws://localhost:8888", "Hello WASM", (...args) => console.log(args));
});
