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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid elf")]
    InvalidElf,
}
