use thiserror::Error;

#[derive(Error, Debug)]
pub enum PAGError {
    #[error("invalid pag file")]
    FileError,
    #[error("invalid image")]
    ImageError,
    #[error("invalid surface")]
    SurfaceError,
}
