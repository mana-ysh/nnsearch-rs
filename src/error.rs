
use thiserror::Error;


#[derive(Debug, Error, PartialEq)]
pub enum NNSearchError {
    #[error("ValueError: {0}")]
    ValueError(String),
}
