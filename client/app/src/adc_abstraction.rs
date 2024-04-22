use embassy_rp::adc::{Adc, AdcSample, Async, Channel, Sample};
use embassy_rp::dma::Channel as DmaChannel;
use embassy_rp::Peripheral;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use once_cell::sync::OnceCell;

static ADC: OnceCell<Mutex<CriticalSectionRawMutex, Adc<Async>>> = OnceCell::new();

pub fn init_adc(adc: Adc<'static, Async>) {
    ADC.set(Mutex::new(adc)).unwrap_or_else(|_| {
        panic!("ADC already initialized, this should never happen (adc is a singleton)")
    });
}

pub struct SharedAdc;

#[allow(dead_code)]
impl SharedAdc {
    pub async fn read(&mut self, ch: &mut Channel<'_>) -> Result<u16, embassy_rp::adc::Error> {
        ADC.get()
            .expect("create an adc before using it")
            .lock()
            .await
            .read(ch)
            .await
    }

    pub async fn read_many<S: AdcSample>(
        &mut self,
        ch: &mut Channel<'_>,
        buf: &mut [S],
        div: u16,
        dma: impl Peripheral<P = impl DmaChannel>,
    ) -> Result<(), embassy_rp::adc::Error> {
        ADC.get()
            .expect("create an adc before using it")
            .lock()
            .await
            .read_many(ch, buf, div, dma)
            .await
    }

    pub async fn read_many_raw(
        &mut self,
        ch: &mut Channel<'_>,
        buf: &mut [Sample],
        div: u16,
        dma: impl Peripheral<P = impl DmaChannel>,
    ) {
        ADC.get()
            .expect("create an adc before using it")
            .lock()
            .await
            .read_many_raw(ch, buf, div, dma)
            .await
    }
}
