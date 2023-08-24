#![cfg_attr(not(test), no_std)]
#![feature(impl_trait_projections)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
#![doc = include_str!("../README.md")]
use core::{num::ParseIntError, str::Utf8Error};

use embedded_io_async::{ReadExactError, WriteAllError};

mod fmt;

pub mod client;
mod concat;
pub mod headers;
pub mod request;
pub mod response;

/// Errors that can be returned by this library.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// An error with DNS (it's always DNS)
    Dns,
    /// An error with the underlying network
    Network(embedded_io_async::ErrorKind),
    /// An error encoding or decoding data
    Codec,
    /// An error parsing the URL
    InvalidUrl(nourl::Error),
    /// Tls Error
    #[cfg(feature = "embedded-tls")]
    Tls(embedded_tls::TlsError),
    /// The provided buffer is too small
    BufferTooSmall,
    /// The request is already sent
    AlreadySent,
    /// An invalid number of bytes were written to request body
    IncorrectBodyWritten,
    /// The underlying connection was closed
    ConnectionClosed,
}

impl embedded_io_async::Error for Error {
    fn kind(&self) -> embedded_io_async::ErrorKind {
        embedded_io_async::ErrorKind::Other
    }
}

impl From<embedded_io_async::ErrorKind> for Error {
    fn from(e: embedded_io_async::ErrorKind) -> Error {
        Error::Network(e)
    }
}

impl<E: embedded_io_async::Error> From<ReadExactError<E>> for Error {
    fn from(value: ReadExactError<E>) -> Self {
        match value {
            ReadExactError::UnexpectedEof => Error::ConnectionClosed,
            ReadExactError::Other(e) => Error::Network(e.kind()),
        }
    }
}

impl<E: embedded_io_async::Error> From<WriteAllError<E>> for Error {
    fn from(value: WriteAllError<E>) -> Self {
        match value {
            WriteAllError::WriteZero => Error::ConnectionClosed,
            WriteAllError::Other(e) => Error::Network(e.kind()),
        }
    }
}

#[cfg(feature = "embedded-tls")]
impl From<embedded_tls::TlsError> for Error {
    fn from(e: embedded_tls::TlsError) -> Error {
        Error::Tls(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Error {
        Error::Codec
    }
}

impl From<Utf8Error> for Error {
    fn from(_: Utf8Error) -> Error {
        Error::Codec
    }
}

impl From<nourl::Error> for Error {
    fn from(e: nourl::Error) -> Self {
        Error::InvalidUrl(e)
    }
}
