use num_derive::{FromPrimitive, ToPrimitive};

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
