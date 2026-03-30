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
        mut reader: Reader,
        is_64: bool,
        base_offset: usize,
        endian: scroll::Endian,
        object_file_offset: u64,
    ) -> Result<Self> {
        reader.seek(SeekFrom::Start(base_offset as u64))?;

        let symoff: u32 = reader.ioread_with(endian)?;
        let nsyms: u32 = reader.ioread_with(endian)?;
        let stroff: u32 = reader.ioread_with(endian)?;
        let strsize: u32 = reader.ioread_with(endian)?;

        Ok(LcSymtab {
            reader,
            is_64,
            symoff,
            nsyms,
            stroff,
            strsize,
            endian,
            object_file_offset,
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
            .finish()
    }
}

impl LcSymtab {
    pub fn nlist_iterator(&self) -> NlistIterator {
        NlistIterator::new(
            self.reader.clone(),
            self.is_64,
            self.object_file_offset + self.symoff as u64,
            self.object_file_offset + self.stroff as u64,
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

        let offset = match self.is_64 {
            true => self.symoff + BYTES_PER_NLIST64 as u64 * self.current as u64,
            false => self.symoff + BYTES_PER_NLIST32 as u64 * self.current as u64,
        };
        if let Err(_) = self.reader.seek(SeekFrom::Start(offset)) {
            return None;
        }

        self.current += 1;

        if let Ok(nlist) = Nlist::parse(self.reader.clone(), self.stroff, self.is_64, self.endian) {
            return Some(nlist);
        } else {
            return None;
        }
    }
}
