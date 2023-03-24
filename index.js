import init, { register } from "./pkg/websockets.js";

init().then(() => {
  register("wss://echo.websocket.events", "Hello WASM", (...args) =>
    console.log(args)
  );
});
