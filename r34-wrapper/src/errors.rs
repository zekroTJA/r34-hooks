use std::error;

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    ResponseStatusError(reqwest::Error),
    ResponseBodyReadError(reqwest::Error),
    XmlParsingError(quick_xml::DeError),
    Unexpected(Box<dyn error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestError(err) => write!(f, "request failed: {err}"),
            Self::ResponseStatusError(err) => write!(f, "response status error: {err}"),
            Self::ResponseBodyReadError(err) => write!(f, "reading response body failed: {err}"),
            Self::XmlParsingError(err) => write!(f, "parsing response body failed: {err}"),
            Self::Unexpected(err) => write!(f, "unexpected error: {err}"),
        }
    }
}

impl error::Error for Error {}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

pub type Result<T, E = Error> = core::result::Result<T, E>;
