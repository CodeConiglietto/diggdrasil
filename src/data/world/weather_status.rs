pub enum WeatherStatus {
    Clear,
    Raining,
    Snowing,
    Sandstorm,
}

impl Default for WeatherStatus {
    fn default() -> WeatherStatus {
        WeatherStatus::Clear
    }
}
