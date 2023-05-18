use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use elf::elf64::raw::{Elf64Headers, StringTable};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Display the ELF file header
    #[arg(short, long)]
    file_header: bool,

    /// Display the ELF program headers
    #[arg(short, long)]
    program_headers: bool,

    /// Display the ELF program headers
    #[arg(short, long)]
    section_headers: bool,

    /// Path to the ELF file
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let mut file = File::open(cli.file).unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();

    let elf = Elf64Headers::parse(&buf[..]).unwrap();
    // println!("{:?}", elf);

    if cli.file_header {
        let machine = elf.header.e_machine;
        let output = format!(
            "ELF file header: \n\
            \tClass: {} \n\
            \tMachine: 0x{:02x}",
            elf.header.e_ident.class, machine
        );

        println!("{output}")
    }

    let text_headers = elf.get_header(".text").unwrap();
    let data = text_headers.get_section_buffer(&buf[..]).unwrap();
    // println!("{:?}", data);

    for s in elf.section_headers.iter() {
        if s.sh_type == 0x03 {
            let table = StringTable::parse(&buf[..], s).unwrap();
            println!("{:?}", table.get_all_strings());
        }
    }

    println!("{:?}", elf.sh_names.get_all_strings());
}
