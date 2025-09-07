use tiny_crawler1::crawler::Seed;
use tiny_crawler1::{crawler, errors::Errors};

#[tokio::main]
async fn main() -> Result<(), Errors> {
    let mut seed_urls: Vec<Seed> = Vec::with_capacity(10);
    let url = "https://news.ycombinator.com/".to_string();
    // seed_urls.push("https://github.com/forfd8960".to_string());
    seed_urls.push(Seed {
        url: url.clone(),
        base: "https://news.ycombinator.com/".to_string(),
    });

    if let Ok(my_crawler) = crawler::Crawler::new(seed_urls, 1, 1, "./data".to_string()) {
        if let Err(e) = my_crawler.crawl().await {
            eprintln!("failed to crawl: {}", e);
        } else {
            println!("success crawled: {}", url);
        }
    } else {
        eprintln!("create crawler faile");
    }

    Ok(())
}
