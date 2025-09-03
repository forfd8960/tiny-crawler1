use crate::errors::Errors;
use crate::parser::ContentParser;
use reqwest::Client;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::pin::Pin;

#[derive(Debug)]
pub struct Seed {
    pub url: String,
    pub base: String,
}

#[derive(Debug)]
pub struct Crawler {
    seed_urls: Vec<Seed>,
    max_depth: usize,
    concurrency: u32,
    parser: ContentParser,
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
    pub fn new(seed_urls: Vec<Seed>, max_depth: usize, concurrency: u32) -> Self {
        Self {
            seed_urls,
            max_depth,
            concurrency,
            parser: ContentParser::new(),
        }
    }

    pub async fn retrieve_content(
        &self,
        client: &Client,
        url: &str,
        result: &mut CrawlResult,
    ) -> Result<String, Errors> {
        let res = client.get(url).send().await?;
        let status = res.status();

        let resp_content = res.text().await?;

        println!("append {}'s content to result", url);
        result.append_res(
            url.to_string(),
            Response {
                data: resp_content.clone(),
                status: Some(status),
                err: None,
            },
        );

        Ok(resp_content)
    }

    pub async fn crawl(&self, depth: usize) -> Result<CrawlResult, Errors> {
        //todo: implement crawl data from URLs
        /*
            let resp = reqwest::get("https://httpbin.org/ip")
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{resp:#?}");
            */
        if depth > self.max_depth {
            return Err(Errors::InvalidDepth(depth, self.max_depth));
        }

        let client = reqwest::Client::new();
        let mut result = CrawlResult::new(self.max_depth * 10);

        for seed in &self.seed_urls {
            let resp_content = self
                .retrieve_content(&client, &seed.url, &mut result)
                .await?;
            let sub_content = self.parser.parse(&resp_content, &seed.base)?;

            println!("sub_content links: {:?}", sub_content.links);

            let sub_res = self
                .boxed_crawl_sub(&mut result, sub_content.links, &seed.base, depth - 1)
                .await;

            match sub_res.await {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        Ok(result)
    }

    pub async fn boxed_crawl_sub<'a, 'b: 'a>(
        &'a self,
        result: &'b mut CrawlResult,
        sub_links: Vec<String>,
        base_url: &'a str,
        depth: usize,
    ) -> Pin<Box<dyn Future<Output = Result<(), Errors>> + 'a>> {
        Box::pin(self.crawl_sub(result, sub_links, base_url, depth))
    }

    pub async fn crawl_sub(
        &self,
        result: &mut CrawlResult,
        sub_links: Vec<String>,
        base_url: &str,
        depth: usize,
    ) -> Result<(), Errors> {
        if depth <= 0 {
            return Ok(());
        }

        if sub_links.len() == 0 {
            return Ok(());
        }

        let client = reqwest::Client::new();
        for link in sub_links {
            println!("crawling {} .........", link);

            let resp = self.retrieve_content(&client, &link, result).await?;
            let sub_content = self.parser.parse(&resp, base_url)?;

            let _ = self
                .boxed_crawl_sub(result, sub_content.links, base_url, depth - 1)
                .await;
        }

        Ok(())
    }
}
