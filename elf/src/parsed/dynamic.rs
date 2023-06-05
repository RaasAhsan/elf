use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, ToPrimitive, FromPrimitive)]
pub enum DynamicTag {
    Null = 0,
    Needed = 1,
    PtrRelSz = 2,
    PltGot = 3,
    StrTab = 5,
    SymTab = 6,
    Rela = 7,
    RelaSz = 8,
    RelaEnt = 9,
    StrSz = 10,
    SymEnt = 11,
    Init = 12,
    Fini = 13,
    Rel = 17,
    RelSz = 18,
    RelEnt = 19,
    PltRel = 20,
    Debug = 21,
    JmpRel = 23,
    InitArray = 25,
    FiniArray = 26,
    InitArraySz = 27,
    FiniArraySz = 28,
}
