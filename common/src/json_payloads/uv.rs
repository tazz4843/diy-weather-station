#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct UvData {
    pub power_micro_watts: u64,
    pub voltage_micro_volts: u64,
}

impl defmt::Format for UvData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UvData {{ power: {} uW/cm^2, voltage: {} uV }}",
            self.power_micro_watts,
            self.voltage_micro_volts
        )
    }
}
