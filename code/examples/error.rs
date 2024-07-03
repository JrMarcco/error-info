use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use backtrace::Backtrace;
use thiserror::Error;
use tokio::net::TcpListener;
use tracing::{error, info};

use code::ToErrorInfo;

#[allow(unused)]
#[derive(Debug, Error, ToErrorInfo)]
#[error_info(ty = "http::StatusCode", prefix = "0A")]
enum BusinessError {
    #[error("Invalid param: {0}.")]
    #[error_info(code = "400", inner_code = "IP")]
    InvalidParam(String),

    #[error("Item {0} not found.")]
    #[error_info(code = "404", inner_code = "NF")]
    NotFound(String),

    #[error("Internal server error: {0}.")]
    #[error_info(
        code = "500",
        inner_code = "IE",
        client_msg = "We had some server problem, please try again later."
    )]
    InternalError(String),
    #[error("Unknown error")]
    #[error_info(code = "500", inner_code = "UE")]
    Unknown,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(handle_index));
    let addr = "0.0.0.0:8080";

    info!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn handle_index() -> Result<&'static str, BusinessError> {
    let bt = Backtrace::new();
    Err(BusinessError::InternalError(format!("{bt:?}")))
}

impl IntoResponse for BusinessError {
    fn into_response(self) -> Response {
        let info = self.to_error_info();
        let status = info.code;

        if status.is_server_error() {
            error!("{:?}", info);
        } else {
            info!("{:?}", info);
        }

        Response::builder()
            .status(status)
            .body(info.to_string().into())
            .unwrap()
    }
}
