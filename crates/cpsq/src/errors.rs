use {
    common_cli::*,
    compris::{read::*, ser::*},
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
    Exit(Exit),

    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    #[error("read: {0}")]
    Read(#[from] ReadError),

    #[error("write: {0}")]
    Write(#[from] WriteError),

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
