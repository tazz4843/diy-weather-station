use defmt::unwrap;
use embassy_time::Duration;
use picoserve::{KeepAlive, ShutdownMethod, Timeouts};
use static_cell::make_static;

mod endpoints;

pub struct AppState {}

type AppRouter = impl picoserve::routing::PathRouter<AppState>;

pub const WEB_TASK_POOL_SIZE: usize = 4;

#[embassy_executor::task(pool_size = WEB_TASK_POOL_SIZE)]
async fn web_task(
	id: usize,
	stack: &'static embassy_net::Stack<cyw43::NetDriver<'static>>,
	app: &'static picoserve::Router<AppRouter, AppState>,
	config: &'static picoserve::Config<Duration>,
	state: AppState,
) -> ! {
	let mut tcp_rx_buffer = [0; 1024];
	let mut tcp_tx_buffer = [0; 1024];
	let mut http_buffer = [0; 2048];

	picoserve::listen_and_serve_with_state(
		id,
		app,
		config,
		stack,
		80,
		&mut tcp_rx_buffer,
		&mut tcp_tx_buffer,
		&mut http_buffer,
		&state,
	)
	.await
}

pub async fn set_up_and_spawn_webserver(
	spawner: &embassy_executor::Spawner,
	stack: &'static embassy_net::Stack<cyw43::NetDriver<'static>>,
) {
	let app = make_static!(endpoints::router());
	let config = make_static!(picoserve::Config {
		timeouts:        Timeouts {
			start_read_request: Some(Duration::from_secs(1)),
			read_request:       Some(Duration::from_secs(5)),
			write:              Some(Duration::from_secs(3)),
		},
		connection:      KeepAlive::KeepAlive,
		shutdown_method: ShutdownMethod::Shutdown,
	});

	for id in 0..WEB_TASK_POOL_SIZE {
		unwrap!(spawner.spawn(web_task(id, stack, app, config, AppState {})))
	}
}
