#[macro_use]
extern crate tracing;

use common::config::ServerConfig;
use common::json_payloads::SensorData;
use influxdb::{Client as InfluxClient, InfluxDbWriteable};
use reqwest::{Client as ReqwestClient, ClientBuilder, Error as ReqwestError, Response, Url};
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::time::MissedTickBehavior;

pub async fn run() {
    let config_file = std::fs::read_to_string("config.toml").expect("config file should exist");
    let config: Arc<ServerConfig> =
        Arc::new(toml::from_str(&config_file).expect("config should parse"));

    let target_url =
        Url::from_str(&format!("http://{}/data", config.target_url)).expect("URL should parse");

    let station_http_client = ClientBuilder::new()
        .pool_idle_timeout(Some(Duration::from_nanos(1)))
        .pool_max_idle_per_host(0)
        .timeout(Duration::from_secs(3))
        .build()
        .expect("client should build");
    let influx_client = InfluxClient::new(&config.influx_url, &config.influx_db);
    let api_clients = ClientBuilder::new().build().expect("client should build");

    let (ctrl_c_tx, mut ctrl_c_rx) = tokio::sync::oneshot::channel::<()>();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for ctrl-c");
        ctrl_c_tx
            .send(())
            .expect("at least one ctrl+c receiver should still be alive");
    });

    // align the current timestamp to the next 5 second interval (ie the nearest timestamp that is
    // divisible by 5)
    let ts = SystemTime::now();
    let ts = ts
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time should be after epoch");
    let ts = ts.as_nanos();
    // how long until the nearest 5 second interval?
    let ts = 5_000_000_000 - (ts % 5_000_000_000);
    // and sleep until then
    let ts = Duration::from_nanos(ts as u64);
    info!("Sleeping for {:?}", ts);
    tokio::time::sleep(ts).await;

    let mut last_pws_update = SystemTime::UNIX_EPOCH;

    let mut interval = tokio::time::interval(Duration::from_secs(5));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            _ = interval.tick() => {},
            _ = &mut ctrl_c_rx => {
                info!("Received Ctrl-C, exiting");
                break;
            }
        }

        // get client data
        let request = match station_http_client.get(target_url.clone()).build() {
            Ok(r) => r,
            Err(e) => {
                error!("Request build failed: {}", e);
                continue;
            }
        };

        let response = match tokio::time::timeout(
            Duration::from_millis(3500),
            station_http_client.execute(request),
        )
        .await
        {
            Ok(Ok(r)) => r,
            Ok(Err(e)) => {
                error!("Request failed: {}", e);
                continue;
            }
            Err(_) => {
                error!("Request timed out");
                continue;
            }
        };

        let payload = match response.json::<SensorData>().await {
            Ok(p) => p,
            Err(e) => {
                error!("JSON error: {}", e);
                continue;
            }
        };

        if last_pws_update
            .elapsed()
            .expect("system clock should be monotonic")
            > Duration::from_secs(60)
        {
            let api_client2 = api_clients.clone();
            let p2 = payload;
            let cfg = Arc::clone(&config);
            match execute_pws_request(&api_client2, p2, cfg).await {
                Ok(res) if res.status().is_client_error() || res.status().is_server_error() => {
                    let status = res.status();
                    let body = res.text().await;
                    error!(
                        "PWSWeather request failed: code {}, body {:?}",
                        status, body
                    );
                }
                Ok(res) => {
                    info!("PWSWeather request successful: {}", res.status());
                    last_pws_update = SystemTime::now();
                }
                Err(e) => {
                    error!("PWSWeather request failed: {}", e);
                }
            }
        }

        let if_client = influx_client.clone();
        tokio::spawn(async move {
            // convert to influxdb payload
            let influx_payload = common::influx_payloads::SensorData::from(payload);
            debug!("Influx payload: {:?}", influx_payload);

            // send to influxdb
            match if_client.query(influx_payload.into_query("weather")).await {
                Ok(r) => debug!("InfluxDB write successful: {}", r),
                Err(e) => error!("InfluxDB error: {}", e),
            }
        });
    }
}

async fn execute_pws_request(
    client: &ReqwestClient,
    sensor_data: SensorData,
    config: Arc<ServerConfig>,
) -> Result<Response, ReqwestError> {
    let mut url = Url::from_str("https://pwsupdate.pwsweather.com/api/v1/submitwx")
        .expect("URL should parse");
    {
        let mut qp = url.query_pairs_mut();
        qp.extend_pairs(&[
            ("ID", config.pws_weather.id.as_str()),
            ("PASSWORD", config.pws_weather.password.as_str()),
            ("softwaretype", "CustomClient0.0.0"),
            ("action", "updateraw"),
        ]);
        let current_date = chrono::Utc::now();
        // PWSWeather wants format "YYYY-MM-DD HH:MM:SS"
        qp.append_pair(
            "dateutc",
            &current_date.format("%Y-%m-%d %H:%M:%S").to_string(),
        );

        // finally we add sensor data
        qp.append_pair(
            "tempf",
            &temp_deg_c_to_f(sensor_data.temperature.temperature_milli_celsius as f64 / 1000.0)
                .to_string(),
        );
        qp.append_pair(
            "baromin",
            &baro_pa_to_inhg(sensor_data.pressure.pressure as _).to_string(),
        );
        qp.append_pair(
            "humidity",
            &(sensor_data.temperature.humidity_milli_percent as f32 / 1000.0).to_string(),
        );
        qp.append_pair(
            "UV",
            &(sensor_data.uv.power_micro_watts as f32 / 25_000.0).to_string(),
        );
        qp.finish();
    }

    client
        .execute(client.post(url).build().expect("request should build"))
        .await
}

fn temp_deg_c_to_f(temp: f64) -> f64 {
    temp * 9.0 / 5.0 + 32.0
}
fn baro_pa_to_inhg(pressure: f64) -> f64 {
    pressure * 0.00029529983071445
}

/// Given pressure in Pa, the current temperature in Kelvin,
/// and an altitude in meters, return the equivalent pressure at sea level
fn convert_pressure_to_sea_level(pressure: f64, temperature: f64, altitude: f64) -> f64 {
    /// Earth gravity in m/s^2
    const GRAVITY: f64 = 9.80665;
    /// Molar mass of Earth's air in kg/mol
    const MOLAR_MASS: f64 = 0.0289644;
    /// Ideal gas constant in J/(mol*K)
    const GAS_CONSTANT: f64 = 8.314462618;

    pressure / (-((MOLAR_MASS * GRAVITY * altitude) / (GAS_CONSTANT * temperature))).exp()
}
