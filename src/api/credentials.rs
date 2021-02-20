#[derive(Debug, Clone)]
pub struct Credentials {
    pub access_token: String,
}

impl Credentials {
    #[allow(dead_code)]
    #[allow(clippy::clippy::must_use_candidate)]
    pub fn new(token: String) -> Credentials {
        Credentials {
            access_token: token,
        }
    }

    #[allow(clippy::clippy::must_use_candidate)]
    pub fn default() -> Credentials {
        Credentials {
            access_token: String::new(),
        }
    }
}
