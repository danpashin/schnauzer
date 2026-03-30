use crate::Reader;
use crate::Result;

use scroll::IOread;

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

use super::LcStr;

/// `dylinker_command`
#[repr(C)]
#[derive(Debug, AutoEnumFields)]
pub struct LcDylinker {
    pub name: LcStr,
}

impl LcDylinker {
    pub(super) fn parse(
        reader: &Reader,
        command_offset: u64,
        base_offset: u64,
        endian: scroll::Endian,
    ) -> Result<Self> {
        let reader_clone = reader.clone();

        reader.with_lock(|reader| {
            reader.seek(SeekFrom::Start(base_offset))?;

            let name_offset: u32 = reader.ioread_with(endian)?;
            let name_offset = command_offset + u64::from(name_offset);

            let name = LcStr {
                reader: reader_clone,
                file_offset: name_offset,
            };

            Ok(LcDylinker { name })
        })
    }
}
