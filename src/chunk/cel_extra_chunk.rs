use std::io::{self, Read, Seek, SeekFrom, Write};

use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

bitflags! {
    pub struct Flags: u32 {
        const PreciseBounds = 1;
    }
}

#[derive(Debug)]
pub struct CelExtraChunk {
    pub flags: Flags,
    pub precise_x_position: f32,
    pub precise_y_position: f32,
    pub width: f32,
    pub height: f32,
}

impl CelExtraChunk {
    pub fn from_read<R>(read: &mut R) -> io::Result<Self>
    where
        R: Read + Seek,
    {
        let flags = Flags::from_bits_truncate(read.read_u32::<LittleEndian>()?);
        let precise_x_position = read.read_f32::<LittleEndian>()?;
        let precise_y_position = read.read_f32::<LittleEndian>()?;
        let width = read.read_f32::<LittleEndian>()?;
        let height = read.read_f32::<LittleEndian>()?;
        read.seek(SeekFrom::Current(16))?;

        Ok(Self {
            flags,
            precise_x_position,
            precise_y_position,
            width,
            height,
        })
    }

    pub fn write<W>(&self, wtr: &mut W) -> io::Result<()>
    where
        W: Write + Seek,
    {
        wtr.write_u32::<LittleEndian>(self.flags.bits)?;
        wtr.write_f32::<LittleEndian>(self.precise_x_position)?;
        wtr.write_f32::<LittleEndian>(self.precise_y_position)?;
        wtr.write_f32::<LittleEndian>(self.width)?;
        wtr.write_f32::<LittleEndian>(self.height)?;
        wtr.seek(SeekFrom::Current(16))?;
        Ok(())
    }
}
