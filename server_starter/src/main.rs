use std::time::Duration;

use axum::{
    extract::ws::Message, extract::ws::WebSocket, extract::State, extract::WebSocketUpgrade,
    response::IntoResponse, routing::get, Router, Server,
};
use tokio::sync::broadcast;

type Snapshot = String;

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<Snapshot>,
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<Snapshot>(1);

    let app_state = AppState { tx: tx.clone() };

    let router = Router::new()
        .route("/realtime/messages", get(realtime_messages_get))
        .with_state(app_state.clone());

    tokio::task::spawn_blocking(move || {
        let mut i = 0;
        loop {
            let msg = format!("Hello {}", i);
            let _ = tx.send(msg.to_string());

            std::thread::sleep(Duration::from_millis(250));
            i += 1;
        }
    });

    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Listening on {}", addr);

    server.await.unwrap();
}

async fn realtime_messages_get(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|ws: WebSocket| async { realtime_messages_stream(state, ws).await })
}

async fn realtime_messages_stream(app_state: AppState, mut ws: WebSocket) {
    let mut rx = app_state.tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        let payload = serde_json::to_string(&msg).unwrap();
        ws.send(Message::Text(payload)).await.unwrap();
    }
}
