use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, ToPrimitive, FromPrimitive)]
pub enum DynamicTag {
    Null = 0x00,
    Needed = 0x01,
    PtrRelSz = 0x02,
    StrTab = 0x05,
    SymTab = 0x07,
    Rela = 0x08,
    RelaSz = 0x09,
    RelaEnt = 10,
    Rel = 17,
    RelSz = 18,
    RelEnt = 19,
}
