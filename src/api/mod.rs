mod auth;
mod client;

pub mod endpoints {
    //! Enum (hacky!) of the two API endpoints for the Starling API.

    #[non_exhaustive]
    #[derive(Clone, Default, Debug, Copy)]
    pub struct Endpoint;

    impl Endpoint {
        pub const SANDBOX: &'static str =
            "https://api-sandbox.starlingbank.com";
        pub const PRODUCTION: &'static str =
            "https://api.starlingbank.com";
    }
}

pub use self::auth::{Authenticator, OAuthHolder};
pub use self::client::Client;
pub use endpoints::Endpoint;
