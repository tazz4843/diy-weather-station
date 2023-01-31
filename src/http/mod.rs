use crate::prometheus::*;
use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::response::Response;
use axum::routing::get;
use axum::Router;

use crate::types::{Sensor, WebSocketInbound};

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
        for sensor in inbound.sensors {
            match sensor {
                Sensor::Temperature(v) => TEMPERATURE.set(v.temperature),
                Sensor::Humidity(v) => HUMIDITY.set(v.humidity),
                Sensor::Pressure(v) => PRESSURE.set(v.pressure),
                Sensor::WindSpeed(v) => WIND_SPEED.set(v.wind_speed),
                Sensor::WindDirection(v) => WIND_DIRECTION.set(v.wind_direction),
                Sensor::Rainfall(v) => RAINFALL.set(v.rainfall),
                Sensor::Uv(v) => UV.set(v.uv),
                Sensor::Light(v) => UV.set(v.light),
                Sensor::AirQuality(v) => AIR_QUALITY.set(v.air_quality),
                Sensor::ParticulateMatter(v) => PARTICULATE_MATTER.set(v.pm_2_5),
                Sensor::Noise(v) => NOISE.set(v.noise),
                Sensor::Wetness(v) => todo!(),
                Sensor::Radiation(v) => todo!(),
                Sensor::Lightning(v) => todo!(),
                Sensor::Magnetometer(v) => todo!(),
            }
        }
    }
}

pub fn build_router() -> Router {
    Router::new().route("/ws", get(websocket_route))
}
