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
