use reqwest::{
    blocking::{Client, ClientBuilder},
    redirect::Policy,
};
use std::time::Duration;

pub fn build_client() -> Client {
    ClientBuilder::new().timeout(Duration::from_secs(5)).redirect(Policy::limited(3)).build().unwrap()
}
