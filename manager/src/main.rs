use data_downloader::Requester;
use tracing_subscriber::{fmt, EnvFilter, prelude::*};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let requester = Requester::default();
    requester.get_historical_data("BTCUSDT", "1h", 1704110400000, 1715177886000, "BTCUSDT20240101.parquet").await;
}
