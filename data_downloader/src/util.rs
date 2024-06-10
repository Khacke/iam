use polars::prelude::*;
use tracing::warn;

pub fn calculate_rsi(data: &DataFrame, period: usize) -> Vec<f64> {
    let close_prices = data.column("close").expect("Close prices column not found");

    let (initial_avg_gain, initial_avg_loss) = close_prices.iter()
    .zip(close_prices.iter().skip(1))
    .take(period)
    .map(|(prev, curr)| {
        //TODO: make this prettier
        if let AnyValue::Float64(prev) = prev {
            if let AnyValue::Float64(curr) = curr {
                let diff = curr - prev;
                if diff > 0.0 {
                    (diff, 0.0)
                } 
                else {
                    (0.0, -diff)
                }
            }
            else {
                warn!("Could not cast curr to float");
                (0.0, 0.0)
            }
        } else {
            warn!("Could not cast prev to float");
            (0.0, 0.0)
        }
    })
    .fold((0.0, 0.0),|(gain, loss), (new_gain, new_loss)| {
        (gain + new_gain, loss + new_loss)
    });

    let initial_avg_gain = initial_avg_gain / period as f64;
    let initial_avg_loss = initial_avg_loss / period as f64;

    let mut rsi_values: Vec<f64> = Vec::with_capacity(period);
    let mut avg_gain = initial_avg_gain;
    let mut avg_loss = initial_avg_loss;

    let initial_rs = avg_gain / avg_loss;
    let initial_rsi = 100.0 - (100.0 / (1.0 + initial_rs));
    rsi_values.push(initial_rsi);

    let rsi_iter = close_prices.iter()
    .zip(close_prices.iter().skip(1))
    .skip(period)
    .map(|(prev, curr)| {
        //TODO: make this prettier
        if let AnyValue::Float64(prev) = prev{
            if let AnyValue::Float64(curr) = curr {
                let diff = curr - prev;
                let gain = if diff > 0.0 {diff} else {0.0};
                let loss = if diff < 0.0 {-diff} else {0.0};

                avg_gain = (avg_gain * (period as f64 - 1.0) + gain) / period as f64;
                avg_loss = (avg_loss * (period as f64 - 1.0) + loss) / period as f64;

                let rs = avg_gain / avg_loss;

                100.0 - (100.0 / (1.0 + rs))
            }
            else {
                warn!("Could not cast curr to float");
                50.0 //TODO: ?
            }
        }
        else {
            warn!("Could not cast prev to float");
            50.0 //TODO: ?
        }
    });

    rsi_values.extend(rsi_iter);
    rsi_values
}