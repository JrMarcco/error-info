use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::str::FromStr;

pub use derive::ToErrorInfo;

pub trait ToErrorInfo {
    type T: FromStr;

    fn to_error_info(&self) -> ErrorInfo<Self::T>;
}

pub struct ErrorInfo<T> {
    pub code: T, // could be http status code.
    pub inner_code: &'static str,
    pub client_msg: &'static str,
    pub server_msg: String,
    pub hash: String,
}

impl<T> ErrorInfo<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    pub fn new(
        code: &str,
        inner_code: &'static str,
        client_msg: &'static str,
        server_msg: impl fmt::Display,
    ) -> Self {
        let server_msg = server_msg.to_string();
        let mut hasher = DefaultHasher::new();

        server_msg.hash(&mut hasher);
        let hash = hasher.finish();

        Self {
            code: T::from_str(code).expect("Can not parse code."),
            inner_code,
            client_msg,
            server_msg: server_msg.to_string(),
            hash: URL_SAFE_NO_PAD.encode(hash.to_be_bytes()),
        }
    }
}

impl<T> ErrorInfo<T> {
    pub fn client_msg(&self) -> &str {
        if self.client_msg.is_empty() {
            &self.server_msg
        } else {
            self.client_msg
        }
    }
}

// For client msg
impl<T> fmt::Display for ErrorInfo<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}-{}] {}",
            self.inner_code,
            self.hash,
            self.client_msg()
        )
    }
}

// Fro server log
impl<T> fmt::Debug for ErrorInfo<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}-{}] {}", self.inner_code, self.hash, self.server_msg)
    }
}
