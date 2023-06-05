use crate::raw::SHT_DYNAMIC;

use super::{
    header::{ProgramHeader, SectionHeader},
    Error, DT_REL, DT_RELA, PT_DYNAMIC,
};

#[derive(Debug, Clone)]
pub struct DynamicTable<'a> {
    entries: &'a [Dynamic],
}

impl<'a> DynamicTable<'a> {
    pub fn parse_section<A: AsRef<[u8]>>(
        buf: &'a A,
        hdr: &SectionHeader,
    ) -> Result<DynamicTable<'a>, Error> {
        if hdr.sh_type != SHT_DYNAMIC {
            return Err(Error::Message("section not a dynamic table".to_string()));
        }

        let shbuf: &[u8] = hdr.get_section_buffer(buf)?;
        if shbuf.is_empty() {
            return Err(Error::Message("invalid dynamic table".to_string()));
        }

        let ptr = shbuf.as_ptr() as *const Dynamic;
        let entries = (hdr.sh_size / hdr.sh_entsize) as usize;
        let dyn_entries: &'a [Dynamic] = unsafe { std::slice::from_raw_parts(ptr, entries) };

        Ok(DynamicTable {
            entries: dyn_entries,
        })
    }

    /// Reads dynamic table from the ELF program header in virtual memory.
    /// Precondition: all loadable segments have been mapped into virtual memory already.
    pub fn parse_segment(base_addr: usize, hdr: &ProgramHeader) -> Result<DynamicTable<'a>, Error> {
        if hdr.get_type() != PT_DYNAMIC {
            return Err(Error::Message("header not PT_DYNAMIC".to_string()));
        }

        let ptr = (base_addr + hdr.get_vaddr() as usize) as *const Dynamic;
        let entry_count = hdr.get_memsz() as usize / std::mem::size_of::<Dynamic>();
        let entries: &'a [Dynamic] = unsafe { std::slice::from_raw_parts(ptr, entry_count) };

        Ok(DynamicTable { entries })
    }

    pub fn has_relocations(&self) -> bool {
        self.find_entry(DT_RELA).is_some()
    }

    pub fn get_entry(&'a self, index: usize) -> &'a Dynamic {
        if index >= self.entries.len() {
            panic!("invalid symbol index");
        }

        &self.entries[index]
    }

    pub fn find_entry(&'a self, tag: u64) -> Option<&'a Dynamic> {
        self.entries.iter().find(|t| t.get_tag() == tag)
    }

    pub fn iter(&'a self) -> impl Iterator<Item = &'a Dynamic> {
        self.entries.iter()
    }
}

static_assertions::const_assert!(std::mem::size_of::<Dynamic>() == 16);

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Dynamic {
    d_tag: u64,
    d_value: u64,
}

impl Dynamic {
    pub fn get_tag(&self) -> u64 {
        self.d_tag
    }

    pub fn get_value(&self) -> u64 {
        self.d_value
    }
}
