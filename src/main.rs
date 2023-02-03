use weather_station_server::run;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    run().await
}
