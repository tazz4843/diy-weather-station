use crate::prometheus::*;
use crate::types::{Sensor, SensorData};
use axum::routing::{get, post};
use axum::{Json, Router};
use prometheus::{Encoder, TextEncoder};

async fn weather_push_route(Json(inbound): Json<SensorData>) {
    let mut temp_set = false;
    let mut humidity_set = false;
    let mut wind_speed_set = false;

    for sensor in inbound.sensors {
        match sensor {
            Sensor::Temperature(v) => {
                TEMPERATURE.set(v.temperature);
                temp_set = true;
            }
            Sensor::Humidity(v) => {
                HUMIDITY.set(v.humidity);
                humidity_set = true;
            }
            Sensor::Pressure(v) => PRESSURE.set(v.pressure),
            Sensor::WindSpeed(v) => {
                WIND_SPEED.set(v.wind_speed);
                wind_speed_set = true;
            }
            Sensor::WindDirection(v) => WIND_DIRECTION.set(v.wind_direction),
            Sensor::Rainfall(v) => RAINFALL.set(v.rainfall),
            Sensor::Uv(v) => UV.set(v.uv),
            Sensor::Light(v) => {
                LIGHT.set(v.light);
                RAW_LIGHT
                    .with_label_values(&["full_spectrum"])
                    .set(v.raw_light);
                RAW_LIGHT.with_label_values(&["visible"]).set(v.raw_visible);
                RAW_LIGHT.with_label_values(&["ir"]).set(v.raw_ir);
            }
            Sensor::AirQuality(v) => {
                AIR_QUALITY.set(v.aqi);
                TVOC.set(v.tvoc);
                ECO2.set(v.eco2);
                for (i, element_resistance) in v.resistances.into_iter().enumerate() {
                    MOX_RESISTANCES
                        .with_label_values(&[&i.to_string()])
                        .set(element_resistance);
                }
            }
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

    if temp_set && humidity_set {
        // try calculating feels like
        let temperature = TEMPERATURE.get();
        let humidity = HUMIDITY.get();

        let feels_like = if temperature >= 27.0 {
            let temp_pow2 = temperature.powi(2);
            let humidity_pow2 = humidity.powi(2);

            -8.78469475556
                + (1.61139411 * temperature)
                + (2.33854883889 * humidity)
                + (-0.14611605 * temperature * humidity)
                + (-0.012308094 * temp_pow2)
                + (-0.0164248277778 * humidity_pow2)
                + (0.002211732 * temp_pow2 * humidity)
                + (0.00072456 * temperature * humidity_pow2)
                + (-0.000003582 * temp_pow2 * humidity_pow2)
        } else if wind_speed_set && temperature <= 10.0 && WIND_SPEED.get() > 1.3333333 {
            let wind_speed = WIND_SPEED.get();
            // convert to km/h
            let wind_speed = wind_speed * 3.6;
            let wind_speed_pow016 = wind_speed.powf(0.16);

            13.12 + (0.6215 * temperature) - (11.37 * wind_speed_pow016)
                + (0.3965 * temperature * wind_speed_pow016)
        } else {
            temperature
        };

        FEELS_LIKE.set(feels_like);
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
