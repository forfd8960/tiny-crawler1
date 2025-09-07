use crate::errors::Errors;
use crate::is_valid_url;
use crate::parser::ContentParser;
use crate::storage::DataStore;
use crate::storage::Page;
use crate::url_queue::Link;
use crate::url_queue::URLQueue;
use futures::future::join_all;
use reqwest::Client;
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
    ) -> Result<Self, Errors> {
        let store = DataStore::new(store_dir)?;

        Ok(Self {
            seed_urls,
            max_depth,
            max_worker,
            parser: Arc::new(ContentParser::new()),
            page_store: Arc::new(store),
            url_queue: Arc::new(URLQueue::new(max_depth, 100 as usize)),
        })
    }

    pub async fn crawl(&self) -> Result<(), Errors> {
        for seed in &self.seed_urls {
            let link = Link {
                url: seed.url.clone(),
                base: seed.base.clone(),
                depth: 1,
            };
            let _ = self.url_queue.add_url(&link).await?;
        }

        let mut workers = Vec::new();

        let max_depth = self.max_depth;

        for _ in 0..self.max_worker {
            let url_queue = Arc::clone(&self.url_queue);
            let page_store = Arc::clone(&self.page_store);
            let content_parser = Arc::clone(&self.parser);

            let crawl_task = task::spawn(async move {
                loop {
                    if let Some(target) = url_queue.get_next_link().await {
                        println!(
                            "{} [crawler]crawling {} {}",
                            "-".repeat(10),
                            target.url,
                            "-".repeat(10)
                        );
                        if target.depth > max_depth {
                            break;
                        }

                        if let Err(e) =
                            process_crawl(&content_parser, &target, &page_store, &url_queue).await
                        {
                            println!("failed to crawl: {:?}, {}", target, e);
                        }
                    } else {
                        println!("No url in the Queue");
                        break;
                    }
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
) -> Result<(), Errors> {
    let client = reqwest::Client::new();
    let res_page = retrieve_page(
        &client,
        &content_parser,
        &target.url,
        &target.base,
        target.depth,
    )
    .await?;

    let _ = page_store.save_page(&res_page)?;
    for url in &res_page.links {
        if !is_valid_url(&url) {
            eprintln!("{} is not valid URL", url);
            continue;
        }

        let link = Link {
            url: url.clone(),
            base: target.base.clone(),
            depth: res_page.depth + 1,
        };

        println!("enqueue url: {:?} into URL Queue", link);
        if let Err(e) = url_queue.add_url(&link).await {
            eprintln!("failed to add link: {}, {:?}", url, e);
        }
    }

    Ok(())
}

pub async fn retrieve_page(
    client: &Client,
    parser: &ContentParser,
    url_str: &str,
    base_url: &str,
    depth: usize,
) -> Result<Page, Errors> {
    let res = client.get(url_str).send().await?;
    let resp_content = res.text().await?;
    let page = parser.parse(&resp_content, base_url, depth)?;

    Ok(page)
}

#[cfg(test)]
mod tests {
    use crate::is_valid_url;
    use url::Url;

    #[test]
    fn test_validate_url() {
        let url = "https:item?id=45109927";
        assert!(is_valid_url(url));

        let res = Url::parse(url);
        assert!(res.is_ok());

        let res1 = Url::parse("example.com");
        println!("res1: {:?}", res1);
        assert!(res1.is_ok());
    }
}
