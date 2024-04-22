use crate::i2c_abstraction::SharedI2c;
use common::json_payloads::Ens160Data;
use defmt::info;
use ens160::Ens160;

pub struct Ens160Sensor {
    sensor: Ens160<SharedI2c>,
}

impl Ens160Sensor {
    pub async fn new() -> Result<Self, embassy_rp::i2c::Error> {
        let mut sensor = Ens160::new(SharedI2c, 0x53);
        let (major, minor, patch) = sensor.firmware_version().await?;
        let part_id = sensor.part_id().await?;
        info!(
            "Found ENS160 with part ID {}, and firmware version {}.{}.{}",
            part_id, major, minor, patch
        );
        sensor.operational().await?;
        Ok(Self { sensor })
    }

    pub async fn get_data(&mut self) -> Result<Ens160Data, embassy_rp::i2c::Error> {
        let air_quality = self.sensor.airquality_index().await? as u8;
        let eco2_ppm = *self.sensor.eco2().await?;
        let tvoc_ppb = self.sensor.tvoc().await?;
        Ok(Ens160Data {
            air_quality,
            eco2_ppm,
            tvoc_ppb,
        })
    }
}
