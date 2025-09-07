use crate::{errors::Errors, is_valid_url, storage::Page};
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct ContentParser {
    pub title_seclector: Selector,
    pub link_selector: Selector,
}

#[derive(Debug)]
pub struct ParsedContent {
    pub links: Vec<String>,
    pub title: String,
}

impl ContentParser {
    pub fn new() -> Self {
        Self {
            title_seclector: Selector::parse("title").unwrap(),
            link_selector: Selector::parse("a[href]").unwrap(),
        }
    }

    pub fn parse(&self, content: &str, _: &str, depth: usize) -> Result<Page, Errors> {
        let doc = Html::parse_document(&content);

        let title = doc
            .select(&self.title_seclector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();

        let links = doc
            .select(&self.link_selector)
            .filter_map(|el| el.value().attr("href").filter(|url| is_valid_url(url)))
            .map(|v| v.to_string())
            .collect();

        Ok(Page {
            title: title,
            content: content.to_string(),
            links,
            depth,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ContentParser;

    #[test]
    fn test_parse_html() {
        let html = r#"<html>
        <title>Hello</title>
        <body>
        <a href="https://github.com/alex">Alex Github</a>
        <a href="/logout">Logout</a>
        </body>
        </html>"#;
        let p = ContentParser::new();
        let res = p.parse(html, "https://github.com", 0);
        assert!(res.is_ok());

        let data = res.unwrap();
        println!("title: {}", data.title);
        println!("links: {:?}", data.links);

        assert_eq!(data.title, "Hello".to_string());
        assert_eq!(
            data.links,
            [
                "https://github.com/alex".to_string(),
                "https://github.com/logout".to_string()
            ]
            .to_vec()
        );
    }
}
