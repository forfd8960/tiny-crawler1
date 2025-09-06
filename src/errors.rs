use thiserror::Error;
use tokio::sync::mpsc;

use crate::url_queue::Link;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("craw data failed")]
    CrawDataError(#[from] reqwest::Error),
    #[error("crawl depth: {0} exceed max_depth: {1}")]
    InvalidDepth(usize, usize),
    #[error("create file failed")]
    SaveDataFailed(#[from] std::io::Error),

    #[error("send url failed")]
    SendURL2QueueFailed(#[from] mpsc::error::SendError<Link>),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    // #[error("unknown data store error")]
    // Unknown,
}
