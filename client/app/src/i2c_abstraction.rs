use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embedded_hal_async::i2c::{ErrorType, I2c, Operation, SevenBitAddress};
use once_cell::sync::OnceCell;

static I2C_BUS: OnceCell<
    Mutex<
        CriticalSectionRawMutex,
        embassy_rp::i2c::I2c<embassy_rp::peripherals::I2C0, embassy_rp::i2c::Async>,
    >,
> = OnceCell::new();

pub fn init_bus(
    i2c: embassy_rp::i2c::I2c<'static, embassy_rp::peripherals::I2C0, embassy_rp::i2c::Async>,
) {
    I2C_BUS.set(Mutex::new(i2c)).unwrap_or_else(|_| {
        panic!("I2C bus already initialized, this should never happen (i2c0 bus is a singleton)")
    });
}

#[derive(Copy, Clone)]
pub struct SharedI2c;

impl ErrorType for SharedI2c {
    type Error = embassy_rp::i2c::Error;
}

impl I2c<SevenBitAddress> for SharedI2c {
    async fn read(
        &mut self,
        address: SevenBitAddress,
        read: &mut [u8],
    ) -> Result<(), embassy_rp::i2c::Error> {
        I2C_BUS
            .get()
            .expect("create a bus before using it")
            .lock()
            .await
            .read(address, read)
            .await
    }

    async fn write(
        &mut self,
        address: SevenBitAddress,
        write: &[u8],
    ) -> Result<(), embassy_rp::i2c::Error> {
        I2C_BUS
            .get()
            .expect("create a bus before using it")
            .lock()
            .await
            .write(address, write)
            .await
    }

    async fn write_read(
        &mut self,
        address: SevenBitAddress,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), embassy_rp::i2c::Error> {
        I2C_BUS
            .get()
            .expect("create a bus before using it")
            .lock()
            .await
            .write_read(address, write, read)
            .await
    }

    async fn transaction(
        &mut self,
        address: SevenBitAddress,
        operations: &mut [Operation<'_>],
    ) -> Result<(), embassy_rp::i2c::Error> {
        I2C_BUS
            .get()
            .expect("create a bus before using it")
            .lock()
            .await
            .transaction(address, operations)
            .await
    }
}
