use std::{error::Error, fmt::Display, str::Utf8Error};

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    InvalidEntryIndex(usize),
    MissingEntryIndex,

    InvalidUtf8LeadingByte(u8),
    InvalidUtf8Length { expected: usize, actual: usize },
    InvalidUtf8Character,
}

impl Error for DecodeError {}

impl From<Utf8Error> for DecodeError {
    fn from(_error: Utf8Error) -> Self {
        DecodeError::InvalidUtf8Character
    }
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "decoding error: {:?}", self)
    }
}
