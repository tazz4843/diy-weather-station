use lazy_static::lazy_static;
use prometheus::{
    register_gauge, register_gauge_vec, register_int_gauge, register_int_gauge_vec, Gauge,
    GaugeVec, IntGauge, IntGaugeVec,
};

lazy_static! {
    pub static ref TEMPERATURE: Gauge = register_gauge!("temperature", "Temperature in °C")
        .expect("Failed to register temperature gauge");
    pub static ref FEELS_LIKE: Gauge = register_gauge!("feels_like", "Feels like temperature in °C")
        .expect("Failed to register feels like gauge");
    pub static ref HUMIDITY: Gauge =
        register_gauge!("humidity", "Humidity in %RH").expect("Failed to register humidity gauge");
    pub static ref PRESSURE: Gauge =
        register_gauge!("pressure", "Pressure in hPa").expect("Failed to register pressure gauge");
    pub static ref WIND_SPEED: Gauge = register_gauge!("wind_speed", "Wind speed in m/s")
        .expect("Failed to register wind speed gauge");
    pub static ref WIND_DIRECTION: Gauge =
        register_gauge!("wind_direction", "Wind direction in degrees")
            .expect("Failed to register wind direction gauge");
    pub static ref RAINFALL: Gauge =
        register_gauge!("rainfall", "Rainfall in mm").expect("Failed to register rainfall gauge");
    pub static ref UV: Gauge = register_gauge!("uv_intensity", "Light intensity in mW/cm²")
        .expect("Failed to register UV gauge");
    pub static ref LIGHT: Gauge =
        register_gauge!("light", "Light intensity in lux").expect("Failed to register light gauge");
    pub static ref AIR_QUALITY: Gauge = register_gauge!("air_quality", "VOC index - 0 to 500")
        .expect("Failed to register air quality gauge");
    pub static ref PARTICULATE_MATTER: Gauge =
        register_gauge!("particulate_matter", "PM2.5 in µg/m³")
            .expect("Failed to register particulate matter gauge");
    pub static ref LIGHTNING_COUNT: IntGauge = register_int_gauge!(
        "lightning_count",
        "Number of lightning strikes detected in the past 10 minutes"
    )
    .expect("Failed to register lightning count gauge");
    pub static ref LIGHTNING_DISTANCE: IntGaugeVec = register_int_gauge_vec!(
        "lightning_distance",
        "Distance to strikes, with +/- 1km accuracy",
        &["bound"]
    )
    .expect("Failed to register lightning distance gauges");
    pub static ref MAGNETOMETER: GaugeVec = register_gauge_vec!(
        "magnetometer",
        "Magnetometer readings in µT",
        &["dimension"]
    )
    .expect("Failed to register magnetometer gauges");
    pub static ref NOISE: Gauge =
        register_gauge!("noise", "Noise in dB").expect("Failed to register noise gauge");
    pub static ref WETNESS: IntGauge = register_int_gauge!("wetness", "Is the rain sensor wet?")
        .expect("Failed to register wetness gauge");
    pub static ref RADIATION: Gauge = register_gauge!("radiation", "Radiation in µSv/h")
        .expect("Failed to register radiation gauge");
}
