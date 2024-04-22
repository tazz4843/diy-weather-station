#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    weather_station_server::run().await;
}
