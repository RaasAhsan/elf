pub mod header;
pub mod string_table;
pub mod symbol_table;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("open error: {0}")]
    Open(std::io::Error),
    #[error("read error: {0}")]
    Read(std::io::Error),
    #[error("invalid magic number")]
    InvalidMagicNumber,
    #[error("unexpected class")]
    UnsupportedClass,
    #[error("invalid class")]
    InvalidClass,
    #[error("invalid endianness")]
    InvalidEndianness,
    #[error("error: {0}")]
    Message(String),
}
