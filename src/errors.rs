use failure::Error;
use std;

pub type MessageBirdResult<T> = std::result::Result<T, MessageBirdError>;

#[derive(Debug, Fail)]
pub enum MessageBirdError {
    #[fail(display = "invalid json format: {}", chunk)]
    FormatError { chunk: String },

    #[fail(display = "service return code: {}", code)]
    Service { code: u32 },
}