#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m_rt::{entry, exception};
use defmt::{debug, info};
use defmt_rtt as _;
use embassy_boot_rp::*;
use embassy_rp::gpio::{Level, Output};
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Duration, Instant};

const FLASH_SIZE: usize = 2 * 1024 * 1024;
const MAXIMUM_WATCHDOG_TIMEOUT: Duration = Duration::from_micros(8388607);

#[entry]
fn main() -> ! {
	unsafe { embassy_rp::time_driver::init() }

	let p = embassy_rp::init(Default::default());

	info!("Bootloader started");
	let bl_start = Instant::now();

	info!("Initializing flash");
	let flash = {
		let start_time = Instant::now();
		let flash =
			WatchdogFlash::<FLASH_SIZE>::start(p.FLASH, p.WATCHDOG, MAXIMUM_WATCHDOG_TIMEOUT);
		let flash = Mutex::new(RefCell::new(flash));
		let elapsed = start_time.elapsed();
		debug!("Flash initialized in {:?}μs", elapsed.as_micros());
		flash
	};

	info!("Finding config");
	let start_time = Instant::now();
	let config = BootLoaderConfig::from_linkerfile_blocking(&flash, &flash, &flash);
	let active_offset = config.active.offset();
	let elapsed = start_time.elapsed();
	debug!("Active offset: {}", active_offset);
	debug!("Config found in {:?}μs", elapsed.as_micros());
	info!("Preparing firmware");
	let start_time = Instant::now();
	let bl: BootLoader = BootLoader::prepare(config);
	let elapsed = start_time.elapsed();
	debug!("Firmware prepared in {:?}μs", elapsed.as_micros());
	let elapsed = bl_start.elapsed();
	debug!("Bootloader done in {:?}μs", elapsed.as_micros());

	info!("Booting application");
	unsafe { bl.load(embassy_rp::flash::FLASH_BASE as u32 + active_offset) }
}

#[no_mangle]
#[cfg_attr(target_os = "none", link_section = ".HardFault.user")]
unsafe extern "C" fn HardFault() {
	cortex_m::peripheral::SCB::sys_reset();
}

#[exception]
unsafe fn DefaultHandler(_: i16) -> ! {
	const SCB_ICSR: *const u32 = 0xe000_ed04 as *const u32;
	let irqn = core::ptr::read_volatile(SCB_ICSR) as u8 as i16 - 16;

	panic!("DefaultHandler #{:?}", irqn);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	cortex_m::asm::udf();
}
