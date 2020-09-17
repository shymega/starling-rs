#[derive(Debug)]
pub struct Credentials {
    pub access_token: String,
}

impl Credentials {
    pub fn new(token: String) -> Credentials {
        Credentials {
            access_token: token,
        }
    }
}