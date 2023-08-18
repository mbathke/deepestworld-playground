// TODO: for communication with the websocket server we might have to go through 
// the electron app, so this is currently not working.
let url = new URL("/realtime/messages", window.location.href);
url.protocol = url.protocol.replace("http", "ws");

let ws = new WebSocket(url.href);
ws.onmessage = ev => {
    const json = JSON.parse(ev.data);
    console.log(`Message: ${json}`);
}
