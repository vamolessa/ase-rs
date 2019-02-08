use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{Chunk, Header};

#[derive(Debug, Default)]
pub struct Frame {
    pub byte_count: u32,
    pub frame_duration_milliseconds: u16,
    pub number_of_chunks_old: u16,
    pub number_of_chunks: u32,
    pub chunks: Vec<Chunk>,
}

impl Frame {
    /// create a new default Frame with empty chunks buffer
    pub fn new() -> Self {
        Frame {
            ..Default::default()
        }
    }

    /// add a chunk to the chunks buffer
    pub fn add_chunk(&mut self, chunk: Chunk) -> &mut Self {
        self.chunks.push(chunk);
        self
    }

    const MAGIC: u16 = 0xF1FA;

    pub fn from_read<R>(read: &mut R, header: &Header) -> io::Result<Self>
    where
        R: Read + Seek,
    {
        let byte_count = read.read_u32::<LittleEndian>()?;
        read.seek(SeekFrom::Current(2))?;
        let number_of_chunks_old = read.read_u16::<LittleEndian>()?;
        let frame_duration_milliseconds = read.read_u16::<LittleEndian>()?;
        read.seek(SeekFrom::Current(2))?;
        let number_of_chunks_new = read.read_u32::<LittleEndian>()?;

        let number_of_chunks = if number_of_chunks_new == 0 {
            number_of_chunks_old as u32
        } else {
            number_of_chunks_new
        };

        let mut chunks = Vec::with_capacity(number_of_chunks as usize);
        for _ in 0..number_of_chunks {
            chunks.push(Chunk::from_read(read, header)?);
        }

        Ok(Self {
            byte_count,
            number_of_chunks_old,
            frame_duration_milliseconds,
            number_of_chunks,
            chunks,
        })
    }

    const PREFER_OLD: bool = true;

    pub fn write<W>(&self, wtr: &mut W) -> io::Result<()>
    where
        W: Write + Seek,
    {
        let chunks_buf = vec![];
        let mut chunks_wtr = Cursor::new(chunks_buf);
        for chunk in &self.chunks {
            chunk.write(&mut chunks_wtr)?;
        }

        wtr.write_u32::<LittleEndian>(16 + chunks_wtr.position() as u32)?;
        wtr.write_u16::<LittleEndian>(Frame::MAGIC)?;

        if Frame::PREFER_OLD {
            wtr.write_u16::<LittleEndian>(self.chunks.len() as u16)?;
        } else {
            wtr.seek(SeekFrom::Current(2))?;
        }

        wtr.write_u16::<LittleEndian>(self.frame_duration_milliseconds)?;

        wtr.seek(SeekFrom::Current(2))?;

        if Frame::PREFER_OLD {
            wtr.seek(SeekFrom::Current(4))?;
        } else {
            wtr.write_u32::<LittleEndian>(self.chunks.len() as u32)?;
        }

        wtr.write(&chunks_wtr.into_inner())?;
        Ok(())
    }
}
