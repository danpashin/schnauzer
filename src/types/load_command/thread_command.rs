use crate::Reader;
use crate::Result;

use scroll::IOread;
use std::fmt::{Debug, Formatter};
use std::io::{Seek, SeekFrom};
use std::mem::size_of;

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

// LC_THREAD_FLAVOR_HEADER_SIZE = sizeof(thread_command.flavor) + sizeof(thread_command.count)
const LC_THREAD_FLAVOR_HEADER_SIZE: usize = size_of::<u32>() + size_of::<u32>();

/// `thread_command`
#[repr(C)]
#[derive(AutoEnumFields)]
pub struct LcThread {
    reader: Reader,

    cmdsize: usize,
    base_offset: u64,
    endian: scroll::Endian,
}

impl LcThread {
    pub(super) fn parse(
        reader: &Reader,
        cmdsize: usize,
        base_offset: u64,
        endian: scroll::Endian,
    ) -> Self {
        LcThread {
            reader: reader.clone(),
            cmdsize,
            base_offset,
            endian,
        }
    }

    #[must_use]
    pub fn flavor_iterator(&self) -> FlavorIterator {
        FlavorIterator::new(
            self.reader.clone(),
            self.cmdsize,
            self.base_offset,
            self.endian,
        )
    }
}

#[repr(C)]
#[derive(AutoEnumFields)]
pub struct LcThreadFlavor {
    pub flavor: u32,
    pub count: usize,
    /* struct XXX_thread_state state   thread state for this flavor */
    /* ... */
    state_offset: u64,
}

impl LcThreadFlavor {
    pub(super) fn parse(
        reader: &Reader,
        base_offset: u64,
        endian: scroll::Endian,
    ) -> Result<Option<Self>> {
        reader.with_lock(|reader| {
            reader.seek(SeekFrom::Start(base_offset))?;

            let flavor: u32 = reader.ioread_with(endian)?;
            let count: u32 = reader.ioread_with(endian)?;

            let state_offset = reader.stream_position()?;

            if flavor == 0 && count == 0 {
                // We reached the end of the list
                return Ok(None);
            }

            Ok(Some(LcThreadFlavor {
                flavor,
                count: count as usize,
                state_offset,
            }))
        })
    }

    #[must_use]
    pub fn get_state_offset(&self) -> u64 {
        self.state_offset
    }

    fn calculate_flavor_size(&self) -> usize {
        // the size of a flavor is based on the following:
        // flavor_size = LC_THREAD_FLAVOR_HEADER_SIZE + sizeof(thread_command.state)

        // count * sizeof(uint32_t) is equalivent to sizeof(thread_command.state)
        LC_THREAD_FLAVOR_HEADER_SIZE + self.count * size_of::<u32>()
    }
}

impl Debug for LcThreadFlavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LcThreadFlavor")
            .field("flavor", &self.flavor)
            .field("count", &self.count)
            .finish_non_exhaustive()
    }
}

pub struct FlavorIterator {
    reader: Reader,
    base_offset: u64,
    cmdsize: usize,
    endian: scroll::Endian,

    current: usize,
}

impl FlavorIterator {
    fn new(reader: Reader, cmdsize: usize, base_offset: u64, endian: scroll::Endian) -> Self {
        FlavorIterator {
            reader,
            base_offset,
            cmdsize,
            endian,
            current: 0,
        }
    }
}

impl Iterator for FlavorIterator {
    type Item = LcThreadFlavor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.cmdsize {
            return None;
        }

        let offset = self.base_offset + self.current as u64;

        match LcThreadFlavor::parse(&self.reader, offset, self.endian) {
            Ok(Some(lc_thread_flavor)) => {
                self.current += lc_thread_flavor.calculate_flavor_size();
                Some(lc_thread_flavor)
            }

            Ok(None) => {
                self.current = self.cmdsize;
                None
            }

            Err(_) => None,
        }
    }
}

impl Debug for LcThread {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LcThread")
            .field("cmdsize", &self.cmdsize)
            .finish_non_exhaustive()
    }
}
