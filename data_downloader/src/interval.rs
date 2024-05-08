use std::str::FromStr;
use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum Interval {
    OneMinute =       60,
    ThreeMinute =    180,
    FiveMinute =     300,
    FifteenMinute =  900,
    ThirtyMinute = 1_800,
    OneHour =      3_600,
    TwoHour =      7_200,
    FourHour =    14_400,
    SixHour =     21_600,
    EightHour =   28_800,
    TwelweHour =  43_200,
    OneDay =      86_400,
    ThreeDay =   259_200,
    OneWeek =    604_800,
    OneMonth = 2_592_000// if 1 month = 30 days?
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_parsing() {
        let om = "1m".parse::<Interval>().unwrap();
        let tm = "3m".parse::<Interval>().unwrap();
        let fm = "5m".parse::<Interval>().unwrap();
        let ftm = "15m".parse::<Interval>().unwrap();
        let ttm = "30m".parse::<Interval>().unwrap();
        let oh = "1h".parse::<Interval>().unwrap();
        let th = "2h".parse::<Interval>().unwrap();
        let fh = "4h".parse::<Interval>().unwrap();
        let sh = "6h".parse::<Interval>().unwrap();
        let eh = "8h".parse::<Interval>().unwrap();
        let twh = "12h".parse::<Interval>().unwrap();
        let od = "1d".parse::<Interval>().unwrap();
        let td = "3d".parse::<Interval>().unwrap();
        let ow = "1w".parse::<Interval>().unwrap();
        let omo = "1M".parse::<Interval>().unwrap();
        let er = "wrong value".parse::<Interval>();
        assert_eq!(
            [om, tm, fm, ftm, ttm, oh, th, fh, sh, eh, twh, od, td, ow, omo],
            [
                Interval::OneMinute,
                Interval::ThreeMinute,
                Interval::FiveMinute,
                Interval::FifteenMinute,
                Interval::ThirtyMinute,
                Interval::OneHour,
                Interval::TwoHour,
                Interval::FourHour,
                Interval::SixHour,
                Interval::EightHour,
                Interval::TwelweHour,
                Interval::OneDay,
                Interval::ThreeDay,
                Interval::OneWeek,
                Interval::OneMonth
            ]
        );

        assert!(er.is_err());
    }


    #[test]
    fn interval_value() {
        let om = "1m".parse::<Interval>().unwrap() as u64;
        let tm = "3m".parse::<Interval>().unwrap() as u64;
        let fm = "5m".parse::<Interval>().unwrap() as u64;
        let ftm = "15m".parse::<Interval>().unwrap() as u64;
        let ttm = "30m".parse::<Interval>().unwrap() as u64;
        let oh = "1h".parse::<Interval>().unwrap() as u64;
        let th = "2h".parse::<Interval>().unwrap() as u64;
        let fh = "4h".parse::<Interval>().unwrap() as u64;
        let sh = "6h".parse::<Interval>().unwrap() as u64;
        let eh = "8h".parse::<Interval>().unwrap() as u64;
        let twh = "12h".parse::<Interval>().unwrap() as u64;
        let od = "1d".parse::<Interval>().unwrap() as u64;
        let td = "3d".parse::<Interval>().unwrap() as u64;
        let ow = "1w".parse::<Interval>().unwrap() as u64;
        let omo = "1M".parse::<Interval>().unwrap() as u64;
        assert_eq!(
            [om, tm, fm, ftm, ttm, oh, th, fh, sh, eh, twh, od, td, ow, omo],
            [
                60,    // OneMinute
                180,   // ThreeMinute
                300,   // FiveMinute
                900,   // FifteenMinute
                1_800, // ThirtyMinute
                3_600, // OneHour
                7_200, // TwoHour
                14_400, // FourHour
                21_600, // SixHour
                28_800, // EightHour
                43_200, // TwelveHour
                86_400, // OneDay
                259_200, // ThreeDay
                604_800, // OneWeek
                2_592_000 // OneMonth
            ]
        );
    }
}