use defmt::Formatter;

/// Payload that can be serialized to JSON.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Lis2mdlData {
    pub x_nano_tesla: i32,
    pub y_nano_telsa: i32,
    pub z_nano_tesla: i32,
}

impl defmt::Format for Lis2mdlData {
    fn format(&self, fmt: Formatter) {
        defmt::write!(
            fmt,
            "Lis2mdlData {{ x: {} uT, y: {} uT, z: {} uT }}",
            self.x_nano_tesla,
            self.y_nano_telsa,
            self.z_nano_tesla
        );
    }
}
