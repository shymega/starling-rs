use crate::api::Endpoint;
use crate::Authenticator;

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Client {
    http: attohttpc::Session,
    endpoint: Endpoint,
    auth: Authenticator
}
