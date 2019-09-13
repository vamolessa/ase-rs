use std::io::{self, Read, Seek, SeekFrom, Write};

#[derive(Debug)]
pub struct PathChunk {}

impl PathChunk {
    pub fn from_read<R>(read: &mut R, chunk_data_size: u32) -> io::Result<Self>
    where
        R: Read + Seek,
    {
        read.seek(SeekFrom::Current(chunk_data_size as i64))?;
        Ok(Self {})
    }

    pub fn write<W>(&self, _wtr: &mut W) -> io::Result<()>
    where
        W: Write + Seek,
    {
        Ok(())
    }
}
