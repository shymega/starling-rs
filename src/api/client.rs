use crate::api::credentials::Credentials;

use reqwest::Client as reqwest_client;
use std::fmt;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Client {
    pub auth: Credentials,
    http: reqwest_client,
}

impl Client {
    #[allow(clippy::clippy::must_use_candidate)]
    pub fn default() -> Client {
        Client {
            auth: Credentials::default(),
            http: reqwest_client::default(),
        }
    }
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