use super::primitives::*;

use super::auto_enum_fields::*;
use super::reloc::*;
use super::Reader;
use super::Result;
use schnauzer_derive::AutoEnumFields;
use scroll::ctx::SizeWith;
use scroll::Endian;
use scroll::IOread;
use std::fmt::{Debug, Formatter};
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

/// Both `section` and `section_64`
#[derive(AutoEnumFields)]
pub struct Section {
    object_file_offset: u64,

    pub sectname: Str16Bytes,
    pub segname: Str16Bytes,
    pub addr: Hu64,
    pub size: Hu64,
    pub offset: u32,
    pub align: u32,
    pub reloff: u32,
    pub nreloc: u32,
    pub flags: Hu32,
    pub reserved1: u32,
    pub reserved2: u32,
    /// Only for `section_64`
    pub reserved3: u32opt,

    reader: Reader,
    pub endian: Endian,
}

impl Section {
    pub(super) fn parse(
        mut reader: Reader,
        ctx: X64Context,
        object_file_offset: u64,
    ) -> Result<Self> {
        let endian = ctx.endian().clone();

        let sectname: Str16Bytes = reader.ioread_with(endian)?;
        let segname: Str16Bytes = reader.ioread_with(endian)?;
        let addr: Hu64 = reader.ioread_with(ctx)?;
        let size: Hu64 = reader.ioread_with(ctx)?;
        let offset: u32 = reader.ioread_with(endian)?;
        let align: u32 = reader.ioread_with(endian)?;
        let reloff: u32 = reader.ioread_with(endian)?;
        let nreloc: u32 = reader.ioread_with(endian)?;
        let flags: Hu32 = reader.ioread_with(endian)?;
        let reserved1: u32 = reader.ioread_with(endian)?;
        let reserved2: u32 = reader.ioread_with(endian)?;
        let reserved3: u32opt = reader.ioread_with(ctx)?;

        Ok(Self {
            object_file_offset,
            sectname,
            segname,
            addr,
            size,
            offset,
            align,
            reloff,
            nreloc,
            flags,
            reserved1,
            reserved2,
            reserved3,
            reader,
            endian,
        })
    }
}

impl Section {
    pub fn read_data_to(&self, out: &mut dyn Write) -> Result<()> {
        use std::cmp::min;
        const BUFFER_SIZE: usize = 4096;

        let mut reader = self.reader.clone();
        reader.seek(SeekFrom::Start(
            self.object_file_offset + self.offset as u64,
        ))?;

        let mut remainig = self.size.0 as usize;

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

impl SizeWith<X64Context> for Section {
    fn size_with(ctx: &X64Context) -> usize {
        let endian = ctx.endian();

        Str16Bytes::size_with(endian) // sectname
            + Str16Bytes::size_with(endian) // segname
            + Hu64::size_with(ctx) // addr
            + Hu64::size_with(ctx) // size
            + std::mem::size_of::<u32>() // offset
            + std::mem::size_of::<u32>() // align
            + std::mem::size_of::<u32>() // reloff
            + std::mem::size_of::<u32>() // nreloc
            + Hu32::size_with(endian) // flags
            + std::mem::size_of::<u32>() // reserved1
            + std::mem::size_of::<u32>() // reserved2
            + u32opt::size_with(ctx) // reserved3
    }
}

impl Section {
    pub fn relocations_iterator(&self) -> RelocationIterator {
        RelocationIterator::new(
            self.reader.clone(),
            self.nreloc,
            self.object_file_offset + self.reloff as u64,
            self.endian,
        )
    }
}

pub struct RelocationIterator {
    reader: Reader,

    count: u32,
    base_offset: u64,
    endian: Endian,

    current: u32,
}

impl RelocationIterator {
    fn new(reader: Reader, count: u32, base_offset: u64, endian: Endian) -> Self {
        RelocationIterator {
            reader: reader,
            count: count,
            base_offset: base_offset,
            endian: endian,
            current: 0,
        }
    }
}

impl Iterator for RelocationIterator {
    type Item = RelocationInfo;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.count {
            return None;
        }

        let offset =
            self.base_offset + RelocationInfo::size_with(&self.endian) as u64 * self.current as u64;
        self.current += 1;

        let mut reader = self.reader.clone();
        if let Err(_) = reader.seek(SeekFrom::Start(offset as u64)) {
            return None;
        }

        match reader.ioread_with::<RelocationInfo>(self.endian) {
            Ok(info) => Some(info),
            Err(_) => None,
        }
    }
}

impl Debug for Section {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Section")
            .field("sectname", &self.sectname)
            .field("segname", &self.segname)
            .field("addr", &self.addr)
            .field("size", &self.size)
            .field("offset", &self.offset)
            .field("offset", &self.offset)
            .field("align", &self.align)
            .field("reloff", &self.reloff)
            .field("nreloc", &self.nreloc)
            .field("flags", &self.flags)
            .finish_non_exhaustive()
    }
}
