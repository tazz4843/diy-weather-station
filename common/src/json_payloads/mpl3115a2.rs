use defmt::Formatter;

/// Payload that can be serialized to JSON.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Mpl3115a2Data {
    pub pressure: f32,
}

impl defmt::Format for Mpl3115a2Data {
    fn format(&self, fmt: Formatter) {
        defmt::write!(fmt, "Mpl3115a2Data {{ pressure: {} Pa }}", self.pressure);
    }
}
