#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(slice_as_chunks)]

mod adc_abstraction;
mod config;
mod dfu;
mod http_server;
mod i2c_abstraction;
mod ntp;
mod rng;
mod sensors;

use cyw43_pio::PioSpi;
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
	adc::{Adc, InterruptHandler as AdcInterruptHandler},
	bind_interrupts,
	gpio::{Level, Output},
	i2c::{Config, InterruptHandler as I2CInterruptHandler},
	peripherals::{DMA_CH0, I2C0, PIO0},
	pio::{InterruptHandler as PioInterruptHandler, Pio},
	watchdog::Watchdog,
};
use embassy_time::{Duration, Timer};
use http_server::WEB_TASK_POOL_SIZE;
// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
#[cfg(feature = "debug")]
use panic_probe as _;
// release profile: reset into bootloader on panic
#[allow(unused_imports)]
#[cfg(not(feature = "debug"))]
use panic_reset as _;
use static_cell::make_static;

use crate::{
	adc_abstraction::init_adc,
	dfu::init_dfu,
	http_server::set_up_and_spawn_webserver,
	rng::gen_random,
	sensors::set_sensors,
};

bind_interrupts!(struct Irqs {
	PIO0_IRQ_0 => PioInterruptHandler<PIO0>;
	I2C0_IRQ => I2CInterruptHandler<I2C0>;
	ADC_IRQ_FIFO => AdcInterruptHandler;
});

const BLINK_PERIOD: Duration = Duration::from_nanos(333_333_333);
// watchdog timeout is set high due to how long DHCP can take
const WATCHDOG_TIMEOUT: Duration = Duration::from_secs(20);
const FLASH_SIZE: usize = 2 * 1024 * 1024;

#[embassy_executor::task]
async fn wifi_task(
	runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
	runner.run().await
}
#[embassy_executor::task]
async fn net_task(stack: &'static embassy_net::Stack<cyw43::NetDriver<'static>>) -> ! {
	stack.run().await
}

fn initialize_i2c(
	sda: embassy_rp::peripherals::PIN_20,
	scl: embassy_rp::peripherals::PIN_21,
	peri: I2C0,
) {
	let mut i2c_config = Config::default();
	i2c_config.frequency = 1_000_000;
	let i2c: embassy_rp::i2c::I2c<I2C0, embassy_rp::i2c::Async> =
		embassy_rp::i2c::I2c::new_async(peri, scl, sda, Irqs, i2c_config);
	i2c_abstraction::init_bus(i2c);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
	let p = embassy_rp::init(Default::default());

	// override bootloader watchdog
	let mut watchdog = Watchdog::new(p.WATCHDOG);
	watchdog.start(WATCHDOG_TIMEOUT);

	// set red LED high during boot
	let mut red_led = Output::new(p.PIN_2, Level::High);
	let mut green_led = Output::new(p.PIN_3, Level::Low);

	// prepare a flash memory object for future maybe DFU updates
	spawner.must_spawn(init_dfu(p.FLASH, p.DMA_CH1));

	// init i2c
	watchdog.feed();
	initialize_i2c(p.PIN_20, p.PIN_21, p.I2C0);

	// load Wi-Fi chip firmware from flash (requires `probe-rs` tool and firmware files pre-flashed)
	//     probe-rs download 43439A0.bin --format bin --chip RP2040 --base-address 0x101c0000
	//     probe-rs download 43439A0_clm.bin --format bin --chip RP2040 --base-address 0x101fa000
	let fw = unsafe { core::slice::from_raw_parts(0x101c0000 as *const u8, 230321) };
	let clm = unsafe { core::slice::from_raw_parts(0x101fa000 as *const u8, 4752) };

	// prepare SPI for Wi-Fi chip
	watchdog.feed();
	let pwr = Output::new(p.PIN_23, Level::Low);
	let cs = Output::new(p.PIN_25, Level::High);
	let mut pio = Pio::new(p.PIO0, Irqs);
	let spi = PioSpi::new(
		&mut pio.common,
		pio.sm0,
		pio.irq0,
		cs,
		p.PIN_24,
		p.PIN_29,
		p.DMA_CH0,
	);

	// init ADC
	watchdog.feed();
	init_adc(Adc::new(p.ADC, Irqs, embassy_rp::adc::Config::default()));

	// init sensors
	watchdog.feed();
	let sensors = match sensors::Sensors::new(p.PIN_26, p.PIN_28, p.ADC_TEMP_SENSOR).await {
		Ok(s) => s,
		Err(e) => {
			error!("Error initializing sensors: {:?}", e);
			// blink red LED indefinitely until watchdog resets
			loop {
				red_led.set_high();
				Timer::after(BLINK_PERIOD).await;
				red_led.set_low();
				Timer::after(BLINK_PERIOD).await;
			}
		}
	};
	set_sensors(sensors);

	let net_start = embassy_time::Instant::now();

	// init Wi-Fi chip
	watchdog.feed();
	let state = make_static!(cyw43::State::new());
	let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
	spawner.must_spawn(wifi_task(runner));

	// prepare chip for joining network
	watchdog.feed();
	control.init(clm).await;
	control
		.set_power_management(cyw43::PowerManagementMode::PowerSave)
		.await;
	let hardware_end = embassy_time::Instant::now();

	// init network stack
	watchdog.feed();
	let net_stack_config = embassy_net::Config::dhcpv4(Default::default());
	let net_stack = &*make_static!(embassy_net::Stack::new(
		net_device,
		net_stack_config,
		make_static!(embassy_net::StackResources::<{ WEB_TASK_POOL_SIZE + 1 }>::new()),
		gen_random()
	));
	spawner.must_spawn(net_task(net_stack));
	let net_stack_end = embassy_time::Instant::now();

	// join Wi-Fi network
	let mut success = false;
	for _ in 0..3 {
		watchdog.feed();
		match control
			.join_wpa2(config::WIFI_SSID, config::WIFI_PASSWORD)
			.await
		{
			Ok(_) => {
				success = true;
				break;
			}
			Err(e) => {
				info!("Error joining wifi: {:?}", e.status);
				Timer::after(Duration::from_millis(500)).await;
			}
		}
	}
	if !success {
		info!("Failed to join wifi");
		// blink red LED indefinitely until watchdog resets
		loop {
			red_led.set_high();
			Timer::after(BLINK_PERIOD).await;
			red_led.set_low();
			Timer::after(BLINK_PERIOD).await;
		}
	}
	let wifi_end = embassy_time::Instant::now();

	info!("waiting for DHCP...");
	while !net_stack.is_config_up() {
		watchdog.feed();
		Timer::after(Duration::from_millis(100)).await;
	}
	let net_end = embassy_time::Instant::now();

	info!(
		"Hardware init: {:?} ms",
		(hardware_end - net_start).as_millis()
	);
	info!(
		"Net stack init: {:?} ms",
		(net_stack_end - hardware_end).as_millis()
	);
	info!("Wifi init: {:?} ms", (wifi_end - net_stack_end).as_millis());
	info!("DHCP: {:?} ms", (net_end - wifi_end).as_millis());
	info!("Total init: {:?} ms", (net_end - net_start).as_millis());

	// start web server
	watchdog.feed();
	set_up_and_spawn_webserver(&spawner, net_stack).await;

	// boot complete, turn off red LED and green on for 0.5s
	red_led.set_low();
	green_led.set_high();
	Timer::after(Duration::from_millis(500)).await;
	green_led.set_low();

	loop {
		watchdog.feed();
		Timer::after(Duration::from_secs(1)).await;
	}
}
