use std::io::{self, Read, Seek, SeekFrom};

use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt};

bitflags! {
	pub struct Flags: u32 {
		const HasOpacity = 1;
	}
}

pub struct Header {
	pub file_size: u32,
	pub frames: u16,
	pub width_in_pixels: u16,
	pub height_in_pixels: u16,
	pub color_depth: u16,
	pub flags: Flags,
	pub transparent_palette_entry: u8,
	pub number_of_colors: u16,
	pub pixel_width: u8,
	pub pixel_height: u8,
}

impl Header {
	pub fn from_read<R>(read: &mut R) -> io::Result<Self>
	where
		R: Read + Seek,
	{
		let file_size = read.read_u32::<LittleEndian>()?;
		read.seek(SeekFrom::Current(2))?;
		let frames = read.read_u16::<LittleEndian>()?;
		let width_in_pixels = read.read_u16::<LittleEndian>()?;
		let height_in_pixels = read.read_u16::<LittleEndian>()?;
		let color_depth = read.read_u16::<LittleEndian>()?;
		let flags = Flags::from_bits_truncate(read.read_u32::<LittleEndian>()?);
		read.seek(SeekFrom::Current(2 + 4 + 4))?;
		let transparent_palette_entry = read.read_u8()?;
		read.seek(SeekFrom::Current(3))?;
		let number_of_colors = read.read_u16::<LittleEndian>()?;
		let pixel_width = read.read_u8()?;
		let pixel_height = read.read_u8()?;
		read.seek(SeekFrom::Current(92))?;

		Ok(Self {
			file_size,
			frames,
			width_in_pixels,
			height_in_pixels,
			color_depth,
			flags,
			transparent_palette_entry,
			number_of_colors,
			pixel_width,
			pixel_height,
		})
	}
}
