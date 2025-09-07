pub mod crawler;
pub mod errors;
pub mod parser;
pub mod storage;
pub mod url_queue;

use url::Url;

pub fn is_valid_url(s: &str) -> bool {
    Url::parse(s).is_ok()
}
