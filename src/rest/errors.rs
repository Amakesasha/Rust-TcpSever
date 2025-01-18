use crate::*;

/// Represents various errors that can occur during server operation.
#[derive(Debug, Error)]
pub enum ServerError {
    /// Indicates that the request is empty or took too long to receive.
    #[error("Empty request or it took too long")]
    EmptyRequest,
    /// Indicates that an empty line or took too long to receive.
    #[error("Empty line or it took too long")]
    EmptyLine,
    /// Indicates that the first line of the request is broken
    /// or doesn't have exactly 3 elements (method, path, and http version).
    #[error("The number of elements in the first row is not equal to 3")]
    BrokenFirstLine,

    /// Indicates that a TcpStream failed validation (only with `check_stream` feature).
    #[cfg(feature = "check_stream")]
    #[error("TcpStream failed validation")]
    VerificationFailed,
    /// Indicates that an unknown HTTP method was received.
    #[error("Unknown HTTP method")]
    UnknownMethod(String),
    /// Indicates an error occurred when getting the Socket Address.
    #[error("Error getting Socket Adder")]
    GetSocketAddr(std::io::Error),
    /// Indicates that the socket address was empty when it was expected to have value.
    #[error("The socket address was empty, although it was expected to have a value")]
    SocketAddrEmpty,

    /// Indicates an error occurred while reading data from a source.
    #[error("Reading error")]
    ReadError(std::io::Error),
    /// Indicates an error occurred while writing data to a destination.
    #[error("Writing error")]
    WriteError(std::io::Error),
    /// Indicates an error occurred while flushing.
    #[error("Flush error")]
    FlushError(std::io::Error),
}
