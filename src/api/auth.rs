use crate::AuthenticationKind;

pub type Authenticator = AuthenticationKind;

#[derive(Debug, Default, Clone, Copy)]
pub struct OAuthHolder {}

#[derive(Debug, Default, Clone, Copy)]
pub struct PATHolder {}
