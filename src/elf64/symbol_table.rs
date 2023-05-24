use crate::elf::{SHT_DYNSYM, SHT_SYMTAB};

use super::{header::Elf64SectionHeader, Error};

#[derive(Debug, Clone)]
pub struct SymbolTable<'a> {
    symbols: &'a [Symbol],
}

impl<'a> SymbolTable<'a> {
    pub fn parse(buf: &'a [u8], hdr: &Elf64SectionHeader) -> Result<SymbolTable<'a>, Error> {
        if hdr.sh_type != SHT_SYMTAB && hdr.sh_type != SHT_DYNSYM {
            return Err(Error::Message("section not a symbol table".to_string()));
        }

        let shbuf: &[u8] = hdr.get_section_buffer(buf)?;
        if shbuf.is_empty() {
            return Err(Error::Message("invalid symbol table".to_string()));
        }

        let ptr = shbuf.as_ptr() as *const Symbol;
        let entries = (hdr.sh_size / hdr.sh_entsize) as usize;
        let symbols: &'a [Symbol] = unsafe { std::slice::from_raw_parts(ptr, entries) };

        Ok(SymbolTable { symbols })
    }

    pub fn get_symbol(&'a self, index: usize) -> &'a Symbol {
        if index >= self.symbols.len() {
            panic!("invalid symbol index");
        }

        &self.symbols[index]
    }

    pub fn iter(&'a self) -> impl Iterator<Item = &'a Symbol> {
        self.symbols.iter()
    }
}

static_assertions::const_assert!(std::mem::size_of::<Symbol>() == 24);

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Symbol {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}
