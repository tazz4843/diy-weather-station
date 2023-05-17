use crate::prometheus::*;
use crate::types::{Sensor, SensorData,};
use axum::routing::{get, post};
use axum::{Json, Router};
use prometheus::{Encoder, TextEncoder};

async fn weather_push_route(Json(inbound): Json<SensorData>) {
    for sensor in inbound.sensors {
        match sensor {
            Sensor::Temperature(v) => TEMPERATURE.set(v.temperature),
            Sensor::Humidity(v) => HUMIDITY.set(v.humidity),
            Sensor::Pressure(v) => PRESSURE.set(v.pressure),
            Sensor::WindSpeed(v) => WIND_SPEED.set(v.wind_speed),
            Sensor::WindDirection(v) => WIND_DIRECTION.set(v.wind_direction),
            Sensor::Rainfall(v) => RAINFALL.set(v.rainfall),
            Sensor::Uv(v) => UV.set(v.uv),
            Sensor::Light(v) => LIGHT.set(v.light),
            Sensor::AirQuality(v) => AIR_QUALITY.set(v.air_quality),
            Sensor::ParticulateMatter(v) => PARTICULATE_MATTER.set(v.pm_2_5),
            Sensor::Noise(v) => NOISE.set(v.noise),
            Sensor::Wetness(v) => WETNESS.set(v.is_wet as _),
            Sensor::Radiation(v) => RADIATION.set(v.radiation_sv),
            Sensor::Lightning(v) => {
                LIGHTNING_COUNT.set(v.strikes);
                LIGHTNING_DISTANCE
                    .with_label_values(&["closest"])
                    .set(v.closest_distance.unwrap_or(-1));
                LIGHTNING_DISTANCE
                    .with_label_values(&["average"])
                    .set(v.average_distance.unwrap_or(-1));
                LIGHTNING_DISTANCE
                    .with_label_values(&["farthest"])
                    .set(v.farthest_distance.unwrap_or(-1));
            }
            Sensor::Magnetometer(v) => {
                MAGNETOMETER.with_label_values(&["x"]).set(v.x);
                MAGNETOMETER.with_label_values(&["y"]).set(v.y);
                MAGNETOMETER.with_label_values(&["z"]).set(v.z);
            }
        }
    }
}

async fn prometheus_route() -> Vec<u8> {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();

    let metrics = prometheus::gather();
    if let Err(e) = encoder.encode(&metrics, &mut buffer) {
        error!("failed to encode metrics: {}", e);
        vec![]
    } else {
        buffer
    }
}

fn build_router() -> Router {
    Router::new()
        .route("/push", post(weather_push_route))
        .route("/metrics", get(prometheus_route))
}

pub async fn run() {
    let app = build_router();
    let addr = ([0, 0, 0, 0], 2950).into();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
