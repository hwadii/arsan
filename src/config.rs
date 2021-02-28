pub struct Config {
    pub city: String,
    pub country: String,
    pub method: u8,
}

impl Config {
    pub fn new(city: &str, country: &str) -> Self {
        Self {
            city: city.to_owned(),
            country: country.to_owned(),
            method: 12,
        }
    }
}
