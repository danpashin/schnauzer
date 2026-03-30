use crate::reader::Reader;
use crate::result::Result;
use std::fmt::Debug;
use std::fmt::Display;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

/// Represents `union lc_str`
pub struct LcStr {
    pub(crate) reader: Reader,

    pub(crate) file_offset: u64,
}

impl LcStr {
    pub fn load_string(&self) -> Result<String> {
        let mut reader_mut = self.reader.clone();
        reader_mut.seek(SeekFrom::Start(self.file_offset))?;
        reader_mut.read_zero_terminated_string()
    }
}

impl Debug for LcStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.load_string();
        let s = s.as_deref().unwrap_or("<Error>");
        write!(f, "{s}")
    }
}

impl Display for LcStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.load_string();
        let s = s.as_deref().unwrap_or("");
        write!(f, "{s}")
    }
}

pub struct BitVec {
    pub(super) reader: Reader,

    pub(super) file_offset: u64,
    pub(super) bytecount: u32,
}

impl BitVec {
    pub fn load_bit_vector(&self) -> Result<Vec<u8>> {
        let mut reader_mut = self.reader.clone();
        reader_mut.seek(SeekFrom::Start(self.file_offset))?;

        let mut v = vec![0u8; self.bytecount as usize];
        reader_mut.read_exact(&mut v)?;

        Ok(v)
    }
}

impl Debug for BitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.load_bit_vector())
    }
}
