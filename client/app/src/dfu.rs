use embassy_boot::{FirmwareUpdater, FirmwareUpdaterError};
use embassy_boot_rp::{AlignedBuffer, FirmwareUpdaterConfig};
use embassy_embedded_hal::flash::partition::Error as PartitionError;
use embassy_futures::select::{select, Either};
use embassy_rp::{
	flash::Error as FlashError,
	peripherals::{DMA_CH1, FLASH},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex, pipe::Pipe};
use embassy_time::Timer;
use embedded_storage_async::nor_flash::NorFlash;

use crate::FLASH_SIZE;

/// Size of the buffer used for DFU updates
///
/// Equal to four pages of flash memory
pub const DFU_PIPE_BUFFER_SIZE: usize = embassy_rp::flash::PAGE_SIZE * 4;

pub type DfuUpdatePipe = Pipe<CriticalSectionRawMutex, DFU_PIPE_BUFFER_SIZE>;
pub static DFU_UPDATE_PIPE: DfuUpdatePipe = DfuUpdatePipe::new();

pub type DfuPipeDoneSignal = embassy_sync::signal::Signal<CriticalSectionRawMutex, ()>;
pub static DFU_PIPE_DONE_SIGNAL: DfuPipeDoneSignal = DfuPipeDoneSignal::new();

pub type DfuUpdateSignal = embassy_sync::signal::Signal<CriticalSectionRawMutex, DfuUpdateResult>;
pub static DFU_UPDATE_SIGNAL: DfuUpdateSignal = DfuUpdateSignal::new();

pub enum DfuUpdateResult {
	/// DFU update was successful
	///
	/// No guarantee this will ever be received,
	/// as the chip is reset rather quickly after a successful update
	Success,

	/// DFU update failed
	///
	/// Error message is included
	Failure(DfuUpdateFailureReason),
}
pub enum DfuUpdateFailureReason {
	/// DFU update failed due to a firmware updater error
	FirmwareUpdater(FirmwareUpdaterError),
	/// DFU update failed due to a flash error
	Flash(FlashError),
	/// DFU update failed due to a partition error
	Partition(PartitionError<FlashError>),
}
impl From<FirmwareUpdaterError> for DfuUpdateFailureReason {
	fn from(e: FirmwareUpdaterError) -> Self {
		Self::FirmwareUpdater(e)
	}
}
impl From<FlashError> for DfuUpdateFailureReason {
	fn from(e: FlashError) -> Self {
		Self::Flash(e)
	}
}
impl From<PartitionError<FlashError>> for DfuUpdateFailureReason {
	fn from(e: PartitionError<FlashError>) -> Self {
		match e {
			PartitionError::Flash(e) => Self::Flash(e),
			e => Self::Partition(e),
		}
	}
}
impl core::fmt::Debug for DfuUpdateFailureReason {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::FirmwareUpdater(e) => write!(f, "FirmwareUpdaterError({:?})", e),
			Self::Flash(e) => write!(f, "FlashError({:?})", e),
			Self::Partition(e) => write!(f, "PartitionError({:?})", e),
		}
	}
}

#[embassy_executor::task]
pub async fn init_dfu(flash: FLASH, dma: DMA_CH1) -> ! {
	let flash = Mutex::new(embassy_rp::flash::Flash::<_, _, FLASH_SIZE>::new(
		flash, dma,
	));
	let updater_config = FirmwareUpdaterConfig::from_linkerfile(&flash, &flash);
	let mut aligned = AlignedBuffer([0; embassy_rp::flash::WRITE_SIZE]);
	let mut updater = FirmwareUpdater::new(updater_config, &mut aligned.0);

	'outer: loop {
		// wait for data to show up in the pipe (which may be never!)
		let mut buf = AlignedBuffer([0; DFU_PIPE_BUFFER_SIZE]);
		let size = DFU_UPDATE_PIPE.read(&mut buf.0).await as u32;

		let flash = match updater.prepare_update().await {
			Ok(flash) => flash,
			Err(e) => {
				DFU_UPDATE_SIGNAL.signal(DfuUpdateResult::Failure(e.into()));
				continue;
			}
		};

		let mut offset = 0;
		if let Err(e) = flash.write(offset, &buf.0).await {
			DFU_UPDATE_SIGNAL.signal(DfuUpdateResult::Failure(e.into()));
			continue;
		}
		offset += size;

		let mut read;
		loop {
			let res = select(
				DFU_PIPE_DONE_SIGNAL.wait(),
				DFU_UPDATE_PIPE.read(&mut buf.0),
			)
			.await;
			read = match res {
				Either::First(()) | Either::Second(0) => break,
				Either::Second(read) => read as u32,
			};
			if let Err(e) = flash.write(offset, &buf.0).await {
				DFU_UPDATE_SIGNAL.signal(DfuUpdateResult::Failure(e.into()));
				continue 'outer;
			}
			offset += read;
		}

		if let Err(e) = updater.mark_updated().await {
			DFU_UPDATE_SIGNAL.signal(DfuUpdateResult::Failure(e.into()));
			continue;
		}
		DFU_UPDATE_SIGNAL.signal(DfuUpdateResult::Success);
		// wait 2 seconds for the signal to be sent and HTTP connection to close
		Timer::after_secs(2).await;
		// reset the chip
		cortex_m::peripheral::SCB::sys_reset();
	}
}
