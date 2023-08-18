let url = new URL("/realtime/messages", window.location.href);
url.protocol = url.protocol.replace("http", "ws");

let ws = new WebSocket(url.href);
ws.onmessage = ev => {
    const json = JSON.parse(ev.data);
    console.log(`Message: ${json}`);
}
