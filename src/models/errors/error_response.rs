use super::error_detail::ErrorDetail;

pub struct ErrorResponse {
    pub errors: Vec<ErrorDetail>,
    pub success: bool,
}