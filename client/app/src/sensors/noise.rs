use crate::adc_abstraction::SharedAdc;
use common::json_payloads::NoiseData;
use embassy_rp::adc::Channel;
use embassy_rp::gpio::Pull;

pub struct NoiseSensor<'a> {
    pin: Channel<'a>,
}

impl NoiseSensor<'_> {
    pub fn new(pin_28: embassy_rp::peripherals::PIN_28) -> Self {
        Self {
            pin: Channel::new_pin(pin_28, Pull::Down),
        }
    }

    pub async fn get_data(&mut self) -> Result<NoiseData, embassy_rp::adc::Error> {
        let ticks = SharedAdc.read(&mut self.pin).await?;
        let zero_to_one = (ticks as f32 - 769.0) / 125.0;
        let noise_db = 20.0 * libm::log10f(libm::fabsf(zero_to_one));

        Ok(NoiseData {
            noise_db,
            zero_to_one,
            ticks,
        })
    }
}
