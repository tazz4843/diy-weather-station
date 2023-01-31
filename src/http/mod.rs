use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::response::Response;
use axum::routing::get;
use axum::Router;

use crate::types::WebSocketInbound;

pub async fn websocket_route(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(websocket_handler)
}

async fn websocket_handler(mut ws: WebSocket) {
    while let Some(msg) = ws.recv().await {
        let msg = match msg {
            Ok(msg) => msg.into_data(),
            Err(e) => {
                error!("websocket error: {}", e);
                break;
            }
        };
        let inbound: WebSocketInbound = match rmp_serde::from_slice(&msg) {
            Ok(v) => v,
            Err(e) => {
                error!("Error deserializing MessagePack: {e}");
                return;
            }
        };
        let WebSocketInbound::IncomingData(inbound) = inbound;
    }
}

pub fn build_router() -> Router {
    Router::new().route("/ws", get(websocket_route))
}
