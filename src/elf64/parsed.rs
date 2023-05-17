use crate::elf::ELF_MAGIC;

#[derive(Debug, Clone)]
pub struct Elf64<'a> {
    header: &'a Elf64Header,
    program_headers: &'a [Elf64ProgramHeader],
    section_headers: &'a [Elf64SectionHeader],
}

/// We must assume a byte-for-byte representation because ELF files can be deployed
/// to both little-endian/big-endian, 32-bit/64-bit computers.
impl<'a> Elf64<'a> {
    pub fn parse(buf: &'a [u8]) -> Result<Elf64<'a>, Error> {
        let header = Elf64Header::from_buffer(buf)?;

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

        Ok(Self {
            header,
            program_headers,
            section_headers,
        })
    }
}

static_assertions::const_assert!(std::mem::size_of::<Elf64Header>() == 64);
static_assertions::const_assert!(std::mem::size_of::<Elf64Ident>() == 16);
static_assertions::const_assert!(std::mem::size_of::<Elf64ProgramHeader>() == 0x38);
static_assertions::const_assert!(std::mem::size_of::<Elf64SectionHeader>() == 0x40);

#[derive(Debug, Clone)]
#[repr(C, packed)]
pub struct Elf64Header {
    e_ident: Elf64Ident,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

impl Elf64Header {
    pub fn from_buffer<'a>(buf: &'a [u8]) -> Result<&'a Elf64Header, Error> {
        if buf.len() < std::mem::size_of::<Elf64Header>() {
            return Err(Error::Message("invalid header length".to_string()));
        }

        let ptr = buf.as_ptr() as *const Elf64Header;
        let header: &'a Elf64Header = unsafe { &*ptr };
        Ok(header)
    }
}

#[derive(Debug, Clone)]
#[repr(C, packed)]
pub struct Elf64Ident {
    magic: [u8; 4],
    class: u8,
    data: u8,
    version: u8,
    os_abi: u8,
    abi_version: u8,
    _padding: [u8; 7],
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
        header: &Elf64Header,
    ) -> Result<&'a [Elf64ProgramHeader], Error> {
        let offset = header.e_phoff as usize;
        let length = (header.e_phentsize as usize) * (header.e_phnum as usize);

        let phbuf = &buf[offset..];
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
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
}

impl Elf64SectionHeader {
    pub fn parse_headers<'a>(
        buf: &'a [u8],
        header: &Elf64Header,
    ) -> Result<&'a [Elf64SectionHeader], Error> {
        let offset = header.e_shoff as usize;
        let length = (header.e_shentsize as usize) * (header.e_shnum as usize);

        let shbuf = &buf[offset..];
        if shbuf.len() < length {
            return Err(Error::Message("invalid section headers length".to_string()));
        }

        let ptr = shbuf.as_ptr() as *const Elf64SectionHeader;
        let pheader: &'a [Elf64SectionHeader] =
            unsafe { std::slice::from_raw_parts(ptr, header.e_shnum as usize) };
        Ok(pheader)
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
