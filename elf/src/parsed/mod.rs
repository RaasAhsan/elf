use self::{header::Header, section::SectionHeader, segment::ProgramHeader};

pub mod header;
pub mod section;
pub mod segment;
pub mod symbol;

pub type Address = u64;

#[derive(Clone, Debug)]
pub struct Elf {
    pub header: Header,
    // TODO: what is the most precise representation of program and section headers?
    // Programs are ultimately composed of many sections, so it doesn't make sense to duplicate that data
    pub program_headers: Vec<ProgramHeader>,
    pub section_headers: Vec<SectionHeader>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid elf")]
    InvalidElf,
}
