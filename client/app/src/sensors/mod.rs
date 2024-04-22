use common::json_payloads::SensorData;
use cpu_temp_sensor::CpuTempSensor;
use defmt::{write, Formatter};
use embassy_futures::join::{join4, join5};
use embassy_rp::peripherals::{ADC_TEMP_SENSOR, PIN_26, PIN_28};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use ens160::Ens160Sensor;
use lis2mdl::{Lis2mdlError, Lis2mdlSensor};
use mpl3115a2::{Mpl3115a2Error, Mpl3115a2Sensor};
use noise::NoiseSensor;
use once_cell::sync::OnceCell;
use sht4x::{Sht4xError, Sht4xSensor};
use tsl2591::{Tsl2591Error, Tsl2591Sensor};
use uv::UvSensor;

mod cpu_temp_sensor;
mod ens160;
mod lis2mdl;
mod mpl3115a2;
mod noise;
mod sht4x;
mod tsl2591;
mod uv;

pub struct Sensors<'a> {
    ens160: Ens160Sensor,
    mpl3115a2: Mpl3115a2Sensor,
    sht4x: Sht4xSensor,
    tsl2591: Tsl2591Sensor,
    lis2mdl: Lis2mdlSensor,
    uv: UvSensor<'a>,
    noise: NoiseSensor<'a>,
    cpu_temp: CpuTempSensor<'a>,
}

impl Sensors<'_> {
    pub async fn new(
        pin_26: PIN_26,
        pin_28: PIN_28,
        cpu_temp: ADC_TEMP_SENSOR,
    ) -> Result<Self, SensorError> {
        Ok(Self {
            ens160: Ens160Sensor::new().await?,
            mpl3115a2: Mpl3115a2Sensor::new().await?,
            sht4x: Sht4xSensor::new().await?,
            tsl2591: Tsl2591Sensor::new().await?,
            lis2mdl: Lis2mdlSensor::new().await?,
            uv: UvSensor::new(pin_26),
            noise: NoiseSensor::new(pin_28),
            cpu_temp: CpuTempSensor::new(cpu_temp),
        })
    }

    pub async fn get_data(&mut self) -> Result<SensorData, SensorError> {
        let (air_quality, pressure, temperature, light, (magnetic, uv, noise, cpu_temp)) = join5(
            self.ens160.get_data(),
            self.mpl3115a2.get_data(),
            self.sht4x.get_data(),
            self.tsl2591.get_data(),
            join4(
                self.lis2mdl.get_data(),
                self.uv.get_data(),
                self.noise.get_data(),
                self.cpu_temp.get_data(),
            ),
        )
        .await;
        let air_quality = air_quality?;
        let pressure = pressure?;
        let temperature = temperature?;
        let light = light?;
        let magnetic = magnetic?;
        let uv = uv?;
        let noise = noise?;
        let cpu_temp = cpu_temp?;
        Ok(SensorData {
            air_quality,
            pressure,
            temperature,
            light,
            magnetic,
            uv,
            noise,
            cpu_temp,
        })
    }
}

pub enum SensorError {
    I2c(embassy_rp::i2c::Error),
    Adc(embassy_rp::adc::Error),
    InvalidData,
    UnsupportedChip,
    Unknown,
    IdMismatch(u8),
    SignalOverflow,
    InfraredOverflow,
    /// Heuristic was unable to find a valid gain/integration time combination
    /// for the current light level
    TotalOverflow,
}

impl From<embassy_rp::i2c::Error> for SensorError {
    fn from(error: embassy_rp::i2c::Error) -> Self {
        Self::I2c(error)
    }
}

impl From<embassy_rp::adc::Error> for SensorError {
    fn from(error: embassy_rp::adc::Error) -> Self {
        Self::Adc(error)
    }
}

impl From<Mpl3115a2Error> for SensorError {
    fn from(error: Mpl3115a2Error) -> Self {
        match error {
            Mpl3115a2Error::I2c(e) => Self::I2c(e),
            Mpl3115a2Error::InvalidData => Self::InvalidData,
            Mpl3115a2Error::UnsupportedChip => Self::UnsupportedChip,
        }
    }
}

impl From<Sht4xError> for SensorError {
    fn from(error: Sht4xError) -> Self {
        match error {
            Sht4xError::I2c(e) => Self::I2c(e),
            Sht4xError::Unknown => Self::Unknown,
        }
    }
}

impl From<Tsl2591Error> for SensorError {
    fn from(error: Tsl2591Error) -> Self {
        match error {
            Tsl2591Error::I2c(e) => Self::I2c(e),
            Tsl2591Error::IdMismatch(id) => Self::IdMismatch(id),
            Tsl2591Error::SignalOverflow => Self::SignalOverflow,
            Tsl2591Error::InfraredOverflow => Self::InfraredOverflow,
            Tsl2591Error::TotalOverflow => Self::TotalOverflow,
        }
    }
}

impl From<Lis2mdlError> for SensorError {
    fn from(error: Lis2mdlError) -> Self {
        match error {
            Lis2mdlError::I2c(e) => Self::I2c(e),
            Lis2mdlError::InvalidData => Self::InvalidData,
        }
    }
}

impl defmt::Format for SensorError {
    fn format(&self, fmt: Formatter) {
        match self {
            Self::I2c(e) => write!(fmt, "I2C error: {:?}", e),
            Self::Adc(e) => write!(fmt, "ADC error: {:?}", e),
            Self::InvalidData => write!(fmt, "Invalid data"),
            Self::UnsupportedChip => write!(fmt, "Unsupported chip"),
            Self::Unknown => write!(fmt, "Unknown error"),
            Self::IdMismatch(id) => write!(fmt, "ID mismatch: {}", id),
            Self::SignalOverflow => write!(fmt, "Signal overflow"),
            Self::InfraredOverflow => write!(fmt, "Infrared overflow"),
            Self::TotalOverflow => write!(fmt, "Total overflow"),
        }
    }
}

static SENSOR_GLOBAL: OnceCell<Mutex<CriticalSectionRawMutex, Sensors>> = OnceCell::new();
pub fn set_sensors(sensors: Sensors<'static>) {
    SENSOR_GLOBAL.set(Mutex::new(sensors)).unwrap_or_else(|_| {
        panic!("Sensors already initialized, this should never happen (sensors are a singleton)")
    });
}
pub fn get_sensors() -> &'static Mutex<CriticalSectionRawMutex, Sensors<'static>> {
    SENSOR_GLOBAL
        .get()
        .expect("create sensors before using them")
}
