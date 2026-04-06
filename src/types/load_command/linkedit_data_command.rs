use scroll::IOread;

use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::auto_enum_fields::*;
use crate::reader::Reader;
use schnauzer_derive::AutoEnumFields;

/// `linkedit_data_command`
#[repr(C)]
#[derive(AutoEnumFields)]
pub struct LcLinkEditData {
    reader: Reader,
    object_file_offset: u64,

    pub dataoff: u32,
    pub datasize: u32,
}

impl LcLinkEditData {
    pub(super) fn parse(
        reader: &Reader,
        base_offset: u64,
        object_file_offset: u64,
        endian: scroll::Endian,
    ) -> crate::result::Result<Self> {
        let reader_clone = reader.clone();

        reader.with_lock(|reader| {
            reader.seek(SeekFrom::Start(base_offset))?;

            let dataoff: u32 = reader.ioread_with(endian)?;
            let datasize: u32 = reader.ioread_with(endian)?;

            Ok(LcLinkEditData {
                reader: reader_clone,
                object_file_offset,
                dataoff,
                datasize,
            })
        })
    }

    pub fn read_data_to(&self, out: &mut dyn Write) -> crate::result::Result<()> {
        self.reader.with_lock(|reader| {
            reader.seek(SeekFrom::Start(
                self.object_file_offset + u64::from(self.dataoff),
            ))?;

            let mut reader = reader.take(u64::from(self.datasize));
            let count = std::io::copy(&mut reader, out)?;
            if count != u64::from(self.datasize) {
                let text = format!(
                    "Expected to write {} bytes, wrote only {}",
                    self.datasize, count
                );
                return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, text).into());
            }

            Ok(())
        })
    }
}

impl Debug for LcLinkEditData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LcLinkEditData")
            .field("dataoff", &self.dataoff)
            .field("datasize", &self.datasize)
            .finish_non_exhaustive()
    }
}
