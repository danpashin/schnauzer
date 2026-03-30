use super::primitives::*;
use super::MachObject;
use super::Reader;
use super::Result;
use scroll::IOread;

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

use super::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

#[derive(AutoEnumFields)]
pub struct FatArch {
    pub(crate) reader: Reader,

    pub cputype: CPUType,
    pub cpusubtype: CPUSubtype,
    pub offset: u32,
    pub size: u32,
    pub align: u32,
}

impl FatArch {
    pub(super) fn parse(mut reader: Reader, base_offset: u64) -> Result<FatArch> {
        const ENDIAN: scroll::Endian = scroll::BE;
        reader.seek(SeekFrom::Start(base_offset))?;

        let cpu_type: CPUType = reader.ioread_with(ENDIAN)?;
        let cpu_subtype: CPUSubtype = reader.ioread_with(ENDIAN)?;
        let offset: u32 = reader.ioread_with(ENDIAN)?;
        let size: u32 = reader.ioread_with(ENDIAN)?;
        let align: u32 = reader.ioread_with(ENDIAN)?;

        Ok(FatArch {
            reader: reader.clone(),
            cputype: cpu_type,
            cpusubtype: cpu_subtype,
            offset,
            size,
            align,
        })
    }
}

impl FatArch {
    pub fn object(&self) -> Result<MachObject> {
        MachObject::parse(self.reader.clone(), u64::from(self.offset))
    }

    #[must_use]
    pub fn printable_cpu(&self) -> Option<PrintableCPU> {
        PrintableCPU::new(self.cputype, self.cpusubtype)
    }
}

impl Debug for FatArch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("FatArch");

        s.field("cpu_type", &self.cputype)
            .field("cpu_subtype", &self.cpusubtype)
            .field("offset", &self.offset)
            .field("size", &self.size)
            .field("align", &self.align);

        if let Ok(h) = MachObject::parse(self.reader.clone(), u64::from(self.offset)) {
            s.field("mach_header()", &h);
        }

        s.finish()
    }
}
