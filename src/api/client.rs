use reqwest::Client as reqwest_client;
use std::fmt;
use crate::models::enums::Endpoint;

#[allow(dead_code)]
#[derive(Clone, Default, Debug)]
pub struct Client {
    http: reqwest_client,
    endpoint: Endpoint,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Credentials: {:?}\nClient: not available.",
            self.auth
        )
    }
}