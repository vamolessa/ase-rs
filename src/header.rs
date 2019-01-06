use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Read, Seek, SeekFrom};

pub struct Header {
	pub file_size: u32,
	pub frames: u16,
	pub width_in_pixels: u16,
	pub height_in_pixels: u16,
	pub color_depth: u16,
	pub flags: u32,
	pub transparent_palette_entry: u8,
	pub number_of_colors: u16,
	pub pixel_width: u8,
	pub pixel_height: u8,
}

impl Header {
	pub fn read<R>(stream: &mut R) -> io::Result<Self>
	where
		R: Read + Seek,
	{
		let file_size = stream.read_u32::<LittleEndian>()?;
		stream.seek(SeekFrom::Current(2))?;
		let frames = stream.read_u16::<LittleEndian>()?;
		let width_in_pixels = stream.read_u16::<LittleEndian>()?;
		let height_in_pixels = stream.read_u16::<LittleEndian>()?;
		let color_depth = stream.read_u16::<LittleEndian>()?;
		let flags = stream.read_u32::<LittleEndian>()?;
		stream.seek(SeekFrom::Current(2 + 4 + 4))?;
		let transparent_palette_entry = stream.read_u8()?;
		stream.seek(SeekFrom::Current(3))?;
		let number_of_colors = stream.read_u16::<LittleEndian>()?;
		let pixel_width = stream.read_u8()?;
		let pixel_height = stream.read_u8()?;
		stream.seek(SeekFrom::Current(92))?;

		let header = Header {
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
		};

		Ok(header)
	}
}
