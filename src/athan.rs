use crate::config::Config;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

const BASE_URL: &str = "http://api.aladhan.com";
type Timings = HashMap<chrono::NaiveDate, AthanSlice>;

#[derive(Deserialize)]
struct RawAthan {
    code: u32,
    status: String,
    data: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct AthanSlice {
    #[serde(rename = "Fajr")]
    fajr: String,
    #[serde(rename = "Sunrise")]
    sunrise: String,
    #[serde(rename = "Dhuhr")]
    dhuhr: String,
    #[serde(rename = "Asr")]
    asr: String,
    #[serde(rename = "Sunset")]
    sunset: String,
    #[serde(rename = "Maghrib")]
    maghrib: String,
    #[serde(rename = "Isha")]
    isha: String,
    #[serde(rename = "Imsak")]
    imsak: String,
    #[serde(rename = "Midnight")]
    midnight: String,
}

impl fmt::Display for AthanSlice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Fajr: {}\nSunrise: {}\nDhuhr: {}\nAsr: {}\nSunset: {}\nMaghrib: {}\nIsha: {}\nImsak: {}\nMidnight: {}",
            self.fajr,
            self.sunrise,
            self.dhuhr,
            self.asr,
            self.sunset,
            self.maghrib,
            self.isha,
            self.imsak,
            self.midnight
        )
    }
}

impl RawAthan {
    async fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        // let request_url = reqwest::Url::parse_with_params(
        //     &format!("{}/v1/calendarByCity", BASE_URL),
        //     &[
        //         ("city", &config.city),
        //         ("country", &config.country),
        //         ("method", &config.method.to_string()),
        //     ],
        // )?;
        // let response = reqwest::get(request_url).await?.json::<Self>().await?;
        let contents = std::fs::read_to_string("example.json")?;
        let response = serde_json::from_str::<Self>(&contents)?;
        Ok(response)
    }
}

#[derive(Debug)]
pub struct Athan {
    pub timings: Timings,
}

impl Athan {
    pub async fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        let mut mapping: Timings = HashMap::new();
        let raw_athan = RawAthan::new(config).await?;
        for entry in &raw_athan.data {
            let date = serde_json::from_str::<String>(&entry["date"]["readable"].to_string())
                .map(|ts| chrono::NaiveDate::parse_from_str(&ts, "%d %h %Y"))
                .unwrap()?;
            let timings = serde_json::from_value::<AthanSlice>(entry["timings"].to_owned())?;
            mapping.insert(date, timings);
        }
        Ok(Self { timings: mapping })
    }

    pub fn get(&self, date: &chrono::NaiveDate) -> Result<&AthanSlice, &str> {
        self.timings.get(date).ok_or("Given date does not exist.")
    }

    pub fn today(&self) -> Result<&AthanSlice, &str> {
        self.timings
            .get(&chrono::Utc::today().naive_utc())
            .ok_or("Given date does not exist.")
    }

    pub fn tomorrow(&self) -> Result<&AthanSlice, &str> {
        self.timings
            .get(&chrono::Utc::today().naive_utc().succ())
            .ok_or("Given date does not exist.")
    }

    pub fn next_n(&self, start: &chrono::NaiveDate, n: u8) -> AthanSlice {
        todo!();
    }
}
