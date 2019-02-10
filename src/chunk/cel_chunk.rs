use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use flate2::read::ZlibDecoder;
use std::io::prelude::*;

use crate::color::{Pixels, RGBA256};
use crate::helpers::read_bytes;
use crate::{ColorDepth, Header};

#[derive(Debug)]
pub enum Cel {
    RawCel {
        width: u16,
        height: u16,
        pixels: Pixels,
    },
    LinkedCel {
        frame_position: u16,
    },
    CompressedImage {
        width: u16,
        height: u16,
        zlib_compressed_data: Vec<u8>,
    },
}

impl Cel {
    pub fn w(&self) -> Option<u16> {
        match &self {
            Cel::CompressedImage { height, .. } => Some(*height),
            Cel::RawCel { height, .. } => Some(*height),
            _ => None,
        }
    }

    pub fn h(&self) -> Option<u16> {
        match &self {
            Cel::CompressedImage { width, .. } => Some(*width),
            Cel::RawCel { width, .. } => Some(*width),
            _ => None,
        }
    }

    pub fn pixels(&self, color_depth: &ColorDepth) -> Option<Pixels> {
        match &self {
            Cel::CompressedImage {
                zlib_compressed_data,
                ..
            } => {
                let mut d = ZlibDecoder::new(&zlib_compressed_data[..]);
                let mut s = Vec::new();
                d.read_to_end(&mut s).unwrap();
                let len = s.len() as u64;
                let mut rdr = Cursor::new(s);
                let pixels = CelChunk::read_pixels(&mut rdr, color_depth, len);
                Some(pixels.unwrap().clone())
            }
            Cel::RawCel { pixels, .. } => Some(pixels.clone()),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct CelChunk {
    pub layer_index: u16,
    pub x_position: i16,
    pub y_position: i16,
    pub opacity_level: u8,
    pub cel: Cel,
}

impl CelChunk {
    pub fn new(
        layer_index: u16,
        x: i16,
        y: i16,
        w: u16,
        h: u16,
        pixels: Pixels,
    ) -> Self {
        let cel = Cel::RawCel {
            width: w,
            height: h,
            pixels,
        };

        Self {
            layer_index,
            x_position: x,
            y_position: y,
            opacity_level: 255,
            cel,
        }
    }

    fn read_pixels<R>(
        read: &mut R,
        color_depth: &ColorDepth,
        pixels_size: u64,
    ) -> io::Result<Pixels>
    where
        R: Read + Seek,
    {
        match color_depth {
            ColorDepth::Indexed => {
                Pixels::indexed_from_read(read, pixels_size as usize)
            }
            ColorDepth::Grayscale => {
                Pixels::grayscale_from_read(read, pixels_size as usize)
            }
            ColorDepth::RGBA => {
                Pixels::rgba_from_read(read, pixels_size as usize)
            }
        }
    }

    pub fn from_read<R>(
        read: &mut R,
        chunk_data_size: u32,
        header: &Header,
    ) -> io::Result<Self>
    where
        R: Read + Seek,
    {
        let chunk_start = read.seek(SeekFrom::Current(0))?;

        let layer_index = read.read_u16::<LittleEndian>()?;
        let x_position = read.read_i16::<LittleEndian>()?;
        let y_position = read.read_i16::<LittleEndian>()?;
        let opacity_level = read.read_u8()?;
        let cel_type = read.read_u16::<LittleEndian>()?;
        read.seek(SeekFrom::Current(7))?;
        let cel = match cel_type {
            0 => {
                let width = read.read_u16::<LittleEndian>()?;
                let height = read.read_u16::<LittleEndian>()?;
                let pixels_size = chunk_start + chunk_data_size as u64
                    - read.seek(SeekFrom::Current(0))?;

                let pixels = CelChunk::read_pixels(
                    read,
                    &header.color_depth,
                    pixels_size,
                )?;
                Cel::RawCel {
                    width,
                    height,
                    pixels,
                }
            }
            1 => Cel::LinkedCel {
                frame_position: read.read_u16::<LittleEndian>()?,
            },
            2 => {
                let width = read.read_u16::<LittleEndian>()?;
                let height = read.read_u16::<LittleEndian>()?;

                let data_size = chunk_start + chunk_data_size as u64
                    - read.seek(SeekFrom::Current(0))?;
                let zlib_compressed_data =
                    read_bytes(read, data_size as usize)?;
                Cel::CompressedImage {
                    width,
                    height,
                    zlib_compressed_data,
                }
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Invalid Cel Type {}", cel_type),
                ));
            }
        };

        Ok(Self {
            layer_index,
            x_position,
            y_position,
            opacity_level,
            cel,
        })
    }

    pub fn write<W>(&self, wtr: &mut W) -> io::Result<()>
    where
        W: Write + Seek,
    {
        wtr.write_u16::<LittleEndian>(self.layer_index)?;
        wtr.write_i16::<LittleEndian>(self.x_position)?;
        wtr.write_i16::<LittleEndian>(self.y_position)?;
        wtr.write_u8(self.opacity_level)?;
        let cel_type = match self.cel {
            Cel::RawCel { .. } => 0,
            Cel::LinkedCel { .. } => 1,
            Cel::CompressedImage { .. } => 2,
        };
        wtr.write_u16::<LittleEndian>(cel_type)?;
        wtr.seek(SeekFrom::Current(7))?;
        match &self.cel {
            Cel::RawCel {
                width,
                height,
                pixels,
            } => {
                wtr.write_u16::<LittleEndian>(*width)?;
                wtr.write_u16::<LittleEndian>(*height)?;
                pixels.write(wtr)?;
            }
            Cel::LinkedCel { frame_position } => {
                wtr.write_u16::<LittleEndian>(*frame_position)?;
            }
            Cel::CompressedImage {
                width,
                height,
                zlib_compressed_data,
            } => {
                wtr.write_u16::<LittleEndian>(*width)?;
                wtr.write_u16::<LittleEndian>(*height)?;
                wtr.write(&zlib_compressed_data)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_compression() {
        use super::*;
        let data = vec![
            120, 156, 99, 96, 160, 61, 48, 72, 104, 250, 15, 194, 228, 232, 33,
            21, 83, 170, 159, 24, 243, 209, 253, 66, 11, 183, 34, 203, 17, 19,
            86, 248, 194, 153, 20, 119, 208, 42, 94, 240, 217, 75, 200, 95,
            184, 220, 70, 138, 95, 41, 177, 139, 20, 63, 80, 98, 23, 169, 97,
            70, 75, 127, 81, 219, 46, 92, 242, 212, 14, 67, 100, 49, 98, 210,
            55, 185, 118, 145, 147, 14, 168, 237, 47, 90, 165, 67, 92, 118,
            225, 83, 71, 172, 61, 196, 216, 69, 138, 89, 164, 218, 69, 110,
            153, 64, 142, 60, 41, 128, 158, 118, 161, 3, 0, 164, 249, 126, 89,
        ];
        let cel = Cel::CompressedImage {
            width: 0,
            height: 0,
            zlib_compressed_data: data,
        };

        cel.pixels(&ColorDepth::RGBA);
    }
}
