use tiny_crawler1::crawler::Seed;
use tiny_crawler1::{crawler, errors::Errors};

#[tokio::main]
async fn main() -> Result<(), Errors> {
    let mut seed_urls: Vec<Seed> = Vec::with_capacity(10);
    let url = "https://www.henrikkarlsson.xyz/p/attention".to_string();
    seed_urls.push(Seed {
        url: url.clone(),
        base: "https://www.henrikkarlsson.xyz/".to_string(),
    });

    if let Ok(my_crawler) = crawler::Crawler::new(seed_urls, 1, 1, "./data".to_string()) {
        // 启动爬虫任务
        match my_crawler.crawl().await {
            Ok(_) => println!("Successfully crawled"),
            Err(e) => eprintln!("Failed to crawl: {}", e),
        }
    } else {
        eprintln!("Failed to create crawler");
    }

    Ok(())
}
