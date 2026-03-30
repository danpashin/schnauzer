use crate::Reader;
use crate::Result;

use scroll::IOread;

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

use super::LcStr;

/// `sub_library_command`
#[repr(C)]
#[derive(Debug, AutoEnumFields)]
pub struct LcSublibrary {
    pub sub_library: LcStr,
}

impl LcSublibrary {
    pub(super) fn parse(
        mut reader: Reader,
        command_offset: u64,
        base_offset: u64,
        endian: scroll::Endian,
    ) -> Result<Self> {
        reader.seek(SeekFrom::Start(base_offset))?;

        let name_offset: u32 = reader.ioread_with(endian)?;
        let name_offset = command_offset + u64::from(name_offset);

        let sub_library = LcStr {
            reader: reader.clone(),
            file_offset: name_offset,
        };

        Ok(LcSublibrary { sub_library })
    }
}
