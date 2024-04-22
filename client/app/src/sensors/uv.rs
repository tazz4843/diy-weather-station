use crate::adc_abstraction::SharedAdc;
use common::json_payloads::UvData;
use embassy_rp::adc::Channel;
use embassy_rp::gpio::Pull;

pub struct UvSensor<'a> {
    pin: Channel<'a>,
}

impl UvSensor<'_> {
    pub fn new(pin_26: embassy_rp::peripherals::PIN_26) -> Self {
        Self {
            pin: Channel::new_pin(pin_26, Pull::None),
        }
    }

    pub async fn get_data(&mut self) -> Result<UvData, embassy_rp::adc::Error> {
        let ticks = SharedAdc.read(&mut self.pin).await?;
        // Integer approximation of (3.3 / 65535) ≈ 1/19855 when multiplied by 1e6
        let voltage_micro_volts = ticks as u64 * 3300000 / 65535;
        // Integer approximation of (1000 / 366) ≈ 3 when multiplied by 1e3
        let power_micro_watts = voltage_micro_volts * 1000 / 366;

        Ok(UvData {
            power_micro_watts,
            voltage_micro_volts,
        })
    }
}
