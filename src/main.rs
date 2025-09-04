use tiny_crawler1::crawler::Seed;
use tiny_crawler1::storage::DataStore;
use tiny_crawler1::{crawler, errors::Errors};

use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Errors> {
    let mut seed_urls: Vec<Seed> = Vec::with_capacity(10);
    // seed_urls.push("https://github.com/forfd8960".to_string());
    seed_urls.push(Seed {
        url: "https://www.buildwithrs.dev/blog".to_string(),
        base: "https://www.buildwithrs.dev".to_string(),
    });

    let my_crawler = crawler::Crawler::new(seed_urls, 2, 1);
    // println!("my_crawler: {:?}", my_crawler);

    let res = my_crawler.crawl(2).await?;

    let path = Path::new("./data");
    let store = DataStore::new(path);
    for (url, resp) in res.data {
        store.save_data(&url.replace("https://", "").replace("/", "_"), &resp.data)?;
    }
    Ok(())
}
