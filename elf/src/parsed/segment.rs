use enumflags2::{bitflags, BitFlags};
use num_derive::{FromPrimitive, ToPrimitive};

use super::Address;

#[derive(Debug, Clone)]
pub struct ProgramHeader {
    pub r#type: SegmentType,
    pub flags: BitFlags<SegmentFlag>,
    pub offset: u64,
    pub vaddr: Address,
    pub paddr: Address,
    pub filesz: u64,
    pub memsz: u64,
    pub align: u64,
    // TODO: how do we refer to the contents? byte array, parsed format, etc? redundancy in tagging
}

#[derive(Debug, Clone, ToPrimitive, FromPrimitive)]
pub enum SegmentType {
    Null = 0x0,
    Load = 0x1,
    Dynamic = 0x2,
    Interp = 0x3,
    Note = 0x4,
    Shlib = 0x5,
    Phdr = 0x6,
    Tls = 0x7,
}

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SegmentFlag {
    Execute = 0b001,
    Write = 0b010,
    Read = 0b100,
}

impl SegmentFlag {
    pub fn name(&self) -> &'static str {
        match self {
            SegmentFlag::Execute => "E",
            SegmentFlag::Write => "W",
            SegmentFlag::Read => "R",
        }
    }
}
