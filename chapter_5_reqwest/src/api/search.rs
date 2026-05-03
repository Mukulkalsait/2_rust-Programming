use crate::{
    models::{SearchQuery, SearchResponse},
    utils::get_json,
};
use reqwest::blocking::Client;

pub fn search_request(client: &Client) -> Result<SearchResponse, Box<dyn std::error::Error>> {
    let res = client.get("http://localhost:3000/search").query(&[("q", "rust"), ("page", "1")]).send()?;
    // Y: expansion => http://localhost:3000/serach?q=rust&page=1
    get_json(res)
}

pub fn search_request2(client: &Client) -> Result<SearchResponse, Box<dyn std::error::Error>> {
    let query1 = SearchQuery { q: "mukul".to_string(), page: 2 };
    let res = client.get("http://localhost:3000/search").query(&query1).send().unwrap();
    get_json(res)
}
