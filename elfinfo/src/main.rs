use std::{fs::File, path::PathBuf};

use clap::Parser;
use elf::{
    parsed::{
        dynamic::DynamicTag,
        header::Header,
        relocation::RelocationType,
        segment::{SegmentFlag, SegmentType},
        symbol::SymbolType,
    },
    raw::{
        dynamic::DynamicTable,
        header::Headers,
        relocation::{Rela, RelocationTable},
        string::StringTable,
        symbol::SymbolTable,
        SHT_DYNAMIC, SHT_DYNSYM, SHT_RELA, SHT_SYMTAB,
    },
};
use enumflags2::BitFlags;
use memmap2::Mmap;
use num_traits::FromPrimitive;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    all: bool,

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

    /// Display section-to-segment mapping
    #[arg(long)]
    section_mapping: bool,

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
    let header: Header = Header::from_raw(elf.header).unwrap();

    if cli.file_header || cli.all {
        println!(
            "ELF file header: \n\
            \tClass: {:?} \n\
            \tMachine: 0x{:02x}\n\
            \tData: {:?}\n\
            \tType: {:?}\n\
            \tEntrypoint: 0x{:08x}",
            header.class, header.machine, header.data, header.r#type, header.entrypoint
        );

        println!();
    }

    if cli.section_headers || cli.all {
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

    if cli.program_headers || cli.all {
        println!("ELF program headers:");
        println!(
            "\t{:<24} {:<16} {:<16} {:<16} {:<16} {:<16}",
            "Type", "Offset", "Start", "Mem Size", "File Size", "Flags"
        );

        for h in elf.program_headers.iter() {
            let elf_type = SegmentType::from_u32(h.get_type());
            let flags = BitFlags::<SegmentFlag>::from_bits((h.get_flags() & 0xff) as u8).unwrap();
            let flags_vec = flags.iter().collect::<Vec<_>>();
            println!(
                "\t{:<24} 0x{:014x} 0x{:014x} 0x{:014x} 0x{:014x} {:?}",
                format!("{:?} (0x{:02x})", elf_type, h.get_type()),
                h.get_offset(),
                h.get_vaddr(),
                h.get_memsz(),
                h.get_filesz(),
                flags_vec
            );
        }

        println!();
    }

    if cli.symbols || cli.all {
        if let Some(sh) = elf.find_section_header(SHT_SYMTAB) {
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
                let st_type = SymbolType::from_u8(sym.get_type()).unwrap();

                let name = if sym.get_name() == 0 {
                    ""
                } else {
                    strtab.get_string(sym.get_name() as usize).to_str().unwrap()
                };

                println!(
                    "\t{index:>3}: {:<32} 0x{:08x} {:>6} {:?}",
                    name,
                    sym.get_value(),
                    sym.get_size(),
                    st_type
                );
            }
        } else {
            println!("There is no symbol table in this ELF object.");
        }

        println!();
    }

    if cli.dyn_syms || cli.all {
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

    if cli.relocations || cli.all {
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

                let reloc_table =
                    RelocationTable::<Rela>::parse_section_header(&mmap, hdr).unwrap();
                let sym_table = SymbolTable::parse(&mmap, &elf, sym_hdr).unwrap();

                println!("Relocation section ({name} @ 0x{:06x}):", sh_offset);
                println!(
                    "\t{:<16} {:<16} {:<16} {:<24} {:<32}",
                    "Offset", "Info", "Addend", "Type", "Symbol Name"
                );

                for reloc in reloc_table.iter() {
                    let symbol = sym_table.get_elf_symbol(reloc.get_symbol() as usize); // TODO: factor this out
                    let reloc_type = RelocationType::from_u32(reloc.get_type());
                    println!(
                        "\t{:016x} {:016x} {:016x} {:<24?} {sym_name:<32}",
                        reloc.get_offset(),
                        reloc.get_info(),
                        reloc.get_addend(),
                        reloc_type,
                        sym_name = symbol.name
                    );
                }

                println!()
            }
        }
    }

    if cli.dynamic || cli.all {
        let sh = elf.find_section_header(SHT_DYNAMIC).unwrap();
        let name = elf
            .sh_names
            .get_string(sh.sh_name as usize)
            .to_str()
            .unwrap();

        println!("Dynamic linking information ({name}):");
        println!("\t{:<16} {:<16}", "Tag", "Value");

        let dyntab = DynamicTable::parse_section(&mmap, sh).unwrap();

        for dynamic in dyntab.iter() {
            let tag = DynamicTag::from_u64(dynamic.get_tag());
            println!(
                "\t{:<16} {:016x}",
                tag.map(|t| format!("{t:?}"))
                    .unwrap_or(format!("{:016x}", dynamic.get_tag())),
                dynamic.get_value()
            );
        }

        println!();
    }

    if cli.section_mapping || cli.all {
        println!("Section-to-segment mapping:");
        println!("\tSegment Sections");

        for (i, ph) in elf.program_headers.iter().enumerate() {
            let ph_addr = ph.get_vaddr();
            let ph_addr_end = ph_addr + ph.get_memsz();

            let mut segments = String::new();

            for sh in elf.section_headers.iter() {
                let sh_addr = sh.sh_addr;

                if sh_addr >= ph.get_vaddr() && sh_addr < ph_addr_end {
                    let name = elf
                        .sh_names
                        .get_string(sh.sh_name as usize)
                        .to_str()
                        .unwrap();
                    segments.push_str(&format!("{name} "));
                }
            }
            println!("\t {i:>6}: {segments}");
        }

        println!();
    }
}
