pub mod dynamic;
pub mod header;
pub mod relocation;
pub mod string;
pub mod symbol;

pub const ELF_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

pub const ELF_CLASS_32: u8 = 0x01;
pub const ELF_CLASS_64: u8 = 0x02;

pub const ELF_DATA_LITTLE: u8 = 0x01;
pub const ELF_DATA_BIG: u8 = 0x02;

pub const SHT_NULL: u32 = 0x00;
pub const SHT_PROGBITS: u32 = 0x01;
pub const SHT_SYMTAB: u32 = 0x02;
pub const SHT_STRTAB: u32 = 0x03;
pub const SHT_RELA: u32 = 0x04;
pub const SHT_DYNAMIC: u32 = 0x06;
pub const SHT_DYNSYM: u32 = 0x0B;

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
