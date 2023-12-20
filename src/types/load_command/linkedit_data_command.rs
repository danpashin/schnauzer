use scroll::IOread;

use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::auto_enum_fields::*;
use crate::reader::RcReader;
use schnauzer_derive::AutoEnumFields;

/// `linkedit_data_command`
#[repr(C)]
#[derive(AutoEnumFields)]
pub struct LcLinkEditData {
    reader: RcReader,
    object_file_offset: u64,

    pub dataoff: u32,
    pub datasize: u32,
}

impl LcLinkEditData {
    pub(super) fn parse(
        reader: RcReader,
        base_offset: usize,
        object_file_offset: u64,
        endian: scroll::Endian,
    ) -> crate::result::Result<Self> {
        let reader_clone = reader.clone();

        let mut reader_mut = reader.borrow_mut();
        reader_mut.seek(SeekFrom::Start(base_offset as u64))?;

        let dataoff: u32 = reader_mut.ioread_with(endian)?;
        let datasize: u32 = reader_mut.ioread_with(endian)?;
        std::mem::drop(reader_mut);

        Ok(LcLinkEditData {
            reader: reader_clone,
            object_file_offset,
            dataoff,
            datasize,
        })
    }

    pub fn read_data_to(&self, out: &mut dyn Write) -> crate::result::Result<()> {
        use std::cmp::min;
        const BUFFER_SIZE: usize = 4096;

        let mut reader = self.reader.borrow_mut();
        reader.seek(SeekFrom::Start(
            self.object_file_offset + self.dataoff as u64,
        ))?;

        let mut remainig = self.datasize as usize;

        let mut tmp = [0u8; BUFFER_SIZE];

        while remainig > 0 {
            let to_read = min(remainig, BUFFER_SIZE);

            match reader.read_exact(&mut tmp[..to_read]) {
                Ok(_) => match out.write_all(&mut tmp[..to_read]) {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(crate::result::Error::Other(Box::new(e)));
                    }
                },
                Err(e) => {
                    return Err(crate::result::Error::Other(Box::new(e)));
                }
            }

            remainig -= to_read;
        }

        Ok(())
    }
}

impl Debug for LcLinkEditData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LcLinkEditData")
            .field("dataoff", &self.dataoff)
            .field("datasize", &self.datasize)
            .finish()
    }
}
