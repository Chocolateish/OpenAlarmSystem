console.warn("TESTTEST");

class ConnectionManager {}

class Connection extends WebSocket {
  constructor() {
    super("ws://localhost:9001");

    this.onopen = () => {
      console.log("Connected to the server");
      this.send("Hello from the client!");
    };
    this.onmessage = (event) => {
      console.log(event.data);
    };
    this.onclose = () => {
      console.log("Connection closed");
    };
  }
}

let testConnection = new Connection();
