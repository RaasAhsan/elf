use enumflags2::bitflags;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone)]
pub enum ObjectClass {
    Elf32,
    Elf64,
}

impl ObjectClass {
    pub fn from_u8(value: u8) -> Option<ObjectClass> {
        match value {
            0x01 => Some(ObjectClass::Elf32),
            0x02 => Some(ObjectClass::Elf64),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ObjectData {
    Little,
    Big,
}

impl ObjectData {
    pub fn from_u8(value: u8) -> Option<ObjectData> {
        match value {
            0x01 => Some(ObjectData::Little),
            0x02 => Some(ObjectData::Big),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ObjectType {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
    Os(u16),
    Proc(u16),
}

impl ObjectType {
    pub fn from_u16(value: u16) -> Option<ObjectType> {
        match value {
            0x00 => Some(ObjectType::None),
            0x01 => Some(ObjectType::Rel),
            0x02 => Some(ObjectType::Exec),
            0x03 => Some(ObjectType::Dyn),
            0x04 => Some(ObjectType::Core),
            0xFE00..=0xFEFF => Some(ObjectType::Os(value)),
            0xFF00..=0xFFFF => Some(ObjectType::Proc(value)),
            _ => None,
        }
    }
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
