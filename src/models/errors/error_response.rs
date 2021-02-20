use super::error_detail::ErrorDetail;
#[derive(Debug)]
pub struct ErrorResponse {
    pub errors: Vec<ErrorDetail>,
    pub success: bool,
}
