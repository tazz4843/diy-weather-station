use common::json_payloads::Tsl2591Data;
use defmt::{debug, info};
use embassy_time::Delay;
use tsl2591::{check_overflow, AdafruitPythonLuxConverter, Error, Gain, IntegrationTime, Tsl2591};

use crate::i2c_abstraction::SharedI2c;

pub enum Tsl2591Error {
	I2c(embassy_rp::i2c::Error),
	IdMismatch(u8),
	SignalOverflow,
	InfraredOverflow,
	/// Heuristic was unable to find a valid gain/integration time combination
	/// for the current light level
	TotalOverflow,
}

impl From<Error<embassy_rp::i2c::Error>> for Tsl2591Error {
	fn from(e: Error<embassy_rp::i2c::Error>) -> Self {
		match e {
			Error::I2c(e) => Self::I2c(e),
			Error::IdMismatch(id) => Self::IdMismatch(id),
			Error::SignalOverflow => Self::SignalOverflow,
			Error::InfraredOverflow => Self::InfraredOverflow,
		}
	}
}

pub struct Tsl2591Sensor {
	sensor: Tsl2591<SharedI2c, Delay>,
}

impl Tsl2591Sensor {
	pub async fn new() -> Result<Self, Tsl2591Error> {
		let mut sensor = Tsl2591::new(SharedI2c).await?;
		info!("Found TSL2591 sensor");
		sensor.set_timing(Some(IntegrationTime::_200MS)).await?;
		Ok(Self { sensor })
	}

	pub async fn get_data(&mut self) -> Result<Tsl2591Data, Tsl2591Error> {
		self.sensor.enable().await?;

		// whee fun overflow checks
		for gain in [Gain::Max, Gain::High, Gain::Med, Gain::Low] {
			self.sensor.set_gain(Some(gain)).await?;
			debug!(
				"TSL2591 gain: {:?}",
				match gain {
					Gain::Low => "Low",
					Gain::Med => "Med",
					Gain::High => "High",
					Gain::Max => "Max",
				}
			);

			match self.sensor.get_channel_data(&mut Delay).await {
				Ok((visible, infrared)) => {
					debug!(
						"TSL2591 data: (visible: {}, infrared: {})",
						visible, infrared
					);

					if check_overflow(IntegrationTime::_200MS, visible, infrared) {
						debug!("TSL2591 overflow, trying again");
						continue;
					}
					let nano_lux = match self
						.sensor
						.calculate_nano_lux::<AdafruitPythonLuxConverter>(visible, infrared)
					{
						Ok(nlux) if nlux > 0 => nlux,
						_ => continue,
					};

					return Ok(Tsl2591Data {
						nano_lux,
						visible,
						infrared,
					});
				}
				Err(Error::I2c(e)) => return Err(Tsl2591Error::I2c(e)),
				Err(Error::IdMismatch(id)) => return Err(Tsl2591Error::IdMismatch(id)),
				Err(Error::SignalOverflow) => {
					debug!("TSL2591 signal overflow, trying again");
				}
				Err(Error::InfraredOverflow) => {
					debug!("TSL2591 infrared overflow, trying again");
				}
			}
		}

		Err(Tsl2591Error::TotalOverflow)
	}
}
