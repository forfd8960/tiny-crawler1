use std::collections::HashMap;

use reqwest::StatusCode;

use crate::errors::Errors;

#[derive(Debug)]
pub struct Crawler {
    seed_urls: Vec<String>,
    max_depth: u32,
    concurrency: u32,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub data: String,               // the response data
    pub status: Option<StatusCode>, // http response status
    pub err: Option<String>,        // err info if request failed
}

#[derive(Debug)]
pub struct CrawlResult {
    pub data: HashMap<String, Response>,
}

impl CrawlResult {
    pub fn new(cap: usize) -> Self {
        Self {
            data: HashMap::with_capacity(cap),
        }
    }

    pub fn append_res(&mut self, url: String, resp: Response) {
        self.data.insert(url, resp);
    }
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
        let client = reqwest::Client::new();
        let mut result = CrawlResult::new((self.max_depth * 10) as usize);

        for url in &self.seed_urls {
            let res = client.get(url.clone()).send().await?;
            let status = res.status();

            match res.text().await {
                Ok(text) => {
                    result.append_res(
                        url.to_string(),
                        Response {
                            data: text,
                            status: Some(status),
                            err: None,
                        },
                    );
                }
                Err(e) => {
                    result.append_res(
                        url.to_string(),
                        Response {
                            data: "".to_string(),
                            status: None,
                            err: Some(e.to_string()),
                        },
                    );
                }
            }
        }

        Ok(result)
    }
}
