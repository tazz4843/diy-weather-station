use crate::i2c_abstraction::SharedI2c;
use common::json_payloads::Lis2mdlData;
use defmt::info;
use embassy_time::Delay;
use lis2mdl::{Error, Lis2mdl, OutputDataRate};

#[derive(Debug)]
pub enum Lis2mdlError {
    I2c(embassy_rp::i2c::Error),
    InvalidData,
}

impl From<Error<embassy_rp::i2c::Error>> for Lis2mdlError {
    fn from(e: Error<embassy_rp::i2c::Error>) -> Self {
        match e {
            Error::I2c(e) => Self::I2c(e),
            Error::InvalidData => Self::InvalidData,
        }
    }
}

pub struct Lis2mdlSensor {
    sensor: Lis2mdl<SharedI2c, Delay>,
}

impl Lis2mdlSensor {
    pub async fn new() -> Result<Self, Lis2mdlError> {
        let mut lis2mdl = Lis2mdl::new(SharedI2c);
        let who_am_i = lis2mdl.who_am_i().await?;
        info!("Found LIS2MDL: 0x{:x}", who_am_i);
        lis2mdl.set_update_rate(OutputDataRate::_10Hz).await?;
        Ok(Self { sensor: lis2mdl })
    }

    pub async fn get_data(&mut self) -> Result<Lis2mdlData, Lis2mdlError> {
        self.sensor.take_single_measurement().await?;
        let output = self.sensor.get_output().await?;
        Ok(Lis2mdlData {
            x_nano_tesla: output.x.0,
            y_nano_telsa: output.y.0,
            z_nano_tesla: output.z.0,
        })
    }
}
