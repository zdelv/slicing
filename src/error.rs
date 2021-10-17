use quick_xml::de::DeError;
use thiserror::Error;
use zip::result::ZipError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Too many models found.")]
    TooManyModels,
    #[error("No model data found in zip.")]
    #[allow(dead_code)]
    EmptyModel,
    #[error("Failed to unzip 3MF.")]
    #[allow(dead_code)]
    ZipError(#[from] ZipError),
    #[error("Failed to parse XML.")]
    #[allow(dead_code)]
    InvalidXML(#[from] DeError),
    #[error("Failed to read units.")]
    #[allow(dead_code)]
    InvalidUnits(String),
    #[error("Failed to parse data as string.")]
    #[allow(dead_code)]
    InvalidZipString(#[from] std::io::Error),
}
