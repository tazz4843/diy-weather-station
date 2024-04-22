#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct NoiseData {
    pub noise_db: f32,
    pub zero_to_one: f32,
    pub ticks: u16,
}

impl defmt::Format for NoiseData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NoiseData {{ noise: {} dB, ticks: {}, zero_to_one: {} }}",
            self.noise_db,
            self.ticks,
            self.zero_to_one
        )
    }
}
