use std::io::{self, Read, Seek, Write};

mod header;
pub use self::header::*;

mod frame;
pub use self::frame::*;

mod chunk;
pub use self::chunk::*;

pub mod color;
mod helpers;

/*
https://github.com/aseprite/aseprite/blob/master/docs/ase-file-specs.md

ASE files use Intel (little-endian) byte order.

BYTE (u8): An 8-bit unsigned integer value
WORD (u16): A 16-bit unsigned integer value
SHORT (i16): A 16-bit signed integer value
DWORD (u32): A 32-bit unsigned integer value
LONG (i32): A 32-bit signed integer value
FIXED (f32): A 32-bit fixed point (16.16) value
BYTE[n] ([u8; n]): "n" bytes.
STRING:
	WORD: string length (number of bytes)
	BYTE[length]: characters (in UTF-8) The '\0' character is not included.
PIXEL: One pixel, depending on the image pixel format:
	RGBA: BYTE[4], each pixel have 4 bytes in this order Red, Green, Blue, Alpha.
	Grayscale: BYTE[2], each pixel have 2 bytes in the order Value, Alpha.
	Indexed: BYTE, Each pixel uses 1 byte (the index).
*/

pub struct Aseprite {
	pub header: Header,
	pub frames: Vec<Frame>,
}

impl Aseprite {
	pub fn from_read<R>(read: &mut R) -> io::Result<Aseprite>
	where
		R: Read + Seek,
	{
		let header = Header::from_read(read)?;
		let mut frames = Vec::with_capacity(header.frames as usize);
		for _ in 0..header.frames {
			frames.push(Frame::from_read(read, &header)?);
		}

		Ok(Self { header, frames })
	}

	pub fn write<W>(&self, wtr: &mut W) -> io::Result<()>
	where
		W: Write + Seek,
	{
		self.header.write(wtr)?;
		for frame in &self.frames {
			frame.write(wtr)?;
		}
		Ok(())
	}
}
