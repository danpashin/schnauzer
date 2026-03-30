use crate::constants::*;
use crate::Reader;
use crate::Result;

use scroll::IOread;

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

use crate::nlist::*;

/// `symtab_command`
#[repr(C)]
#[derive(AutoEnumFields)]
pub struct LcSymtab {
    reader: Reader,

    pub is_64: bool,

    pub symoff: u32,
    pub nsyms: u32,
    pub stroff: u32,
    pub strsize: u32,

    endian: scroll::Endian,
    object_file_offset: u64,
}

impl LcSymtab {
    pub(super) fn parse(
        reader: &Reader,
        is_64: bool,
        base_offset: u64,
        endian: scroll::Endian,
        object_file_offset: u64,
    ) -> Result<Self> {
        let reader_clone = reader.clone();

        reader.with_lock(|reader| {
            reader.seek(SeekFrom::Start(base_offset))?;

            let symoff: u32 = reader.ioread_with(endian)?;
            let nsyms: u32 = reader.ioread_with(endian)?;
            let stroff: u32 = reader.ioread_with(endian)?;
            let strsize: u32 = reader.ioread_with(endian)?;

            Ok(LcSymtab {
                reader: reader_clone,
                is_64,
                symoff,
                nsyms,
                stroff,
                strsize,
                endian,
                object_file_offset,
            })
        })
    }
}

impl Debug for LcSymtab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LcSymtab")
            .field("is_64", &self.is_64)
            .field("symoff", &self.symoff)
            .field("nsyms", &self.nsyms)
            .field("stroff", &self.stroff)
            .field("strsize", &self.strsize)
            .finish_non_exhaustive()
    }
}

impl LcSymtab {
    #[must_use]
    pub fn nlist_iterator(&self) -> NlistIterator {
        NlistIterator::new(
            self.reader.clone(),
            self.is_64,
            self.object_file_offset + u64::from(self.symoff),
            self.object_file_offset + u64::from(self.stroff),
            self.nsyms,
            self.endian,
        )
    }
}

pub struct NlistIterator {
    reader: Reader,
    pub is_64: bool,

    symoff: u64,
    stroff: u64,
    nsyms: u32,

    current: usize,
    endian: scroll::Endian,
}

impl NlistIterator {
    fn new(
        reader: Reader,
        is_64: bool,
        symoff: u64,
        stroff: u64,
        nsyms: u32,
        endian: scroll::Endian,
    ) -> Self {
        NlistIterator {
            reader,
            is_64,
            symoff,
            stroff,
            nsyms,
            current: 0,
            endian,
        }
    }
}

impl Iterator for NlistIterator {
    type Item = Nlist;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.nsyms as usize {
            return None;
        }

        let offset = if self.is_64 {
            self.symoff + BYTES_PER_NLIST64 as u64 * self.current as u64
        } else {
            self.symoff + BYTES_PER_NLIST32 as u64 * self.current as u64
        };
        if self.reader.seek(SeekFrom::Start(offset)).is_err() {
            return None;
        }

        self.current += 1;

        Nlist::parse(&self.reader, self.stroff, self.is_64, self.endian).ok()
    }
}
