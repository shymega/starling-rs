use crate::Authenticator;
use crate::EndpointKind;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Client {
    endpoint: EndpointKind,
    auth: Authenticator,
}
