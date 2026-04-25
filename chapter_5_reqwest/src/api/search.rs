use crate::{models::SearchResponse, utils::get_json};
use reqwest::blocking::Client;

pub fn search_request(client: &Client) -> Result<SearchResponse, Box<dyn std::error::Error>> {
    let res = client.get("http://localhost:3000/search").query(&[("q", "rust"), ("page", "1")]).send()?;
    get_json(res)
}
