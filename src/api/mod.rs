mod auth;
mod client;

mod endpoints {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum EndpointKind {
        Sandbox,
        Production,
    }

    impl EndpointKind {
        fn as_str(self) -> &'static str {
            match self {
                Self::Sandbox => "https://api-sandbox.starlingbank.com",
                Self::Production => "https://api.starlingbank.com",
            }
        }
    }

    impl Default for EndpointKind {
        fn default() -> Self {
            Self::Sandbox
        }
    }

    impl ToString for EndpointKind {
        fn to_string(&self) -> String {
            self.as_str().to_string()
        }
    }
}

mod authentication_kinds {
    use super::auth::{PATHolder, OAuthHolder};

    #[derive(Debug, Clone)]
    pub enum AuthenticationKind {
        OAuthToken(OAuthHolder),
        PersonalAccessToken(PATHolder),
        Unconfigured,
    }

    impl ToString for AuthenticationKind {
        fn to_string(&self) -> String {
            match self {
                Self::PersonalAccessToken(pat_holder) => format!("PAT: {:?}", pat_holder),
                Self::OAuthToken(oauth_holder) => format!("OAuth: {:?}", oauth_holder),
                Self::Unconfigured => format!("Unconfigured authentication type."),
            }
        }
    }
}

pub use self::auth::{Authenticator, OAuthHolder, PATHolder};
pub use self::client::Client;
pub use self::endpoints::EndpointKind;
pub use self::authentication_kinds::AuthenticationKind;