use crate::raw::SHT_DYNAMIC;

use super::{header::SectionHeader, Error};

#[derive(Debug, Clone)]
pub struct DynamicTable<'a> {
    entries: &'a [Dynamic],
}

impl<'a> DynamicTable<'a> {
    pub fn parse<A: AsRef<[u8]>>(
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

    pub fn get_entry(&'a self, index: usize) -> &'a Dynamic {
        if index >= self.entries.len() {
            panic!("invalid symbol index");
        }

        &self.entries[index]
    }

    pub fn iter(&'a self) -> impl Iterator<Item = &'a Dynamic> {
        self.entries.iter()
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Dynamic {
    pub d_tag: u64,
    pub d_value: u64,
}
