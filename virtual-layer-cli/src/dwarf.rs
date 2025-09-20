use std::borrow;

use eyre::Context as _;
use object::{Object as _, ObjectSection as _};

pub fn parse_dwarf(path: &impl AsRef<std::path::Path>) -> eyre::Result<()> {
    let file = std::fs::read(&path).wrap_err("Failed to read built VFS")?;
    let object =
        object::read::File::parse(&*file).wrap_err("Failed to parse built VFS by gimli")?;

    let endian = if object.is_little_endian() {
        gimli::RunTimeEndian::Little
    } else {
        gimli::RunTimeEndian::Big
    };

    fn load_section<'data>(
        object: &object::File<'data>,
        name: &str,
    ) -> object::Result<Section<'data>> {
        Ok(match object.section_by_name(name) {
            Some(section) => Section {
                data: section.uncompressed_data()?,
                relocations: section.relocation_map().map(RelocationMap)?,
            },
            None => Default::default(),
        })
    }

    // The reader type that will be stored in `Dwarf` and `DwarfPackage`.
    // If you don't need relocations, you can use `gimli::EndianSlice` directly.
    type Reader<'data> = gimli::RelocateReader<
        gimli::EndianSlice<'data, gimli::RunTimeEndian>,
        &'data RelocationMap,
    >;

    // Borrow a `Section` to create a `Reader`.
    fn borrow_section<'data>(
        section: &'data Section<'data>,
        endian: gimli::RunTimeEndian,
    ) -> Reader<'data> {
        let slice = gimli::EndianSlice::new(borrow::Cow::as_ref(&section.data), endian);
        gimli::RelocateReader::new(slice, &section.relocations)
    }

    let dwarf_sections = gimli::DwarfSections::load(|id| load_section(&object, id.name()))?;

    let empty_relocations = RelocationMap::default();
    let empty_section =
        gimli::RelocateReader::new(gimli::EndianSlice::new(&[], endian), &empty_relocations);

    // Create `Reader`s for all of the sections and do preliminary parsing.
    // Alternatively, we could have used `Dwarf::load` with an owned type such as `EndianRcSlice`.
    let dwarf = dwarf_sections.borrow(|section| borrow_section(section, endian));

    // Iterate over the compilation units.
    let mut iter = dwarf.units();
    println!("DWARF Units:");
    while let Some(header) = iter.next()? {
        println!(
            "Unit at <.debug_info+0x{:x}>",
            header.offset().as_debug_info_offset().unwrap().0
        );
        //     let unit = dwarf.unit(header)?;
        //     let unit_ref = unit.unit_ref(&dwarf);
        //     dump_unit(unit_ref)?;

        //     // Check for a DWO unit.
        //     let Some(dwp) = &dwp else { continue };
        //     let Some(dwo_id) = unit.dwo_id else { continue };
        //     println!("DWO Unit ID {:x}", dwo_id.0);
        //     let Some(dwo) = dwp.find_cu(dwo_id, &dwarf)? else {
        //         continue;
        //     };
        //     let Some(header) = dwo.units().next()? else {
        //         continue;
        //     };
        //     let unit = dwo.unit(header)?;
        //     let unit_ref = unit.unit_ref(&dwo);
        //     dump_unit(unit_ref)?;
    }

    // todo!();

    Ok(())
}

// The section data that will be stored in `DwarfSections` and `DwarfPackageSections`.
#[derive(Default)]
struct Section<'data> {
    data: borrow::Cow<'data, [u8]>,
    relocations: RelocationMap,
}

// This is a simple wrapper around `object::read::RelocationMap` that implements
// `gimli::read::Relocate` for use with `gimli::RelocateReader`.
// You only need this if you are parsing relocatable object files.
#[derive(Debug, Default)]
struct RelocationMap(object::read::RelocationMap);

impl<'a> gimli::read::Relocate for &'a RelocationMap {
    fn relocate_address(&self, offset: usize, value: u64) -> gimli::Result<u64> {
        Ok(self.0.relocate(offset as u64, value))
    }

    fn relocate_offset(&self, offset: usize, value: usize) -> gimli::Result<usize> {
        <usize as gimli::ReaderOffset>::from_u64(self.0.relocate(offset as u64, value as u64))
    }
}
