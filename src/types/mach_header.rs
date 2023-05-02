use super::RcReader;
use scroll::IOread;
use super::constants::*;
use super::Result;
use super::Magic;

use std::fmt::{Debug};

use super::utils;

pub struct MachHeader {
    pub magic: Magic,
    pub cputype: CPUType,
    pub cpusubtype: CPUSubtype,
    pub filetype: u32,
    pub ncmds: u32,
    pub sizeofcmds: u32,
    pub flags: u32,
    pub reserved: u32, // For 64 bit headers
}

impl MachHeader {
    /// We assume reader is already stands on correct position
    pub(super) fn parse(reader: RcReader) -> Result<MachHeader> {
        let mut reader_mut = reader.borrow_mut();

        let mut ctx = scroll::BE;

        let magic: u32 = reader_mut.ioread_with(ctx)?;
        let magic: Magic = magic.try_into()?;

        if magic.is_reverse() {
            ctx = scroll::LE;
        }
        let ctx = ctx;

        let cpu_type: CPUType = reader_mut.ioread_with(ctx)?;
        let cpu_subtype: CPUSubtype = reader_mut.ioread_with(ctx)?;
        let file_type: u32 = reader_mut.ioread_with(ctx)?;
        let ncmds: u32 = reader_mut.ioread_with(ctx)?;
        let size_of_cmds: u32 = reader_mut.ioread_with(ctx)?;
        let flags: u32 = reader_mut.ioread_with(ctx)?;

        let mut reserved = 0u32;
        if magic.is_64() {
            reserved = reader_mut.ioread_with(ctx)?;
        }

        Ok(MachHeader {
            magic,
            cputype: cpu_type,
            cpusubtype: cpu_subtype,
            filetype: file_type,
            ncmds,
            sizeofcmds: size_of_cmds,
            flags,
            reserved,
        })
    }
}

impl MachHeader {
    pub fn masked_cpu_subtype(&self) -> CPUSubtype {
        utils::masked_cpu_subtype(self.cpusubtype)
         & !CPU_SUBTYPE_MASK
    }

    pub fn feature_flags(&self) -> u32 {
        utils::feature_flags(self.cpusubtype)
    }

    pub fn is_64(&self) -> bool {
        utils::is_64(self.cputype)
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

impl MachHeader {
    pub fn magic(&self) -> Magic {
        self.magic
    }

    pub fn cpu_type(&self) -> CPUType {
        self.cputype
    }

    pub fn cpu_subtype(&self) -> CPUSubtype {
        self.cpusubtype & !CPU_SUBTYPE_MASK
    }

    pub fn file_type(&self) -> u32 {
        self.filetype
    }

    pub fn ncmds(&self) -> u32 {
        self.ncmds
    }

    pub fn size_of_cmds(&self) -> u32 {
        self.sizeofcmds
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }

    pub fn reserved(&self) -> u32 {
        self.reserved
    }
}
