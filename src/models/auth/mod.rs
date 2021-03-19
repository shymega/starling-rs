#[derive(Debug)]
pub struct Authenticator {
    oauth_holder: OAuthHolder,
    api_key: String,
    api_secret: String,
    personal_access_token: String,
}

#[derive(Debug)]
pub(crate) struct OAuthHolder {
}
