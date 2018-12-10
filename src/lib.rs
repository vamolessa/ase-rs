use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Result};

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

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}

pub struct Header {
	pub file_size: u32,
	pub _magic_number: u16,
	pub frames: u16,
	pub width_in_pixels: u16,
	pub height_in_pixels: u16,
	pub color_depth: u16,
	pub flags: u32,
	pub _speed: u16,
	pub _ignored0: u32,
	pub _ignored1: u32,
	pub transparent_palette_entry: u8,
	pub _ignored2: [u8; 3],
	pub number_of_colors: u16,
	pub pixel_width: u8,
	pub pixel_height: u8,
	pub _ignored3: [u8; 92],
}

impl Header {
	pub fn read<R: Read>(read: &mut R) -> Result<Self> {
		let mut header = Header {
			file_size: read.read_u32::<LittleEndian>()?,
			_magic_number: read.read_u16::<LittleEndian>()?,
			frames: read.read_u16::<LittleEndian>()?,
			width_in_pixels: read.read_u16::<LittleEndian>()?,
			height_in_pixels: read.read_u16::<LittleEndian>()?,
			color_depth: read.read_u16::<LittleEndian>()?,
			flags: read.read_u32::<LittleEndian>()?,
			_speed: read.read_u16::<LittleEndian>()?,
			_ignored0: read.read_u32::<LittleEndian>()?,
			_ignored1: read.read_u32::<LittleEndian>()?,
			transparent_palette_entry: read.read_u8()?,
			_ignored2: [read.read_u8()?, read.read_u8()?, read.read_u8()?],
			number_of_colors: read.read_u16::<LittleEndian>()?,
			pixel_width: read.read_u8()?,
			pixel_height: read.read_u8()?,
			_ignored3: [0; 92],
		};

		read.read_exact(&mut header._ignored3)?;

		Ok(header)
	}
}
