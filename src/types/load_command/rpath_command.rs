use crate::Reader;
use crate::Result;

use scroll::{IOread};

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

use super::LcStr;

/// `rpath_command`
#[repr(C)]
#[derive(Debug, AutoEnumFields)]
pub struct LcRpath {
    pub path: LcStr,
}

impl LcRpath {
    pub(super) fn parse(
        mut reader: Reader,
        command_offset: usize,
        base_offset: usize,
        endian: scroll::Endian,
    ) -> Result<Self> {
        reader.seek(SeekFrom::Start(base_offset as u64))?;

        let name_offset: u32 = reader.ioread_with(endian)?;
        let name_offset = name_offset + command_offset as u32;

        let path = LcStr {
            reader,
            file_offset: name_offset,
        };

        Ok(LcRpath { path })
    }
}
