/// Enum (hacky!) of the two API endpoints for the Starling API.

#[non_exhaustive]
#[derive(Clone, Default, Debug, Copy)]
pub struct Endpoint;

impl Endpoint {
    pub const SANDBOX: &'static str = "https://oauth-sandbox.starlingbank.com";
    pub const PRODUCTION: &'static str = "https://oauth.starlingbank.com";
}
