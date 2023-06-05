use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, ToPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum RelocationType {
    AArch64Relative = 0x403,
}
