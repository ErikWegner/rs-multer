use std::fmt::{self, Debug, Display, Formatter};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// A set of errors that can occur during parsing multipart stream and in other
/// operations.
#[non_exhaustive]
pub enum Error {
    /// An unknown field is detected when multipart
    /// [`constraints`](crate::Constraints::allowed_fields) are added.
    UnknownField { field_name: Option<String> },

    /// The field data is found incomplete.
    IncompleteFieldData { field_name: Option<String> },

    /// Couldn't read the field headers completely.
    IncompleteHeaders,

    /// Failed to read headers.
    ReadHeaderFailed(httparse::Error),

    /// Failed to decode the field's raw header name to
    /// [`HeaderName`](http::header::HeaderName) type.
    DecodeHeaderName { name: String, cause: BoxError },

    /// Failed to decode the field's raw header value to
    /// [`HeaderValue`](http::header::HeaderValue) type.
    DecodeHeaderValue { value: Vec<u8>, cause: BoxError },

    /// Multipart stream is incomplete.
    IncompleteStream,

    /// The incoming field size exceeded the maximum limit.
    FieldSizeExceeded { limit: u64, field_name: Option<String> },

    /// The incoming stream size exceeded the maximum limit.
    StreamSizeExceeded { limit: u64 },

    /// Stream read failed.
    StreamReadFailed(BoxError),

    /// Failed to lock the multipart shared state for any changes.
    LockFailure,

    /// The `Content-Type` header is not `multipart/form-data`.
    NoMultipart,

    /// Failed to convert the `Content-Type` to [`mime::Mime`] type.
    DecodeContentType(mime::FromStrError),

    /// No boundary found in `Content-Type` header.
    NoBoundary,

    /// More than one boundary found in `Content-Type` header.
    MultipleBoundaries,

    /// Failed to decode the field data as `JSON` in
    /// [`field.json()`](crate::Field::json) method.
    #[cfg(feature = "json")]
    #[cfg_attr(nightly, doc(cfg(feature = "json")))]
    DecodeJson(serde_json::Error),
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnknownField { field_name } => {
                let name = field_name.as_deref().unwrap_or("<unknown>");
                write!(f, "unknown field received: {:?}", name)
            }
            Error::IncompleteFieldData { field_name } => {
                let name = field_name.as_deref().unwrap_or("<unknown>");
                write!(f, "field {:?} received with incomplete data", name)
            }
            Error::DecodeHeaderName { name, .. } => {
                write!(f, "failed to decode field's raw header name: {:?}", name)
            }
            Error::DecodeHeaderValue { .. } => {
                write!(f, "failed to decode field's raw header value")
            }
            Error::FieldSizeExceeded { limit, field_name } => {
                let name = field_name.as_deref().unwrap_or("<unknown>");
                write!(f, "field {:?} exceeded the size limit: {} bytes", name, limit)
            }
            Error::StreamSizeExceeded { limit } => {
                write!(f, "stream size exceeded limit: {} bytes", limit)
            }
            Error::ReadHeaderFailed(_) => write!(f, "failed to read headers"),
            Error::StreamReadFailed(_) => write!(f, "failed to read stream"),
            Error::DecodeContentType(_) => write!(f, "failed to decode Content-Type"),
            Error::IncompleteHeaders => write!(f, "failed to read field complete headers"),
            Error::IncompleteStream => write!(f, "incomplete multipart stream"),
            Error::LockFailure => write!(f, "failed to lock multipart state"),
            Error::NoMultipart => write!(f, "Content-Type is not multipart/form-data"),
            Error::NoBoundary => write!(f, "multipart boundary not found in Content-Type"),
            Error::MultipleBoundaries => write!(f, "multipart boundary found multiple times in Content-Type"),
            #[cfg(feature = "json")]
            Error::DecodeJson(_) => write!(f, "failed to decode field data as JSON"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ReadHeaderFailed(e) => Some(e),
            Error::DecodeHeaderName { cause, .. } => Some(cause.as_ref()),
            Error::DecodeHeaderValue { cause, .. } => Some(cause.as_ref()),
            Error::StreamReadFailed(e) => Some(e.as_ref()),
            Error::DecodeContentType(e) => Some(e),
            #[cfg(feature = "json")]
            Error::DecodeJson(e) => Some(e),
            Error::UnknownField { .. }
            | Error::IncompleteFieldData { .. }
            | Error::IncompleteHeaders
            | Error::IncompleteStream
            | Error::FieldSizeExceeded { .. }
            | Error::StreamSizeExceeded { .. }
            | Error::LockFailure
            | Error::NoMultipart
            | Error::NoBoundary
            | Error::MultipleBoundaries => None,
        }
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl Eq for Error {}
