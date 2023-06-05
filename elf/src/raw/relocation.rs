use crate::raw::{DT_RELA, DT_RELAENT, DT_RELASZ, SHT_RELA};

use super::{dynamic::DynamicTable, header::SectionHeader, Error};

// TODO: we can probably write a generic table for a fixed type, or write a macro

#[derive(Debug, Clone)]
pub struct RelocationTable<'a, R: Relocation> {
    relocs: &'a [R],
}

impl<'a, R: Relocation> RelocationTable<'a, R> {
    pub fn parse_section_header<A: AsRef<[u8]>>(
        buf: &'a A,
        hdr: &SectionHeader,
    ) -> Result<RelocationTable<'a, R>, Error> {
        if hdr.sh_type != SHT_RELA {
            return Err(Error::Message("section not a relocation table".to_string()));
        }

        let shbuf: &[u8] = hdr.get_section_buffer(buf)?;
        if shbuf.is_empty() {
            return Err(Error::Message("invalid relocation table".to_string()));
        }

        let ptr = shbuf.as_ptr() as *const R;
        let entries = (hdr.sh_size / hdr.sh_entsize) as usize;
        let relocs: &'a [R] = unsafe { std::slice::from_raw_parts(ptr, entries) };

        Ok(RelocationTable { relocs })
    }

    /// Reads relocation table from the dynamic table.
    /// Panics if no relocations present.
    pub fn parse_rela_dynamic(
        base: usize,
        dynamic: &DynamicTable,
    ) -> Result<RelocationTable<'a, Rela>, Error> {
        let rel_addr = dynamic.find_entry(DT_RELA).unwrap();
        let size = dynamic.find_entry(DT_RELASZ).unwrap();
        let ent_size = dynamic.find_entry(DT_RELAENT).unwrap();
        // TODO: assert entry size?

        let entry_count = (size.get_value() / ent_size.get_value()) as usize;

        let ptr = (base + rel_addr.get_value() as usize) as *const Rela;
        let relocs: &'a [Rela] = unsafe { std::slice::from_raw_parts(ptr, entry_count) };

        Ok(RelocationTable { relocs })
    }

    pub fn get_relocation(&'a self, index: usize) -> &'a R {
        if index >= self.relocs.len() {
            panic!("invalid symbol index");
        }

        &self.relocs[index]
    }

    pub fn iter(&'a self) -> impl Iterator<Item = &'a R> {
        self.relocs.iter()
    }
}

static_assertions::const_assert!(std::mem::size_of::<Rel>() == 16);
static_assertions::const_assert!(std::mem::size_of::<Rela>() == 24);

pub trait Relocation {}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Rel {
    pub r_offset: u64,
    pub r_info: u64,
}

impl Relocation for Rel {}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Rela {
    /// Location at which the relocation must be applied.
    r_offset: u64,
    /// Symbol table index and type of relocation
    r_info: u64,
    /// Constant addend for applying the relocation
    r_addend: i64,
}

impl Rela {
    pub fn get_offset(&self) -> u64 {
        self.r_offset
    }

    pub fn get_info(&self) -> u64 {
        self.r_info
    }

    pub fn get_symbol(&self) -> u32 {
        (self.r_info >> 32) as u32
    }

    pub fn get_type(&self) -> u32 {
        (self.r_info & 0xffffffff) as u32
    }

    pub fn get_addend(&self) -> i64 {
        self.r_addend
    }
}

impl Relocation for Rela {}
