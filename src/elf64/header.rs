use crate::elf64::{ELF_CLASS_64, ELF_DATA_LITTLE, ELF_MAGIC};

use super::{string_table::StringTable, Error};

/// A raw representation of the headers in an ELF file.
/// This includes the ELF headers, the program headers, and
/// the section headers. This contains pointers to various
/// sections in the ELF file.
pub struct Headers<'a> {
    pub header: &'a FileHeaders,
    pub program_headers: &'a [ProgramHeader],
    pub section_headers: &'a [SectionHeader],
    pub sh_names: StringTable<'a>,
}

/// We must assume a byte-for-byte representation because ELF files can be deployed
/// to both little-endian/big-endian, 32-bit/64-bit computers.
impl<'a> Headers<'a> {
    pub fn parse<A: AsRef<[u8]>>(buf: &'a A) -> Result<Headers<'a>, Error> {
        let header = FileHeaders::parse(buf)?;

        if header.e_ident.magic != ELF_MAGIC {
            return Err(Error::InvalidMagicNumber);
        }

        if header.e_ident.class != ELF_CLASS_64 {
            return Err(Error::InvalidClass);
        }

        if header.e_ident.data != ELF_DATA_LITTLE {
            return Err(Error::InvalidEndianness);
        }

        let program_headers = ProgramHeader::parse_headers(buf, header)?;
        let section_headers: &[SectionHeader] = SectionHeader::parse_headers(buf, header)?;

        // TODO: validate
        let sh_names_header = &section_headers[header.e_shstrndx as usize];
        let sh_names = StringTable::parse(buf, sh_names_header)?;

        Ok(Self {
            header,
            program_headers,
            section_headers,
            sh_names,
        })
    }

    pub fn get_section_header_by_index(&self, index: usize) -> Option<&SectionHeader> {
        if index >= self.section_headers.len() {
            return None;
        }

        Some(&self.section_headers[index])
    }

    pub fn find_section_header(&self, sh_type: u32) -> Option<&SectionHeader> {
        self.section_headers
            .iter()
            .find(|&hdr| hdr.sh_type == sh_type)
    }
}

static_assertions::const_assert!(std::mem::size_of::<FileHeaders>() == 64);
static_assertions::const_assert!(std::mem::size_of::<Ident>() == 16);
static_assertions::const_assert!(std::mem::size_of::<ProgramHeader>() == 0x38);
static_assertions::const_assert!(std::mem::size_of::<SectionHeader>() == 0x40);

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct FileHeaders {
    pub e_ident: Ident,
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

impl FileHeaders {
    pub fn parse<'a, A: AsRef<[u8]>>(buf: &'a A) -> Result<&'a FileHeaders, Error> {
        let buf = buf.as_ref();
        if buf.len() < std::mem::size_of::<FileHeaders>() {
            return Err(Error::Message("invalid header length".to_string()));
        }

        let ptr = buf.as_ptr() as *const FileHeaders;
        let header: &'a FileHeaders = unsafe { &*ptr };
        Ok(header)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Ident {
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
pub struct ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

impl ProgramHeader {
    pub fn parse_headers<'a, A: AsRef<[u8]>>(
        buf: &'a A,
        header: &FileHeaders,
    ) -> Result<&'a [ProgramHeader], Error> {
        let offset = header.e_phoff as usize;
        let length = (header.e_phentsize as usize) * (header.e_phnum as usize);

        let phbuf = &buf.as_ref()[offset..(offset + length)];
        if phbuf.len() < length {
            return Err(Error::Message("invalid program headers length".to_string()));
        }

        let ptr = phbuf.as_ptr() as *const ProgramHeader;
        let pheader: &'a [ProgramHeader] =
            unsafe { std::slice::from_raw_parts(ptr, header.e_phnum as usize) };
        Ok(pheader)
    }
}

#[derive(Debug, Clone)]
#[repr(C, packed)]
pub struct SectionHeader {
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

impl SectionHeader {
    pub fn parse_headers<'a, A: AsRef<[u8]>>(
        buf: &'a A,
        header: &FileHeaders,
    ) -> Result<&'a [SectionHeader], Error> {
        let offset = header.e_shoff as usize;
        let length = (header.e_shentsize as usize) * (header.e_shnum as usize);

        let shbuf = &buf.as_ref()[offset..(offset + length)];
        if shbuf.len() < length {
            return Err(Error::Message("invalid section headers length".to_string()));
        }

        let ptr = shbuf.as_ptr() as *const SectionHeader;
        let pheader: &'a [SectionHeader] =
            unsafe { std::slice::from_raw_parts(ptr, header.e_shnum as usize) };
        Ok(pheader)
    }

    pub fn get_section_buffer<'a, A: AsRef<[u8]>>(&self, buf: &'a A) -> Result<&'a [u8], Error> {
        let offset = self.sh_offset as usize;
        let size = self.sh_size as usize;

        let buf = &buf.as_ref()[offset..(offset + size)];
        if buf.len() < size {
            return Err(Error::Message(
                "invalid section offset and size".to_string(),
            ));
        }

        Ok(buf)
    }
}
