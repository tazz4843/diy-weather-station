use crate::i2c_abstraction::SharedI2c;
use common::json_payloads::Sht4xData;
use defmt::error;
use embassy_time::Delay;
use sht4x::{Error, Precision, Sht4x};

#[derive(Debug)]
pub enum Sht4xError {
    I2c(embassy_rp::i2c::Error),
    Unknown,
}

impl From<Error<embassy_rp::i2c::Error>> for Sht4xError {
    fn from(e: Error<embassy_rp::i2c::Error>) -> Self {
        match e {
            Error::I2c(e) => Self::I2c(e),
            _ => Self::Unknown,
        }
    }
}

pub struct Sht4xSensor {
    sensor: Sht4x<SharedI2c, Delay>,
}

impl Sht4xSensor {
    pub async fn new() -> Result<Self, Sht4xError> {
        let mut sht4x = Sht4x::new(SharedI2c);
        if let Err(e) = sht4x.serial_number(&mut Delay).await {
            return match e {
                Error::I2c(e) => {
                    error!("I2C error: {:?}", e);
                    Err(Sht4xError::I2c(e))
                }
                _ => {
                    error!("Unknown error");
                    Err(Sht4xError::Unknown)
                }
            };
        }
        Ok(Self { sensor: sht4x })
    }

    pub async fn get_data(&mut self) -> Result<Sht4xData, Sht4xError> {
        let sensor_data = self.sensor.measure(Precision::High, &mut Delay).await?;
        Ok(Sht4xData {
            temperature_milli_celsius: sensor_data.temperature_milli_celsius(),
            humidity_milli_percent: sensor_data.humidity_milli_percent(),
        })
    }
}
