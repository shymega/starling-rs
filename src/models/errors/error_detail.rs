#[derive(Debug)]
pub struct ErrorDetail {
    pub message: String,
}

impl ErrorDetail {
    pub fn new(msg: &str) -> Self {
        Self {
            message: String::from(msg)
        }
    }
}

