use std::{collections::HashMap, ffi::CStr};

use crate::elf::ELF_MAGIC;

/// A raw representation of the headers in an ELF file.
/// This includes the ELF headers, the program headers, and
/// the section headers.
pub struct Elf64Headers<'a> {
    pub header: &'a Elf64FileHeader,
    pub program_headers: &'a [Elf64ProgramHeader],
    pub section_headers: &'a [Elf64SectionHeader],
    pub sh_names: StringTable<'a>,
    pub sh_by_name: HashMap<String, &'a Elf64SectionHeader>,
}

/// We must assume a byte-for-byte representation because ELF files can be deployed
/// to both little-endian/big-endian, 32-bit/64-bit computers.
impl<'a> Elf64Headers<'a> {
    pub fn parse(buf: &'a [u8]) -> Result<Elf64Headers<'a>, Error> {
        let header = Elf64FileHeader::from_buffer(buf)?;

        if header.e_ident.magic != ELF_MAGIC {
            return Err(Error::InvalidMagicNumber);
        }

        // Assert 64-bit ELF
        if header.e_ident.class != 0x02 {
            return Err(Error::InvalidClass);
        }

        // Assert little endianness
        if header.e_ident.data != 0x01 {
            return Err(Error::InvalidEndianness);
        }

        let program_headers = Elf64ProgramHeader::parse_headers(buf, header)?;
        let section_headers: &[Elf64SectionHeader] =
            Elf64SectionHeader::parse_headers(buf, header)?;

        // TODO: validate
        let sh_names_header = &section_headers[header.e_shstrndx as usize];
        let sh_names = StringTable::parse(buf, sh_names_header)?;

        let mut sections_by_name = HashMap::new();
        for s in section_headers.iter() {
            let name = sh_names.get_string(s.sh_name as usize);
            sections_by_name.insert(name.to_str().unwrap().to_string(), s);
        }

        Ok(Self {
            header,
            program_headers,
            section_headers,
            sh_names,
            sh_by_name: sections_by_name,
        })
    }

    pub fn get_header(&self, name: &str) -> Option<&Elf64SectionHeader> {
        self.sh_by_name.get(name).copied()
    }
}

static_assertions::const_assert!(std::mem::size_of::<Elf64FileHeader>() == 64);
static_assertions::const_assert!(std::mem::size_of::<Elf64Ident>() == 16);
static_assertions::const_assert!(std::mem::size_of::<Elf64ProgramHeader>() == 0x38);
static_assertions::const_assert!(std::mem::size_of::<Elf64SectionHeader>() == 0x40);

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Elf64FileHeader {
    pub e_ident: Elf64Ident,
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl Elf64FileHeader {
    pub fn from_buffer<'a>(buf: &'a [u8]) -> Result<&'a Elf64FileHeader, Error> {
        if buf.len() < std::mem::size_of::<Elf64FileHeader>() {
            return Err(Error::Message("invalid header length".to_string()));
        }

        let ptr = buf.as_ptr() as *const Elf64FileHeader;
        let header: &'a Elf64FileHeader = unsafe { &*ptr };
        Ok(header)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Elf64Ident {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
    pub _padding: [u8; 7],
}

#[derive(Debug, Clone)]
#[repr(C, packed)]
pub struct Elf64ProgramHeader {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

impl Elf64ProgramHeader {
    pub fn parse_headers<'a>(
        buf: &'a [u8],
        header: &Elf64FileHeader,
    ) -> Result<&'a [Elf64ProgramHeader], Error> {
        let offset = header.e_phoff as usize;
        let length = (header.e_phentsize as usize) * (header.e_phnum as usize);

        let phbuf = &buf[offset..(offset + length)];
        if phbuf.len() < length {
            return Err(Error::Message("invalid program headers length".to_string()));
        }

        let ptr = phbuf.as_ptr() as *const Elf64ProgramHeader;
        let pheader: &'a [Elf64ProgramHeader] =
            unsafe { std::slice::from_raw_parts(ptr, header.e_phnum as usize) };
        Ok(pheader)
    }
}

#[derive(Debug, Clone)]
#[repr(C, packed)]
pub struct Elf64SectionHeader {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

impl Elf64SectionHeader {
    pub fn parse_headers<'a>(
        buf: &'a [u8],
        header: &Elf64FileHeader,
    ) -> Result<&'a [Elf64SectionHeader], Error> {
        let offset = header.e_shoff as usize;
        let length = (header.e_shentsize as usize) * (header.e_shnum as usize);

        let shbuf = &buf[offset..(offset + length)];
        if shbuf.len() < length {
            return Err(Error::Message("invalid section headers length".to_string()));
        }

        let ptr = shbuf.as_ptr() as *const Elf64SectionHeader;
        let pheader: &'a [Elf64SectionHeader] =
            unsafe { std::slice::from_raw_parts(ptr, header.e_shnum as usize) };
        Ok(pheader)
    }

    pub fn get_section_buffer<'a>(&self, buf: &'a [u8]) -> Result<&'a [u8], Error> {
        let offset = self.sh_offset as usize;
        let size = self.sh_size as usize;

        let buf = &buf[offset..(offset + size)];
        if buf.len() < size {
            return Err(Error::Message(
                "invalid section offset and size".to_string(),
            ));
        }

        Ok(buf)
    }
}

#[derive(Debug, Clone)]
pub struct StringTable<'a> {
    buf: &'a [u8],
}

impl<'a> StringTable<'a> {
    pub fn parse(buf: &'a [u8], hdr: &Elf64SectionHeader) -> Result<StringTable<'a>, Error> {
        if hdr.sh_type != 0x03 {
            return Err(Error::Message("section not a string table".to_string()));
        }

        let buf = hdr.get_section_buffer(buf)?;

        if !buf.is_empty() && buf[0] != 0x00 {
            return Err(Error::Message("invalid string table".to_string()));
        }

        Ok(StringTable { buf })
    }

    pub fn get_string(&self, offset: usize) -> &'a CStr {
        if offset >= self.buf.len() {
            panic!("invalid string access");
        }
        let ptr = self.buf.as_ptr() as *const u8;
        unsafe { CStr::from_ptr(ptr.add(offset)) }
    }

    pub fn get_all_strings(&self) -> Vec<&'a CStr> {
        let mut strings = vec![];
        let mut start = 0;
        for i in 0..(self.buf.len()) {
            if self.buf[i] == 0 {
                strings.push(self.get_string(start));
                start = i + 1;
            }
        }
        strings
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("open error: {0}")]
    Open(std::io::Error),
    #[error("read error: {0}")]
    Read(std::io::Error),
    #[error("invalid magic number")]
    InvalidMagicNumber,
    #[error("unexpected class")]
    UnsupportedClass,
    #[error("invalid class")]
    InvalidClass,
    #[error("invalid endianness")]
    InvalidEndianness,
    #[error("error: {0}")]
    Message(String),
}
