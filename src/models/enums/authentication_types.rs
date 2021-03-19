//! This module

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum AuthenticationType {
    OAUTH_TOKEN,
    PERSONAL_ACCESS_TOKEN,
    UNDEFINED,
}

impl Default for AuthenticationType {
    fn default() -> AuthenticationType {
        AuthenticationType::UNDEFINED
    }
}