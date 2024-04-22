#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub influx_url: String,
    pub influx_db: String,

    pub target_url: String,

    pub pws_weather: PwsWeatherConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwsWeatherConfig {
    pub id: String,
    pub password: String,
}
