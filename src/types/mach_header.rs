use super::auto_enum_fields::*;
use super::primitives::*;
use super::Magic;
use super::Reader;
use super::Result;
use schnauzer_derive::AutoEnumFields;
use scroll::IOread;

use scroll::ctx::SizeWith;
use std::fmt::Debug;

#[derive(Clone, AutoEnumFields)]
pub struct MachHeader {
    pub magic: Magic,
    pub cputype: CPUType,
    pub cpusubtype: CPUSubtype,
    pub filetype: FileType,
    pub ncmds: u32,
    pub sizeofcmds: u32,
    pub flags: ObjectFlags,
    pub reserved: Hu32, // For 64 bit headers
}

impl MachHeader {
    /// We assume reader is already stands on correct position
    pub(super) fn parse(reader: &Reader) -> Result<MachHeader> {
        reader.with_lock(|reader| {
            let mut ctx = scroll::BE;
            let magic: u32 = reader.ioread_with(ctx)?;
            let magic: Magic = magic.try_into()?;

            if magic.is_reverse() {
                ctx = scroll::LE;
            }
            let ctx = ctx;

            let cpu_type: CPUType = reader.ioread_with(ctx)?;
            let cpu_subtype: CPUSubtype = reader.ioread_with(ctx)?;
            let file_type: FileType = reader.ioread_with(ctx)?;
            let ncmds: u32 = reader.ioread_with(ctx)?;
            let size_of_cmds: u32 = reader.ioread_with(ctx)?;
            let flags: ObjectFlags = reader.ioread_with(ctx)?;

            let mut reserved = 0u32;
            if magic.is_64() {
                reserved = reader.ioread_with(ctx)?;
            }

            Ok(MachHeader {
                magic,
                cputype: cpu_type,
                cpusubtype: cpu_subtype,
                filetype: file_type,
                ncmds,
                sizeofcmds: size_of_cmds,
                flags,
                reserved: Hu32(reserved),
            })
        })
    }
}

impl MachHeader {
    /// Returns [`PrintableCPU`] if both `cputype` and `cpusubtype` supported by printable structure.
    #[must_use]
    pub fn printable_cpu(&self) -> Option<PrintableCPU> {
        PrintableCPU::new(self.cputype, self.cpusubtype)
    }
}

impl Debug for MachHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MachHeader")
            .field("magic", &self.magic)
            .field("cpu_type", &self.cputype)
            .field("cpu_subtype", &self.cpusubtype)
            .field("file_type", &self.filetype)
            .field("ncmds", &self.ncmds)
            .field("size_of_cmds", &self.sizeofcmds)
            .field("flags", &self.flags)
            .field("reserved", &self.reserved)
            .finish()
    }
}

impl SizeWith<X64Context> for MachHeader {
    fn size_with(ctx: &X64Context) -> usize {
        let endian = ctx.endian();

        let base_size = size_of::<u32>() // magic
            + CPUType::size_with(endian) // cpu_type
            + CPUSubtype::size_with(endian) // cpu_subtype
            + FileType::size_with(endian) // file_type
            + size_of::<u32>() // ncmds
            + size_of::<u32>() // size_of_cmds
            + ObjectFlags::size_with(endian) // size_of_cmds
        ;

        if ctx.is_64() {
            base_size + size_of::<u32>() // reserved;
        } else {
            base_size
        }
    }
}
