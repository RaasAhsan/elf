use crate::raw::{self};

use super::{Address, Error};

#[derive(Clone, Debug)]
pub struct Header {
    pub class: ObjectClass,
    pub data: ObjectData,
    pub r#type: ObjectType,
    pub machine: u16,
    pub entrypoint: Address,
}

impl Header {
    pub fn from_raw(hdr: &raw::header::FileHeader) -> Result<Self, Error> {
        let class = ObjectClass::from_u8(hdr.e_ident.class).ok_or(Error::InvalidElf)?;
        let data = ObjectData::from_u8(hdr.e_ident.data).ok_or(Error::InvalidElf)?;
        let r#type = ObjectType::from_u16(hdr.e_type).ok_or(Error::InvalidElf)?;

        Ok(Header {
            class,
            data,
            r#type,
            machine: hdr.e_machine,
            entrypoint: hdr.e_entry,
        })
    }
}

impl<'a> TryFrom<&'a raw::header::FileHeader> for Header {
    type Error = Error;

    fn try_from(hdr: &'a raw::header::FileHeader) -> Result<Self, Self::Error> {
        Header::from_raw(hdr)
    }
}

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
