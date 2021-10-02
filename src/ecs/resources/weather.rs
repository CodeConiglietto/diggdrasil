use crate::prelude::*;

#[derive(Default)]
pub struct WeatherResource {
    pub current_weather: WeatherStatus,
    pub wind_direction: Direction,
}

impl WeatherResource {
    pub fn new() -> Self {
        Self::default()
    }
}