use code::ToErrorInfo;
use thiserror::Error;

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

fn main() {
    let err = BusinessError::Unknown;
    let info = err.to_error_info();

    println!("{:?}", info);
}
