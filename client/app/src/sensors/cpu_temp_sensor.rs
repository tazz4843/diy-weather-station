use crate::adc_abstraction::SharedAdc;
use common::json_payloads::CpuTempData;
use embassy_rp::adc::{Channel, Error};
use embassy_rp::peripherals::ADC_TEMP_SENSOR;

pub struct CpuTempSensor<'a> {
    sensor: Channel<'a>,
}

impl CpuTempSensor<'_> {
    pub fn new(cpu_temp: ADC_TEMP_SENSOR) -> Self {
        Self {
            sensor: Channel::new_temp_sensor(cpu_temp),
        }
    }

    pub async fn get_data(&mut self) -> Result<CpuTempData, Error> {
        let raw_temp = SharedAdc.read(&mut self.sensor).await?;
        Ok(CpuTempData {
            temperature_celsius: convert_to_celsius(raw_temp),
        })
    }
}

fn convert_to_celsius(raw_temp: u16) -> f32 {
    // According to chapter 4.9.5. Temperature Sensor in RP2040 datasheet
    let temp = 27.0 - (raw_temp as f32 * 3.3 / 4096.0 - 0.706) / 0.001721;
    let sign = if temp < 0.0 { -1.0 } else { 1.0 };
    let rounded_temp_x10: i16 = ((temp * 10.0) + 0.5 * sign) as i16;
    (rounded_temp_x10 as f32) / 10.0
}
