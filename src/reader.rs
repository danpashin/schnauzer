use super::result::*;
use crate::fmt_ext;
use std::{
    io::{self, BufRead, BufReader, Read, Seek, SeekFrom},
    sync::{Arc, Mutex},
};

pub trait ReadTrait: Read + Seek + Send + Sync + 'static {}

impl<T> ReadTrait for T where T: Read + Seek + Send + Sync + 'static {}

#[derive(Clone)]
pub struct Reader(Arc<Mutex<BufReader<dyn ReadTrait>>>);

impl Reader {
    pub fn new<R>(reader: R) -> Self
    where
        R: ReadTrait,
    {
        let reader = BufReader::new(reader);
        Self(Arc::new(Mutex::new(reader)))
    }

    pub fn with_lock<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut BufReader<dyn ReadTrait>) -> R,
    {
        let mut reader = self.0.lock().expect("Reader lock is posoned");
        f(&mut reader)
    }
}

impl Seek for Reader {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.with_lock(|reader| reader.seek(pos))
    }
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.with_lock(|reader| reader.read(buf))
    }
}

impl Reader {
    pub fn read_zero_terminated_string(&mut self) -> Result<String> {
        let mut buf = Vec::new();
        self.with_lock(|reader| reader.read_until(0, &mut buf))?;

        Ok(fmt_ext::printable_string(&buf))
    }
}

#[cfg(test)]
mod test {
    use super::Reader;

    fn send_sync<T: Send + Sync>() {}

    #[test]
    fn verify_reader_send_sync() {
        send_sync::<Reader>();
    }
}
