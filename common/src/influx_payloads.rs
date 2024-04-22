use chrono::{DateTime, Utc};
use influxdb::InfluxDbWriteable;

#[derive(Debug, Copy, Clone, InfluxDbWriteable)]
pub struct SensorData {
    pub time: DateTime<Utc>,

    #[influxdb(tag)]
    pub station: &'static str,

    pub air_quality: u64,
    pub eco2_ppm: u64,
    pub tvoc_ppb: u64,
    pub air_pressure: f64,
    pub temperature_milli_celsius: i64,
    pub humidity_milli_percent: i64,
    pub nano_lux: i64,
    pub light_visible: u64,
    pub light_infrared: u64,
    pub uv_power_micro_watts: u64,
    pub uv_voltage_micro_volts: u64,
    pub noise_db: f64,
    pub noise_zero_to_one: f64,
    pub noise_ticks: u64,
    pub x_nano_tesla: i64,
    pub y_nano_telsa: i64,
    pub z_nano_tesla: i64,
    pub cpu_temperature_celsius: f64,
}

impl From<crate::json_payloads::SensorData> for SensorData {
    fn from(json_payload: crate::json_payloads::SensorData) -> Self {
        Self {
            time: chrono::Utc::now(),
            station: "main",
            air_quality: json_payload.air_quality.air_quality as u64,
            eco2_ppm: json_payload.air_quality.eco2_ppm as u64,
            tvoc_ppb: json_payload.air_quality.tvoc_ppb as u64,
            air_pressure: json_payload.pressure.pressure as f64,
            temperature_milli_celsius: json_payload.temperature.temperature_milli_celsius as i64,
            humidity_milli_percent: json_payload.temperature.humidity_milli_percent as i64,
            nano_lux: json_payload.light.nano_lux,
            light_visible: json_payload.light.visible as u64,
            light_infrared: json_payload.light.infrared as u64,
            uv_power_micro_watts: json_payload.uv.power_micro_watts,
            uv_voltage_micro_volts: json_payload.uv.voltage_micro_volts,
            noise_db: json_payload.noise.noise_db as f64,
            noise_zero_to_one: json_payload.noise.zero_to_one as f64,
            noise_ticks: json_payload.noise.ticks as u64,
            x_nano_tesla: json_payload.magnetic.x_nano_tesla as i64,
            y_nano_telsa: json_payload.magnetic.y_nano_telsa as i64,
            z_nano_tesla: json_payload.magnetic.z_nano_tesla as i64,
            cpu_temperature_celsius: json_payload.cpu_temp.temperature_celsius as f64,
        }
    }
}
