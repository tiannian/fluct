#[derive(Debug)]
pub enum EngineError {}

pub type EngineResult<T> = std::result::Result<T, EngineError>;
