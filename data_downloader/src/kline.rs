use serde::{de::{self, Visitor}, Deserialize};

#[derive(Debug)]
pub struct Kline {
    pub open_time: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub close_time: u64,
    pub quote_asset_volume: f64,
    pub trade_number: usize,
    pub buy_base: f64,
    pub buy_quote: f64,
}

impl<'de> Deserialize<'de> for Kline {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KlineVisitor;

        impl<'de> Visitor<'de> for KlineVisitor {
            type Value = Kline;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a kline entry in the form of an array")
            }

            fn visit_seq<A>(self, mut seq: A) -> std::prelude::v1::Result<Kline, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let open_time = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("missing open_time"))?;
                let open = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("missing open"))?
                    .parse::<f64>()
                    .unwrap();
                let high = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("missing high"))?
                    .parse::<f64>()
                    .unwrap();
                let low = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("missing low"))?
                    .parse::<f64>()
                    .unwrap();
                let close = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("missing close"))?
                    .parse::<f64>()
                    .unwrap();
                let volume = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("missing volume"))?
                    .parse::<f64>()
                    .unwrap();
                let close_time = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("missing close_time"))?;
                let quote_asset_volume = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("missing quote_asset_volume"))?
                    .parse::<f64>()
                    .unwrap();
                let trade_number = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("missing trade_number"))?;
                let buy_base = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("missing buy_base"))?
                    .parse::<f64>()
                    .unwrap();
                let buy_quote = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("missing buy_quote"))?
                    .parse::<f64>()
                    .unwrap();
                let _: Option<String> = seq.next_element::<String>()?; // Ignore the unused field

                Ok(Kline {
                    open_time,
                    open,
                    high,
                    low,
                    close,
                    volume,
                    close_time,
                    quote_asset_volume,
                    trade_number,
                    buy_base,
                    buy_quote,
                })
            }
        }
        deserializer.deserialize_seq(KlineVisitor)
    }
}