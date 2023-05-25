use std::fmt::Display;

use enumflags2::bitflags;
use num_derive::{FromPrimitive, ToPrimitive};

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

#[derive(Debug, Clone, ToPrimitive, FromPrimitive)]
pub enum ObjectType {
    None = 0x0,
    Rel = 0x1,
    Exec = 0x2,
    Dyn = 0x3,
    Core = 0x4,
}

#[derive(Debug, Clone)]
pub enum SectionType {
    Null = 0x0,
    Progbits = 0x1,
    Symtab = 0x2,
    Strtab = 0x3,
    Rela = 0x4,
    Hash = 0x5,
    Dynamic = 0x6,
    Note = 0x7,
    Nobits = 0x8,
    Rel = 0x9,
    Shlib = 0xa,
    Dynsym = 0xb,
    InitArray = 0xe,
    FiniArray = 0xf,
    PreinitArray = 0x10,
    Group = 0x11,
    SymtabShndx = 0x12,
    ShtNum = 0x13,
    // Os(u32),
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