use std::ffi::{c_char, CStr};

use super::{header::Elf64SectionHeader, Error};

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
        let ptr = self.buf.as_ptr() as *const c_char;
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
