use std::{fs::File, io::Read};

use elf::elf64::raw::Elf64Headers;

fn main() {
    let mut file = File::open("/bin/touch").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();

    let elf = Elf64Headers::parse(&buf[..]).unwrap();
    // println!("{:?}", elf);

    // TODO: unaligned accesses

    let text_headers = elf.get_header(".text").unwrap();
    let data = text_headers.get_section_buffer(&buf[..]).unwrap();
    println!("{:?}", data);

    // println!("{:?}", elf.sh_names.get_all_strings());
}
