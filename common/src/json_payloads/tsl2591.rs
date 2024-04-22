use defmt::Formatter;

/// Payload that can be serialized to JSON.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Tsl2591Data {
    pub nano_lux: i64,
    pub visible: u16,
    pub infrared: u16,
}

impl defmt::Format for Tsl2591Data {
    fn format(&self, fmt: Formatter) {
        defmt::write!(
            fmt,
            "Tsl2591Data {{ lux: {} nlx, visible: {}, infrared: {} }}",
            self.nano_lux,
            self.visible,
            self.infrared
        );
    }
}
