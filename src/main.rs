use std::{fs::File, path::PathBuf};

use clap::Parser;
use elf::{
    elf::{ObjectClass, ObjectData, ObjectType, SegmentFlag, SegmentType, SymbolType},
    elf64::{
        dynamic::DynamicTable,
        header::Headers,
        relocation_table::{Rela, RelocationTable},
        string_table::StringTable,
        symbol_table::SymbolTable,
        SHT_DYNAMIC, SHT_DYNSYM, SHT_RELA, SHT_SYMTAB,
    },
};
use enumflags2::BitFlags;
use memmap2::Mmap;
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

    /// Display the symbol table
    #[arg(long)]
    symbols: bool,

    /// Display the dynamic linking symbol table
    #[arg(long)]
    dyn_syms: bool,

    /// Display the relocations
    #[arg(long, short)]
    relocations: bool,

    /// Display dynamic linking information
    #[arg(long, short)]
    dynamic: bool,

    /// Path to the ELF file
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let file = File::open(cli.file).unwrap();

    let mmap = unsafe { Mmap::map(&file).unwrap() };

    // let mut buf = vec![];
    // file.read_to_end(&mut buf).unwrap();

    let elf = Headers::parse(&mmap).unwrap();
    // println!("{:?}", elf);

    if cli.file_header {
        let machine = elf.header.e_machine;
        let class = ObjectClass::from_u8(elf.header.e_ident.class).unwrap();
        let elf_type = ObjectType::from_u16(elf.header.e_type).unwrap();
        let data = ObjectData::from_u8(elf.header.e_ident.data).unwrap();
        let entry = elf.header.e_entry;
        println!(
            "ELF file header: \n\
            \tClass: {:?} \n\
            \tMachine: 0x{:02x}\n\
            \tData: {:?}\n\
            \tType: {:?}\n\
            \tEntrypoint: 0x{:08x}",
            class, machine, data, elf_type, entry
        );

        println!();
    }

    if cli.section_headers {
        println!("ELF section headers:");
        println!(
            "\t{:<24} {:<16} {:<16} {:<16} {:<16}",
            "Name", "Type", "Offset", "Address", "Size"
        );

        for s in elf.section_headers.iter() {
            // TODO: ideally use a path dependent type here
            let name = elf
                .sh_names
                .get_string(s.sh_name as usize)
                .to_str()
                .unwrap();
            let sh_type = s.sh_type;
            let sh_offset = s.sh_offset;
            let sh_size = s.sh_size;
            let sh_addr = s.sh_addr;
            println!("\t{name:<24} {sh_type:016x} {sh_offset:016x} {sh_addr:016x} {sh_size:016x}");
        }

        println!();
    }

    if cli.program_headers {
        println!("ELF program headers:");
        println!(
            "\t{:<24} {:<16} {:<16} {:<16} {:<16} {:<16}",
            "Type", "Offset", "Start", "Mem Size", "File Size", "Flags"
        );

        for h in elf.program_headers.iter() {
            let p_type = h.p_type;
            let elf_p_type = SegmentType::from_u32(p_type);
            let p_vaddr = h.p_vaddr;
            let p_memsz = h.p_memsz;
            let p_offset = h.p_offset;
            let p_filesz = h.p_filesz;
            let p_flags = h.p_flags;
            let flags = BitFlags::<SegmentFlag>::from_bits((p_flags & 0xff) as u8).unwrap();
            let flags_vec = flags.iter().collect::<Vec<_>>();
            println!(
                "\t{:<24} 0x{:014x} 0x{:014x} 0x{:014x} 0x{:014x} {:?}",
                format!("{elf_p_type:?} (0x{p_type:02x})"),
                p_offset,
                p_vaddr,
                p_memsz,
                p_filesz,
                flags_vec
            );
        }

        println!();
    }

    if cli.symbols {
        let sh = elf.find_section_header(SHT_SYMTAB).unwrap();
        let name = elf
            .sh_names
            .get_string(sh.sh_name as usize)
            .to_str()
            .unwrap();

        println!("Symbol table ({name}):");
        println!(
            "\t{:<4} {:<32} {:<10} {:<6} {:<16}",
            "Num", "Name", "Value", "Size", "Type"
        );

        let symtab = SymbolTable::parse(&mmap, &elf, sh).unwrap();

        // the sh_link attribute for a symtab section designates the string table for symbol names
        let symstr_hdr = elf
            .get_section_header_by_index(sh.sh_link as usize)
            .unwrap();
        let strtab = StringTable::parse(&mmap, symstr_hdr).unwrap();

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
                "\t{index:>3}: {:<32} 0x{:08x} {:>6} {:?}",
                name, st_value, st_size, st_type
            );
        }

        println!();
    }

    if cli.dyn_syms {
        let sh = elf.find_section_header(SHT_DYNSYM).unwrap();
        let name = elf
            .sh_names
            .get_string(sh.sh_name as usize)
            .to_str()
            .unwrap();

        println!("Dynamic linking symbol table ({name}):");

        let symtab = SymbolTable::parse(&mmap, &elf, sh).unwrap();

        for (index, sym) in symtab.symbols_iter().enumerate() {
            let st_type = SymbolType::from_u8(sym.info & 0xf).unwrap();

            println!(
                "\t{index:>3}: {:<32} 0x{:08x} {:>6} {:?}",
                sym.name, sym.value, sym.size, st_type
            );
        }

        println!();
    }

    if cli.relocations {
        for hdr in elf.section_headers.iter() {
            let sh_type = hdr.sh_type;
            if sh_type == SHT_RELA {
                let name = elf
                    .sh_names
                    .get_string(hdr.sh_name as usize)
                    .to_str()
                    .unwrap();
                let sh_offset = hdr.sh_offset;

                // the sh_link attribute for a symtab section designates the string table for symbol names
                let sym_hdr = elf
                    .get_section_header_by_index(hdr.sh_link as usize)
                    .unwrap();

                let reloc_table = RelocationTable::<Rela>::parse(&mmap, hdr).unwrap();
                let sym_table = SymbolTable::parse(&mmap, &elf, sym_hdr).unwrap();

                println!("Relocation section ({name} @ 0x{:06x}):", sh_offset);
                println!(
                    "\t{:<16} {:<16} {:<16} {:<32}",
                    "Offset", "Info", "Addend", "Symbol Name"
                );

                for reloc in reloc_table.iter() {
                    let offset = reloc.r_offset;
                    let info = reloc.r_info;
                    let addend = reloc.r_addend;
                    let symbol = sym_table.get_elf_symbol((info >> 32) as usize); // TODO: factor this out
                    println!(
                        "\t{offset:016x} {info:016x} {addend:016x} {sym_name:<32}",
                        sym_name = symbol.name
                    );
                }

                println!()
            }
        }
    }

    if cli.dynamic {
        let sh = elf.find_section_header(SHT_DYNAMIC).unwrap();
        let name = elf
            .sh_names
            .get_string(sh.sh_name as usize)
            .to_str()
            .unwrap();

        println!("Dynamic linking information ({name}):");
        println!("\t{:<16} {:<16}", "Tag", "Value");

        let symtab = DynamicTable::parse(&mmap, sh).unwrap();

        for (_, dynamic) in symtab.iter().enumerate() {
            let tag = dynamic.d_tag;
            let value = dynamic.d_value;

            println!("\t{:016x} {:016x}", tag, value);
        }

        println!();
    }
}
