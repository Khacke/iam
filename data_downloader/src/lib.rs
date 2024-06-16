// IAMbot Data Downloader 
// Created by Jenei Andras 2024

#![crate_name = "data_downloader"]
use polars::{df, io::parquet::ParquetWriter};
use polars::prelude::*;
use reqwest::{Client, Url};

mod error;
mod interval;
mod kline;
mod util;

use error::{Error, Result};
use interval::Interval;
use kline::Kline;
use tracing::{debug, info};

pub const BINANCE_KLINE_API: &str = "https://api.binance.com/api/v3/klines";

pub struct Requester {
    client: Client,
}

impl Default for Requester {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl Requester {
    pub async fn get_historical_data(
        &self,
        symbol: &str,
        interval: &str,
        start_time: usize,
        end_time: usize,
        output_path: &str,
    ) {
        // TODO: multithreading?
        info!("Beginning to download historical data:\n\tsymbol: {symbol}\n\tinterval: {interval}\n\tstart_time: {start_time}\n\tend_time: {end_time}\n\toutput_path: {output_path}");
        
        let limit = 1000;

        let interval_sec = interval.parse::<Interval>().unwrap() as usize;
        let interval_ms = interval_sec * 1000;

        let chunks = (start_time..end_time)
            .step_by(limit*interval_ms)
            .map(|current_start_time| {
                let current_end_time = std::cmp::min(current_start_time + limit * interval_ms, end_time);
                self.download_chunk(symbol, interval, current_start_time, current_end_time, limit)
            })
            .collect::<Vec<_>>();


        let data: Vec<Kline> = futures::future::join_all(chunks)
            .await
            .into_iter()
            .flatten()
            .collect();

        self.save_to_file(output_path, data).unwrap();
    }

    pub async fn connect_to_ws() {
        // TODO: implement
    }

    async fn download_chunk(
        &self,
        symbol: &str,
        interval: &str,
        start_time: usize,
        end_time: usize,
        limit: usize,
    ) -> Vec<Kline> {
        info!("Downloading chunk:\n\tsymbol: {symbol}\n\tinterval: {interval}\n\tstart_time: {start_time}\n\tend_time: {end_time}\n\t");

        let url = Url::parse_with_params(
            BINANCE_KLINE_API,
            &[
                ("symbol", symbol),
                ("interval", interval),
                ("startTime", start_time.to_string().as_str()),
                ("endTime", end_time.to_string().as_str()),
                ("limit", limit.to_string().as_str()),
            ],
        )
        .unwrap();
        debug!("URL: {url}");
        let resp = self.client.get(url)
            .send()
            .await.unwrap()
            .text()
            .await.unwrap();
        serde_json::from_str(&resp).unwrap()
    }

    fn save_to_file(&self, output_path: &str, data: Vec<Kline>) -> Result<()> {
        info!("Saving file to {output_path}");
        let mut df = df!(
            "open_time" => data.iter().map(|k| k.open_time).collect::<Vec<u64>>(),
            "open" => data.iter().map(|k| k.open).collect::<Vec<f64>>(),
            "high" => data.iter().map(|k| k.high).collect::<Vec<f64>>(),
            "low" => data.iter().map(|k| k.low).collect::<Vec<f64>>(),
            "close" => data.iter().map(|k| k.close).collect::<Vec<f64>>(),
            "volume" => data.iter().map(|k| k.volume).collect::<Vec<f64>>(),
            "close_time" => data.iter().map(|k| k.close_time).collect::<Vec<u64>>(),
            "quote_asset_volume" => data.iter().map(|k| k.quote_asset_volume).collect::<Vec<f64>>(),
            "trade_number" => data.iter().map(|k| k.trade_number as u64).collect::<Vec<u64>>(),
            "buy_base" => data.iter().map(|k| k.buy_base).collect::<Vec<f64>>(),
            "buy_quote" => data.iter().map(|k| k.buy_quote).collect::<Vec<f64>>(),
        )
        .unwrap();
        // TODO: not the task of the data_downloader, implement elsewhere
        let rsi_values = util::calculate_rsi(&df, 14);
        let mut extended_rsi = vec![None; df.height() - rsi_values.len()];
        extended_rsi.extend(rsi_values.iter().cloned().map(Some));
        df.with_column(Series::new("RSI[14]", &extended_rsi)).unwrap();
        //TODO: error handling
        let mut file = std::fs::File::create(output_path).expect("Could not create file");
        ParquetWriter::new(&mut file)
            .finish(&mut df)
            .expect("Could not write to file");
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    // cant test this 
}