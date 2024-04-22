use defmt::Formatter;

/// Payload that can be serialized to JSON.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Ens160Data {
    pub air_quality: u8,
    pub eco2_ppm: u16,
    pub tvoc_ppb: u16,
}

impl defmt::Format for Ens160Data {
    fn format(&self, fmt: Formatter) {
        defmt::write!(
            fmt,
            "Ens160Data {{ air_quality: {}, eco2: {} ppm, tvoc: {} ppb }}",
            self.air_quality,
            self.eco2_ppm,
            self.tvoc_ppb
        )
    }
}
