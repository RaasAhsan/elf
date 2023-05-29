use num_derive::{FromPrimitive, ToPrimitive};

use self::header::ElfHeader;

pub mod header;
pub mod section;
pub mod segment;
pub mod symbol;

pub type Address = u64;

#[derive(Clone, Debug)]
pub struct Elf {
    pub header: ElfHeader,
    pub program_headers: Vec<u8>,
    pub section_headers: Vec<u8>,
}

#[derive(Debug, Clone, ToPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum SymbolType {
    NoType = 0,
    Object = 1,
    Func = 2,
    Section = 3,
    File = 4,
    Common = 5,
    Loos = 10,
    Hios = 12,
    Loproc = 13,
    Hiproc = 15,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid elf")]
    InvalidElf,
}
