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

pub const PT_DYNAMIC: u32 = 0x02;

pub const DT_NULL: u64 = 0;
pub const DT_NEEDED: u64 = 1;
pub const DT_PTRRELSZ: u64 = 2;
pub const DT_PLTGOT: u64 = 3;
pub const DT_STRTAB: u64 = 5;
pub const DT_SYMTAB: u64 = 6;
pub const DT_RELA: u64 = 7;
pub const DT_RELASZ: u64 = 8;
pub const DT_RELAENT: u64 = 9;
pub const DT_INIT: u64 = 12;
pub const DT_REL: u64 = 17;
pub const DT_RELSZ: u64 = 18;
pub const DT_RELENT: u64 = 19;

pub const R_AARCH64_RELATIV: u32 = 0x403;

pub type SymbolTableIndex = u32;

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
