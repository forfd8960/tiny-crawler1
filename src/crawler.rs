use std::collections::HashMap;

use crate::errors::Errors;

#[derive(Debug)]
pub struct Crawler {
    seed_urls: Vec<String>,
    max_depth: u32,
    concurrency: u32,
}

#[derive(Debug)]
pub struct CrawlResult {
    pub data: HashMap<String, String>,
}

impl Crawler {
    pub fn new(seed_urls: Vec<String>, max_depth: u32, concurrency: u32) -> Self {
        Self {
            seed_urls,
            max_depth,
            concurrency,
        }
    }

    pub async fn crawl(&self) -> Result<CrawlResult, Errors> {
        //todo: implement crawl data from URLs
        /*
            let resp = reqwest::get("https://httpbin.org/ip")
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{resp:#?}");
            */
        Ok(CrawlResult {
            data: HashMap::new(),
        })
    }
}
