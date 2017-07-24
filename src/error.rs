use std::io;

use sdl2::video::WindowBuildError;
use sdl2::IntegerOrSdlError;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Text(String),
    WindowBuildError,
    IntegerOrSdlError,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Text(err)
    }
}

impl From<WindowBuildError> for Error {
    fn from(_: WindowBuildError) -> Error {
        Error::WindowBuildError
    }
}

impl From<IntegerOrSdlError> for Error {
    fn from(_: IntegerOrSdlError) -> Error {
        Error::IntegerOrSdlError
    }
}
