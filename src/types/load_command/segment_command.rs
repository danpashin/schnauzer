use crate::primitives::*;
use crate::Reader;
use crate::Result;

use scroll::ctx::SizeWith;
use scroll::IOread;

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

use super::Section;

/// Both `segment_command` and `segment_command_64`
#[repr(C)]
#[derive(AutoEnumFields)]
pub struct LcSegment {
    reader: Reader,

    pub segname: Segname,
    pub vmaddr: Hu64,
    pub vmsize: Hu64,
    pub fileoff: u64_io,
    pub filesize: u64_io,
    pub maxprot: VmProt,
    pub initprot: VmProt,
    pub nsects: u32,
    pub flags: Hu32,

    object_file_offset: u64,
    sects_offset: u64,
    ctx: X64Context,
}

impl LcSegment {
    pub(super) fn parse(
        reader: &Reader,
        base_offset: u64,
        object_file_offset: u64,
        ctx: X64Context,
    ) -> Result<Self> {
        let reader_clone = reader.clone();

        reader.with_lock(|reader| {
            let endian = *ctx.endian();
            reader.seek(SeekFrom::Start(base_offset))?;

            let segname: Segname = reader.ioread_with(endian)?;

            let vmaddr: u64_io = reader.ioread_with(ctx)?;
            let vmaddr = Hu64(vmaddr.0);

            let vmsize: u64_io = reader.ioread_with(ctx)?;
            let vmsize = Hu64(vmsize.0);

            let fileoff: u64_io = reader.ioread_with(ctx)?;
            let filesize: u64_io = reader.ioread_with(ctx)?;
            let maxprot: VmProt = reader.ioread_with(endian)?;
            let initprot: VmProt = reader.ioread_with(endian)?;
            let nsects: u32 = reader.ioread_with(endian)?;
            let flags: Hu32 = reader.ioread_with(endian)?;

            let sects_offset = reader.stream_position()?;

            Ok(LcSegment {
                reader: reader_clone,
                segname,
                vmaddr,
                vmsize,
                fileoff,
                filesize,
                maxprot,
                initprot,
                nsects,
                flags,
                object_file_offset,
                sects_offset,
                ctx,
            })
        })
    }
}

impl Debug for LcSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LcSegment64")
            .field("segname", &self.segname)
            .field("vmaddr", &self.vmaddr)
            .field("vmsize", &self.vmsize)
            .field("fileoff", &self.fileoff)
            .field("filesize", &self.filesize)
            .field("maxprot", &self.maxprot)
            .field("initprot", &self.initprot)
            .field("nsects", &self.nsects)
            .field("flags", &self.flags)
            .finish_non_exhaustive()
    }
}

impl LcSegment {
    #[must_use]
    pub fn sections_iterator(&self) -> SectionIterator {
        SectionIterator::new(
            self.reader.clone(),
            self.nsects,
            self.sects_offset,
            self.object_file_offset,
            self.ctx,
        )
    }
}

pub struct SectionIterator {
    reader: Reader,

    nsects: u32,
    base_offset: u64,
    object_file_offset: u64,
    ctx: X64Context,

    current: u32,
}

impl SectionIterator {
    fn new(
        reader: Reader,
        nsects: u32,
        base_offset: u64,
        object_file_offset: u64,
        ctx: X64Context,
    ) -> Self {
        SectionIterator {
            reader,
            nsects,
            base_offset,
            object_file_offset,
            current: 0,
            ctx,
        }
    }
}

impl Iterator for SectionIterator {
    type Item = Section;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.nsects {
            return None;
        }

        let offset =
            self.base_offset + Section::size_with(&self.ctx) as u64 * u64::from(self.current);
        self.current += 1;

        if self.reader.seek(SeekFrom::Start(offset)).is_err() {
            return None;
        }

        Section::parse(&self.reader, self.ctx, self.object_file_offset).ok()
    }
}
