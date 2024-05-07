use std::str::FromStr;
use crate::error::Error;

pub enum Interval {
    OneMinute = 3_600,
    ThreeMinute = 10_800,
    FiveMinute = 18_000,
    FifteenMinute = 54_000,
    ThirtyMinute = 108_000,
    OneHour = 216_000,
    TwoHour = 432_000,
    FourHour = 864_000,
    SixHour = 1_296_000,
    EightHour = 1_728_000,
    TwelweHour = 2_592_000,
    OneDay = 5_184_000,
    ThreeDay = 15_553_000,
    OneWeek = 36_288_000,
    OneMonth = 155_520_000 // if 1 month = 30 days?
}

impl FromStr for Interval {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1m" =>  Ok(Interval::OneMinute),
            "3m" =>  Ok(Interval::ThreeMinute),
            "5m" =>  Ok(Interval::FiveMinute),
            "15m" => Ok(Interval::FifteenMinute),
            "30m" => Ok(Interval::ThirtyMinute),
            "1h" =>  Ok(Interval::OneHour),
            "2h" =>  Ok(Interval::TwoHour),
            "4h" =>  Ok(Interval::FourHour),
            "6h" =>  Ok(Interval::SixHour),
            "8h" =>  Ok(Interval::EightHour),
            "12h" => Ok(Interval::TwelweHour),
            "1d" =>  Ok(Interval::OneDay),
            "3d" =>  Ok(Interval::ThreeDay),
            "1w" =>  Ok(Interval::OneWeek),
            "1M" =>  Ok(Interval::OneMonth),
            // if none of the patterns match the default is 1h
            _ =>     Err(Error::ParseIntervalError)
        }
    }
}