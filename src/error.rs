use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse model.")]
    ModelParseError(#[from] threemf::error::Error),
}
