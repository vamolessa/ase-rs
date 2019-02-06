use std::io::{self, Read, Seek, SeekFrom, Write};

use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use num_enum::CustomTryInto;

#[derive(Debug, Copy, Clone, Eq, PartialEq, CustomTryInto)]
#[repr(u16)]
pub enum ColorDepth {
	Indexed = 8,
	Grayscale = 16,
	RGBA = 32,
}

impl Default for ColorDepth {
	fn default() -> Self {
		ColorDepth::RGBA
	}
}

bitflags! {
	pub struct Flags: u32 {
		const HasOpacity = 1;
	}
}

impl Default for Flags {
	fn default() -> Self {
		Flags::HasOpacity
	}
}

#[derive(Debug)]
pub struct Header {
	pub file_size: u32,
	pub frames: u16,
	pub width_in_pixels: u16,
	pub height_in_pixels: u16,
	pub color_depth: ColorDepth,
	pub flags: Flags,
	pub speed: u16, // deprecated
	pub transparent_palette_entry: u8,
	pub number_of_colors: u16,
	pub pixel_width: u8,
	pub pixel_height: u8,
}

impl Default for Header {
	fn default() -> Self {
		Header {
			file_size: 0,
			frames: 0,
			width_in_pixels: 0,
			height_in_pixels: 0,
			color_depth: Default::default(),
			flags: Default::default(),
			speed: 100, // deprecated
			transparent_palette_entry: 0,
			number_of_colors: 32,
			pixel_width: 1,
			pixel_height: 1
		}
	}
}

impl Header {

	const MAGIC: u16 = 0xA5E0;

	pub fn new(w: u16, h: u16) -> Self {
		Header {
			width_in_pixels: w,
			height_in_pixels: h,
			..Default::default()
		}
	}


	pub fn from_read<R>(read: &mut R) -> io::Result<Self>
	where
		R: Read + Seek,
	{
		let file_size = read.read_u32::<LittleEndian>()?;
		read.seek(SeekFrom::Current(2))?;
		let frames = read.read_u16::<LittleEndian>()?;
		let width_in_pixels = read.read_u16::<LittleEndian>()?;
		let height_in_pixels = read.read_u16::<LittleEndian>()?;
		let color_depth = read
			.read_u16::<LittleEndian>()?
			.try_into_ColorDepth()
			.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
		let flags = Flags::from_bits_truncate(read.read_u32::<LittleEndian>()?);
		let speed = read.read_u16::<LittleEndian>()?;
		read.seek(SeekFrom::Current(4 + 4))?;
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
			speed,
			transparent_palette_entry,
			number_of_colors,
			pixel_width,
			pixel_height,
		})
	}

	pub fn write<W>(&self, wtr: &mut W, frame_bytes: u32, frame_len: u16) -> io::Result<()>
	where
		W: Write + Seek,
	{
		wtr.write_u32::<LittleEndian>(128 + frame_bytes)?;
		wtr.write_u16::<LittleEndian>(Header::MAGIC)?;
		wtr.write_u16::<LittleEndian>(frame_len)?;
		wtr.write_u16::<LittleEndian>(self.width_in_pixels)?;
		wtr.write_u16::<LittleEndian>(self.height_in_pixels)?;
		wtr.write_u16::<LittleEndian>(self.color_depth as u16)?;
		wtr.write_u32::<LittleEndian>(self.flags.bits)?;
		wtr.write_u16::<LittleEndian>(self.speed)?;
		wtr.seek(SeekFrom::Current(4 + 4))?;
		wtr.write_u8(self.transparent_palette_entry)?;
		wtr.seek(SeekFrom::Current(3))?;
		wtr.write_u16::<LittleEndian>(self.number_of_colors)?;
		wtr.write_u8(self.pixel_width)?;
		wtr.write_u8(self.pixel_height)?;
		wtr.seek(SeekFrom::Current(92))?;

		Ok(())
	}
}
