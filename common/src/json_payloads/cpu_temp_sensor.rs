#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpuTempData {
    pub temperature_celsius: f32,
}

impl defmt::Format for CpuTempData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CpuTempData {{ temp: {} C }}", self.temperature_celsius)
    }
}
