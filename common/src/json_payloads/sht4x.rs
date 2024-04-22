use defmt::Formatter;

/// Payload that can be serialized to JSON.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Sht4xData {
    pub temperature_milli_celsius: i32,
    pub humidity_milli_percent: i32,
}

impl defmt::Format for Sht4xData {
    fn format(&self, fmt: Formatter) {
        defmt::write!(
            fmt,
            "Sht4xData {{ temperature: {} mC, humidity: {} m% }}",
            self.temperature_milli_celsius,
            self.humidity_milli_percent
        )
    }
}
