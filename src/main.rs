use tiny_crawler1::{crawler, errors::Errors};

#[tokio::main]
async fn main() -> Result<(), Errors> {
    let mut seed_urls: Vec<String> = Vec::with_capacity(10);
    // seed_urls.push("https://github.com/forfd8960".to_string());
    seed_urls.push("https://redis.io/docs/latest/commands/".to_string());

    let my_crawler = crawler::Crawler::new(seed_urls, 2, 1);
    // println!("my_crawler: {:?}", my_crawler);

    let res = my_crawler.crawl(2).await?;
    println!("success crawled data: {:?}", res);

    Ok(())
}
