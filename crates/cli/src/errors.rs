use {
    compris::{UnknownFormatError, parse::*, ser::*},
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
    #[error("exit: {0}")]
    Exit(#[from] Exit),

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

impl HasExit for MainError {
    fn get_exit(&self) -> Option<&Exit> {
        if let MainError::Exit(exit) = self { Some(exit) } else { None }
    }
}
