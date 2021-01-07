use thiserror::Error;

#[derive(Error, Debug)]
pub enum LightGBMError {
    #[error("Error: {0}")]
    CError(String),
}
