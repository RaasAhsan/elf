use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use elf::{
    elf::{ObjectType, SymbolType},
    elf64::{header::Elf64Headers, string_table::StringTable, symbol_table::SymbolTable},
};
use num_traits::FromPrimitive;

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

    #[arg(long)]
    symbols: bool,

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
        let elf_type = elf.header.e_type;
        let entry = elf.header.e_entry;
        println!(
            "ELF file header: \n\
            \tClass: {} \n\
            \tMachine: 0x{:02x}\n\
            \tData: {}\n\
            \tType: {}\n\
            \tEntrypoint: 0x{:08x}",
            elf.header.e_ident.class, machine, elf.header.e_ident.data, elf_type, entry
        );

        println!();
    }

    if cli.section_headers {
        println!("ELF section headers:");
        println!("\t{:<24} Type", "Name");

        for s in elf.section_headers.iter() {
            // TODO: ideally use a path dependent type here
            let name = elf
                .sh_names
                .get_string(s.sh_name as usize)
                .to_str()
                .unwrap();
            let sh_type = s.sh_type;
            println!("\t{name:<24} 0x{sh_type:02x}");
        }

        println!();
    }

    if cli.program_headers {
        println!("ELF program headers:");

        for h in elf.program_headers.iter() {
            let p_type = h.p_type;
            let elf_p_type = ObjectType::from_u32(p_type);
            let p_vaddr = h.p_vaddr;
            let p_memsz = h.p_memsz;
            let p_offset = h.p_offset;
            let p_filesz = h.p_filesz;
            let p_flags = h.p_flags;
            println!(
                "\tType: {elf_p_type:?} (0x{:08x}), Offset: 0x{:08x}, Start: 0x{:08x}, Mem Size: 0x{:08x}, File Size: 0x{:08x}, Flags: {:b}",
                p_type, p_offset, p_vaddr, p_memsz, p_filesz, p_flags
            );
        }

        println!();
    }

    if cli.symbols {
        let sh = elf.find_section_header(0x02).unwrap();

        println!("Symbol table:");

        let symtab = SymbolTable::parse(&buf, sh).unwrap();

        // the sh_link attribute for a symtab section designates the string table for symbol names
        let symstr_hdr = elf
            .get_section_header_by_index(sh.sh_link as usize)
            .unwrap();
        let strtab = StringTable::parse(&buf, symstr_hdr).unwrap();

        for (index, sym) in symtab.iter().enumerate() {
            let st_name = sym.st_name;
            let st_value = sym.st_value;
            let st_size = sym.st_size;
            let st_info = sym.st_info;

            let st_type = SymbolType::from_u8(st_info & 0xf).unwrap();

            let name = if st_name == 0 {
                ""
            } else {
                strtab.get_string(st_name as usize).to_str().unwrap()
            };

            println!(
                "\t{index:>3}: {:<16} 0x{:08x} {:<5} {:?}",
                name, st_value, st_size, st_type
            );
        }
    }

    let text_headers = elf.get_header(".text").unwrap();
    let data = text_headers.get_section_buffer(&buf[..]).unwrap();
    // println!("{:?}", data);
    // println!("{:?}", elf.sh_names.get_all_strings());
}
