use super::LoadCommand;
use super::MachHeader;
use super::Reader;
use super::Result;

use crate::X64Context;
use scroll::ctx::SizeWith;
use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

#[derive(Clone)]
pub struct MachObject {
    reader: Reader,

    pub(super) header: MachHeader,
    pub(super) commands_offset: u64,

    /// File offset of single arch
    base_offset: u64,
}

impl MachObject {
    pub(super) fn parse(mut reader: Reader, base_offset: u64) -> Result<MachObject> {
        reader.seek(SeekFrom::Start(base_offset))?;

        let header = MachHeader::parse(reader.clone())?;

        let ctx = match (header.magic.is_reverse(), header.magic.is_64()) {
            (false, true) => X64Context::On(scroll::BE),
            (false, false) => X64Context::Off(scroll::BE),
            (true, true) => X64Context::On(scroll::LE),
            (true, false) => X64Context::Off(scroll::LE),
        };

        let header_size = MachHeader::size_with(&ctx);

        // After reading the header `reader` should stand on
        // start of load commands list
        let commands_offset = base_offset + header_size as u64;

        Ok(MachObject {
            reader,
            header,
            commands_offset,
            base_offset,
        })
    }
}

impl MachObject {
    #[must_use]
    pub fn header(&self) -> &MachHeader {
        &self.header
    }

    /// File offset of single arch
    #[must_use]
    pub fn file_offset(&self) -> u64 {
        self.base_offset
    }

    #[must_use]
    pub fn load_commands_iterator(&self) -> LoadCommandIterator {
        LoadCommandIterator::new(
            self.reader.clone(),
            self.commands_offset,
            self.header.sizeofcmds,
            self.header.magic.endian(),
            self.header.magic.is_64(),
            self.base_offset,
        )
    }

    #[must_use]
    pub fn segments_iterator(&self) -> SegmentIterator {
        SegmentIterator
    }
}

impl Debug for MachObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let commands: Vec<LoadCommand> = self.load_commands_iterator().collect();

        f.debug_struct("MachObject")
            .field("header", &self.header)
            .field("commands_offset", &self.commands_offset)
            .field("self.load_commands_iterator()", &commands)
            .finish_non_exhaustive()
    }
}
pub struct LoadCommandIterator {
    reader: Reader,
    current_offset: u64,
    end_offset: u64,
    endian: scroll::Endian,
    is_64: bool,
    object_file_offset: u64,
}

impl LoadCommandIterator {
    fn new(
        reader: Reader,
        base_offset: u64,
        size_of_cmds: u32,
        endian: scroll::Endian,
        is_64: bool,
        object_file_offset: u64,
    ) -> LoadCommandIterator {
        LoadCommandIterator {
            reader,
            current_offset: base_offset,
            end_offset: base_offset + u64::from(size_of_cmds),
            endian,
            is_64,
            object_file_offset,
        }
    }
}

impl Iterator for LoadCommandIterator {
    type Item = LoadCommand;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_offset >= self.end_offset {
            return None;
        }

        let lc = LoadCommand::parse(
            self.reader.clone(),
            self.current_offset,
            self.endian,
            self.is_64,
            self.object_file_offset,
        );

        if let Ok(lc) = lc {
            self.current_offset += u64::from(lc.cmdsize);

            Some(lc)
        } else {
            None
        }
    }
}

pub struct SegmentIterator;
