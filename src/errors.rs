use crate::*;

/// Represents various errors that can occur during server operation.
#[derive(Debug, Error)]
pub enum ServerError {
    /* --- Request parsing errors --- */
    /// Indicates that the first line of the request is broken
    /// or doesn't have exactly 3 elements (method, path, and http version).
    #[error("The number of elements in the first row is not equal to 3")]
    BrokenFirstLine,
    /// Unknown request method.
    #[error("Unknown request method")]
    InvalidMethod(http::method::InvalidMethod),
    /// Error parsing URL.
    #[error("Error parsing URL")]
    InvalidUrl(http::uri::InvalidUri),
    /// Empty header or it took too long.
    #[error("Empty header or it took too long")]
    InvalidHeader,
    /// Error parsing header value.
    #[error("Error parsing header name")]
    InvalidHeaderName(#[from] http::header::InvalidHeaderName),
    /// Error parsing header value.
    #[error("Error parsing header value")]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),

    /* --- Read/write errors --- */
    /// Indicates that the request is empty or took too long to receive.
    #[error("Empty request or it took too long")]
    EmptyRequest,
    /// Indicates that an empty line or took too long to receive.
    #[error("Empty line or it took too long")]
    EmptyLine,
    /// Indicates an error occurred while reading data from a source.
    #[error("Reading error")]
    Read(std::io::Error),
    /// Indicates an error occurred while writing data to a destination.
    #[error("Writing error")]
    Write(std::io::Error),
    /// Indicates an error occurred while flushing.
    #[error("Flush error")]
    Flush(std::io::Error),

    /* --- Network errors --- */
    /// Indicates that a TcpStream failed validation.
    #[error("TcpStream failed validation")]
    VerificationFailed,
    #[cfg(feature = "get_stream")]
    /// Indicates an error occurred when getting the Socket Address.
    #[error("Error getting Socket Adder")]
    GetSocketAddr,
    /// This HTTP status does not exist.
    #[error("This HTTP status does not exist: {0}")]
    UnknownHttpStatus(u16),

    /* --- File system errors --- */
    /// The provided path is not a file.
    #[error("The provided path is not a file")]
    FolderInsteadFile,
    /// The file is missing or something is blocking it from opening
    #[error("The file is missing or something is blocking it from opening")]
    OpeningFile(std::io::Error),
}
