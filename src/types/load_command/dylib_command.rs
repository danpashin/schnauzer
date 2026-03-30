use crate::Reader;
use crate::Result;
use crate::Version32;

use scroll::IOread;

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

use super::LcStr;

/// `dylib_command`
#[repr(C)]
#[derive(Debug, AutoEnumFields)]
pub struct LcDylib {
    pub name: LcStr,
    pub timestamp: u32,
    pub current_version: Version32,
    pub compatibility_version: Version32,
}

impl LcDylib {
    pub(super) fn parse(
        mut reader: Reader,
        command_offset: u64,
        base_offset: u64,
        endian: scroll::Endian,
    ) -> Result<Self> {
        reader.seek(SeekFrom::Start(base_offset))?;

        let name_offset: u32 = reader.ioread_with(endian)?;
        let timestamp: u32 = reader.ioread_with(endian)?;
        let current_version: Version32 = reader.ioread_with(endian)?;
        let compatibility_version: Version32 = reader.ioread_with(endian)?;

        let name_offset = command_offset + u64::from(name_offset);

        let name = LcStr {
            reader,
            file_offset: name_offset,
        };

        Ok(LcDylib {
            name,
            timestamp,
            current_version,
            compatibility_version,
        })
    }
}
