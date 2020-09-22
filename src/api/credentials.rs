pub struct Credentials {
    pub access_token: String,
}

impl Default for Credentials {
    fn default() -> Credentials {
        Credentials {
            access_token: "".to_owned()
        }
    }
}

impl Credentials {
    pub fn new(token: String) -> Credentials {
        Credentials {
            access_token: token,
        }
    }
}