#[derive(Debug)]
pub enum MindError {
    Stub,
}

pub type Result<T> = std::result::Result<T, MindError>;
