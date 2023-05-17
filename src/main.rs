use std::{fs::File, io::Read};

use elf::elf64::parsed::Elf64;

fn main() {
    let mut file = File::open("/bin/touch").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();

    let elf = Elf64::parse(&buf[..]).unwrap();
    println!("{:?}", elf);

    for section in elf.section_headers {
        if section.sh_type == 0x3 {
            let offset = section.sh_offset as usize;
            let size = section.sh_size as usize;
            let bytes = &buf[offset..(offset + size)];

            let mut strings = vec![];
            let mut start = 0;

            for (current, b) in bytes.iter().enumerate() {
                if *b == 0x0 {
                    strings.push(std::str::from_utf8(&bytes[start..current]).unwrap());
                    start = current + 1;
                }
            }

            println!("{:?}", strings);
        }
    }
}
