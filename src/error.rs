use std::io;

use sdl2::video::WindowBuildError;
use sdl2::IntegerOrSdlError;
use sdl2::render::TextureValueError;
use sdl2::render::TargetRenderError;
use sdl2::render::UpdateTextureError;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Text(String),
    WindowBuildError,
    IntegerOrSdlError,
    TextureValueError,
    TargetRenderError,
    UpdateTextureError,
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

impl From<TextureValueError> for Error {
    fn from(_: TextureValueError) -> Error {
        Error::TextureValueError
    }
}

impl From<TargetRenderError> for Error {
    fn from(_: TargetRenderError) -> Error {
        Error::TargetRenderError
    }
}

impl From<UpdateTextureError> for Error {
    fn from(_: UpdateTextureError) -> Error {
        Error::UpdateTextureError
    }
}
