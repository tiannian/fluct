#[derive(Debug)]
pub enum ApiError {}

pub type ApiResult<T> = std::result::Result<T, ApiError>;
