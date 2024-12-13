use {
    compris::{parse::*, ser::*, UnknownFormatError},
    kutil_cli::run::*,
    read_url::*,
    std::io,
    thiserror::*,
};

//
// MainError
//

#[derive(Error, Debug)]
pub enum MainError {
    #[error("exit: {0}")]
    Exit(#[from] Exit),

    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    #[error("unknown format: {0}")]
    UnknownFormat(#[from] UnknownFormatError),

    #[error("read: {0}")]
    Read(#[from] ParseError),

    #[error("write: {0}")]
    Write(#[from] SerializeError),

    #[error("URL: {0}")]
    Url(#[from] UrlError),
}

impl HasExit for MainError {
    fn get_exit(&self) -> Option<&Exit> {
        if let MainError::Exit(exit) = self {
            Some(exit)
        } else {
            None
        }
    }
}
