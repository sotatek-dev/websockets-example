const WebSocketServer = require("ws");

const wss = new WebSocketServer.Server({ port: 8888 });

wss.on("connection", (ws) => {
  console.log("new client connected");

  ws.on("message", (data) => {
    console.log(`Client has sent us: ${data}`);
    ws.send(data);
  });

  ws.on("close", () => {
    console.log("the client has disconnected");
  });
  ws.onerror = function () {
    console.log("Some Error occurred");
  };
});

console.log("The WebSocket server is running on port 8888");
