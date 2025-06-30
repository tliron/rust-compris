use {
    compris::{parse::*, ser::*, *},
    kutil_cli::run::*,
    read_url::*,
    std::io,
    thiserror::*,
};

//
// MainError
//

/// Main error.
#[derive(Debug, Error)]
pub enum MainError {
    /// Exit.
    #[error("{0}")]
    Exit(#[from] ExitError),

    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// Unknown format.
    #[error("unknown format: {0}")]
    UnknownFormat(#[from] UnknownFormatError),

    /// Read.
    #[error("read: {0}")]
    Read(#[from] ParseError),

    /// Write.
    #[error("write: {0}")]
    Write(#[from] SerializeError),

    /// URL.
    #[error("URL: {0}")]
    Url(#[from] UrlError),
}

impl RunError for MainError {
    fn handle(&self) -> (bool, u8) {
        (
            false,
            match self {
                MainError::Exit(exit) => exit.code,
                _ => 1,
            },
        )
    }
}
