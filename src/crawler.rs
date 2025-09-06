use crate::errors::Errors;
use crate::parser::ContentParser;
use crate::storage::DataStore;
use crate::storage::Page;
use crate::url_queue::Link;
use crate::url_queue::URLQueue;
use futures::future::join_all;
use reqwest::Client;
use std::path::Path;
use std::sync::Arc;
use tokio::task;

#[derive(Debug)]
pub struct Seed {
    pub url: String,
    pub base: String,
}

#[derive(Debug)]
pub struct Crawler {
    seed_urls: Vec<Seed>,
    max_depth: usize,
    max_worker: usize,
    parser: Arc<ContentParser>,
    page_store: Arc<DataStore>,
    url_queue: Arc<URLQueue>,
}

impl Crawler {
    pub fn new(
        seed_urls: Vec<Seed>,
        max_depth: usize,
        max_worker: usize,
        store_dir: String,
    ) -> Self {
        Self {
            seed_urls,
            max_depth,
            max_worker,
            parser: Arc::new(ContentParser::new()),
            page_store: Arc::new(DataStore::new(store_dir)),
            url_queue: Arc::new(URLQueue::new(max_depth, 100 as usize)),
        }
    }

    pub async fn crawl(&self, depth: usize) -> Result<(), Errors> {
        if depth > self.max_depth {
            return Err(Errors::InvalidDepth(depth, self.max_depth));
        }

        for seed in &self.seed_urls {
            let link = Link {
                url: seed.url.clone(),
                depth: 0,
            };
            let _ = self.url_queue.add_url(&link).await?;
        }

        let mut workers = Vec::new();

        for _ in 0..self.max_worker {
            let url_queue = Arc::clone(&self.url_queue);
            let page_store = Arc::clone(&self.page_store);
            let content_parser = Arc::clone(&self.parser);

            let crawl_task = task::spawn(async move {
                if let Some(target) = url_queue.get_next_link().await {
                    process_crawl(&content_parser, &target, &page_store, &url_queue).await;
                }
            });

            workers.push(crawl_task);
        }

        let _ = join_all(workers).await;

        Ok(())
    }
}

pub async fn process_crawl(
    content_parser: &ContentParser,
    target: &Link,
    page_store: &DataStore,
    url_queue: &URLQueue,
) {
    let client = reqwest::Client::new();
    let res_page = retrieve_page(&client, &content_parser, &target.url, target.depth).await;

    if res_page.is_err() {
        eprintln!("failed to retrieve page: {}", target.url);
        return;
    }

    let page = &res_page.unwrap();
    let store_page = page_store.save_page(page);
    if store_page.is_err() {
        eprintln!(
            "failed to store page: {}, {:?}",
            target.url,
            store_page.err()
        );
        return;
    }

    for url in &page.links {
        let link = Link {
            url: url.clone(),
            depth: page.depth,
        };
        if let Err(e) = url_queue.add_url(&link).await {
            eprintln!("failed to add link: {}, {:?}", url, e);
        }
    }
}

pub async fn retrieve_page(
    client: &Client,
    parser: &ContentParser,
    url_str: &str,
    depth: usize,
) -> Result<Page, Errors> {
    let res = client.get(url_str).send().await?;
    let resp_content = res.text().await?;
    let parts: Vec<&str> = url_str.split("/").collect();
    let page = parser.parse(&resp_content, parts[0], depth)?;

    Ok(page)
}
