use scraper::{Html, Selector};

// https://blog.logrocket.com/web-scraping-rust/
// Do a request for the given URL, with a minimum time between requests
// to avoid overloading the server.
pub fn do_throttled_request(url: &str) -> Result<String, Error> {
    // See the real code for the throttling - it's omitted here for clarity
    let response = reqwest::blocking::get(url)?;
    response.text()
}

// let document = Html::parse_document(&body);
// // Find the table with the most rows
// let main_table = document.select(&TABLE).max_by_key(|table| {
//     table.select(&TR).count()
// }).expect("No tables found in document?");

fn scrape_subreddit(url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Make an HTTP GET request to the subreddit URL
    let html = reqwest::blocking::get(url)?.text()?;

    // Parse the HTML content using the scraper crate
    let document = Html::parse_document(&html);

    // Use a CSS selector to find the titles of the top posts
    let selector = Selector::parse(".Post__title").unwrap();
    let mut titles = Vec::new();
    for element in document.select(&selector) {
        let title = element.text().collect::<String>();
        titles.push(title);
    }

    Ok(titles)
}
