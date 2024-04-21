#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Error {
    text: String,
    mach_code: ErrorCode,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ErrorCode {
    NotAuthenticated,
    NotFound,
}

pub type Result<T> = core::result::Result<T, Error>;
