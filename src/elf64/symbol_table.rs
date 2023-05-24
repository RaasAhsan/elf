use crate::elf::{SHT_DYNSYM, SHT_SYMTAB};

use super::{
    header::{Elf64Headers, Elf64SectionHeader},
    string_table::StringTable,
    Error,
};

#[derive(Debug, Clone)]
pub struct SymbolTable<'a> {
    string_table: StringTable<'a>,
    symbols: &'a [Symbol],
}

impl<'a> SymbolTable<'a> {
    pub fn parse(
        buf: &'a [u8],
        elf: &Elf64Headers,
        hdr: &Elf64SectionHeader,
    ) -> Result<SymbolTable<'a>, Error> {
        if hdr.sh_type != SHT_SYMTAB && hdr.sh_type != SHT_DYNSYM {
            return Err(Error::Message("section not a symbol table".to_string()));
        }

        let shbuf: &[u8] = hdr.get_section_buffer(buf)?;
        if shbuf.is_empty() {
            return Err(Error::Message("invalid symbol table".to_string()));
        }

        let sh_link = hdr.sh_link;
        let strtab_hdr = elf.get_section_header_by_index(sh_link as usize).unwrap();
        let string_table = StringTable::parse(buf, strtab_hdr)?;

        let ptr = shbuf.as_ptr() as *const Symbol;
        let entries = (hdr.sh_size / hdr.sh_entsize) as usize;
        let symbols: &'a [Symbol] = unsafe { std::slice::from_raw_parts(ptr, entries) };

        Ok(SymbolTable {
            string_table,
            symbols,
        })
    }

    pub fn get_symbol(&'a self, index: usize) -> &'a Symbol {
        if index >= self.symbols.len() {
            panic!("invalid symbol index");
        }

        &self.symbols[index]
    }

    pub fn get_elf_symbol(&'a self, index: usize) -> ElfSymbol<'a> {
        if index >= self.symbols.len() {
            panic!("invalid symbol index");
        }

        let symbol = &self.symbols[index];

        let name_index = symbol.st_name;
        let name = self
            .string_table
            .get_string(name_index as usize)
            .to_str()
            .unwrap();
        let info = symbol.st_info;
        let other = symbol.st_other;
        let shndx = symbol.st_shndx;
        let value = symbol.st_value;
        let size = symbol.st_size;

        ElfSymbol {
            name,
            info,
            other,
            shndx,
            value,
            size,
        }
    }

    pub fn iter(&'a self) -> impl Iterator<Item = &'a Symbol> {
        self.symbols.iter()
    }
}

static_assertions::const_assert!(std::mem::size_of::<Symbol>() == 24);

/// Raw symbol representation
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

/// High-level symbol representation
#[derive(Debug, Clone, Copy)]
pub struct ElfSymbol<'a> {
    pub name: &'a str,
    pub info: u8,
    pub other: u8,
    pub shndx: u16,
    pub value: u64,
    pub size: u64,
}
