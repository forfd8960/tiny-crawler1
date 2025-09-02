use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("craw data failed")]
    CrawDataError(#[from] reqwest::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    // #[error("unknown data store error")]
    // Unknown,
}
