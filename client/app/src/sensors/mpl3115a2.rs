use crate::i2c_abstraction::SharedI2c;
use common::json_payloads::Mpl3115a2Data;
use defmt::info;
use mpl3115a2_hal::{PressureAlt, MPL3115A2};

pub struct Mpl3115a2Sensor {
    sensor: MPL3115A2<SharedI2c>,
}

pub enum Mpl3115a2Error {
    I2c(embassy_rp::i2c::Error),
    InvalidData,
    UnsupportedChip,
}

impl From<mpl3115a2_hal::Error<embassy_rp::i2c::Error>> for Mpl3115a2Error {
    fn from(e: mpl3115a2_hal::Error<embassy_rp::i2c::Error>) -> Self {
        match e {
            mpl3115a2_hal::Error::I2c(e) => Self::I2c(e),
            mpl3115a2_hal::Error::InvalidData => Self::InvalidData,
            mpl3115a2_hal::Error::UnsupportedChip => Self::UnsupportedChip,
        }
    }
}

impl Mpl3115a2Sensor {
    pub async fn new() -> Result<Self, Mpl3115a2Error> {
        let mut sensor = MPL3115A2::new(SharedI2c, PressureAlt::Pressure).await?;
        let device_id = sensor.get_device_id().await?;
        info!("Found MPL3115A2 with device ID {}", device_id);
        sensor.activate().await?;
        sensor.set_oversample_rate(7).await?;
        Ok(Self { sensor })
    }

    pub async fn get_data(&mut self) -> Result<Mpl3115a2Data, Mpl3115a2Error> {
        let pressure = self.sensor.get_pa_reading().await?;
        Ok(Mpl3115a2Data { pressure })
    }
}
