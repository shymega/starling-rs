use crate::AuthenticationKind;

pub type Authenticator = AuthenticationKind;

#[derive(Debug, Clone)]
pub struct OAuthHolder {}

#[derive(Debug, Clone)]
pub struct PATHolder {}
