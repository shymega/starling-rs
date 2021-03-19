use reqwest::Client as reqwest_client;
use crate::models::enums::Endpoint;

#[allow(dead_code)]
#[derive(Clone, Default, Debug)]
pub struct Client {
    http: reqwest_client,
    endpoint: Endpoint,
}
