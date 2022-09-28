use crate::Authenticator;
use crate::EndpointKind;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Client {
    http: attohttpc::Session,
    endpoint: EndpointKind,
    auth: Authenticator,
}
