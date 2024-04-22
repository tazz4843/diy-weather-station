use defmt::{write, Format, Formatter};
use serde_derive::Serialize;

mod cpu_temp_sensor;
mod ens160;
mod lis2mdl;
mod mpl3115a2;
mod noise;
mod sht4x;
mod tsl2591;
mod uv;

pub use cpu_temp_sensor::CpuTempData;
pub use ens160::Ens160Data;
pub use lis2mdl::Lis2mdlData;
pub use mpl3115a2::Mpl3115a2Data;
pub use noise::NoiseData;
pub use sht4x::Sht4xData;
pub use tsl2591::Tsl2591Data;
pub use uv::UvData;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SensorData {
	pub air_quality: Ens160Data,
	pub pressure:    Mpl3115a2Data,
	pub temperature: Sht4xData,
	pub light:       Tsl2591Data,
	pub uv:          UvData,
	pub noise:       NoiseData,
	pub magnetic:    Lis2mdlData,
	pub cpu_temp:    CpuTempData,
}

impl Format for SensorData {
	fn format(&self, fmt: Formatter) {
		write!(fmt, "SensorData {{");
		write!(fmt, "\n\tair_quality: {:?}, ", self.air_quality);
		write!(fmt, "\n\tpressure: {:?}, ", self.pressure);
		write!(fmt, "\n\ttemperature: {:?}, ", self.temperature);
		write!(fmt, "\n\tlight: {:?}, ", self.light);
		write!(fmt, "\n\tmagnetic: {:?}, ", self.magnetic);
		write!(fmt, "\n\tuv: {:?}", self.uv);
		write!(fmt, "\n\tnoise: {:?}", self.noise);
		write!(fmt, "\n\tcpu_temp: {:?}", self.cpu_temp);
		write!(fmt, "\n}}");
	}
}
