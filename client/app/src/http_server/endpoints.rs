use defmt::error;
use embassy_futures::select::{select, Either};
use embedded_io_async::Read;
use picoserve::{
	request::Request,
	response::{IntoResponse, Json, ResponseWriter, StatusCode},
	ResponseSent,
};

use crate::{
	dfu::{
		DfuUpdateResult,
		DFU_PIPE_BUFFER_SIZE,
		DFU_PIPE_DONE_SIGNAL,
		DFU_UPDATE_PIPE,
		DFU_UPDATE_SIGNAL,
	},
	http_server::{AppRouter, AppState},
	sensors::get_sensors,
};

pub fn router() -> picoserve::Router<AppRouter, AppState> {
	picoserve::Router::new()
		.route("/", picoserve::routing::get(index))
		.route("/data", picoserve::routing::get(data))
		.route(
			"/dfu",
			picoserve::routing::get(method_not_allowed).post_service(DfuUpdateEndpoint),
		)
}

async fn method_not_allowed() -> impl IntoResponse {
	StatusCode::new(405)
}

async fn index() -> impl IntoResponse {
	"see /data for the real shit"
}

async fn data() -> impl IntoResponse {
	let sensors = get_sensors();
	match sensors.lock().await.get_data().await {
		Ok(r) => Ok(Json(r)),
		Err(e) => {
			error!("Got sensor error while serving request: {:?}", e);
			Err((
				StatusCode::new(500),
				"Internal server error. Check logs for details.",
			))
		}
	}
}

struct DfuUpdateEndpoint;

impl picoserve::routing::RequestHandlerService<AppState> for DfuUpdateEndpoint {
	async fn call_request_handler_service<R: Read, W: ResponseWriter<Error = R::Error>>(
		&self,
		_state: &AppState,
		_path_parameters: (),
		mut request: Request<'_, R>,
		response_writer: W,
	) -> Result<ResponseSent, W::Error> {
		let mut body_rdr = request.body_connection.body().reader();

		let mut buf = [0; DFU_PIPE_BUFFER_SIZE];
		let mut read;
		loop {
			read = match select(body_rdr.read(&mut buf), DFU_UPDATE_SIGNAL.wait()).await {
				Either::First(res) => res?,
				Either::Second(DfuUpdateResult::Failure(err)) => {
					return picoserve::response::Response::new(
						StatusCode::new(500),
						format_args!("{:?}", err),
					)
					.write_to(request.body_connection.finalize().await?, response_writer)
					.await
				}
				Either::Second(DfuUpdateResult::Success) => {
					panic!("unexpected DfuUpdateResult::Success")
				}
			};
			if read == 0 {
				break;
			}

			let res = select(
				DFU_UPDATE_PIPE.write_all(&buf[..read]),
				DFU_UPDATE_SIGNAL.wait(),
			)
			.await;
			match res {
				Either::First(()) => {}
				Either::Second(DfuUpdateResult::Failure(err)) => {
					return picoserve::response::Response::new(
						StatusCode::new(500),
						format_args!("{:?}", err),
					)
					.write_to(request.body_connection.finalize().await?, response_writer)
					.await
				}
				Either::Second(DfuUpdateResult::Success) => {
					panic!("unexpected DfuUpdateResult::Success")
				}
			};
		}
		DFU_PIPE_DONE_SIGNAL.signal(());

		picoserve::response::Response::new(StatusCode::new(200), "{}")
			.write_to(request.body_connection.finalize().await?, response_writer)
			.await
	}
}
