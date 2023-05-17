use std::{fs::File, io::Read};

use elf::elf64::parsed::Elf64;

fn main() {
    let mut file = File::open("/bin/touch").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();

    let elf = Elf64::parse(&buf[..]).unwrap();
    println!("{:?}", elf);
}
