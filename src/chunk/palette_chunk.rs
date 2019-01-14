/*
DWORD       New palette size (total number of entries)
DWORD       First color index to change
DWORD       Last color index to change
BYTE[8]     For future (set to zero)
+ For each palette entry in [from,to] range (to-from+1 entries)
  WORD      Entry flags:
			  1 = Has name
  BYTE      Red (0-255)
  BYTE      Green (0-255)
  BYTE      Blue (0-255)
  BYTE      Alpha (0-255)
  + If has name bit in entry flags
	STRING  Color name
	*/

use std::io::{self, Read, Seek, SeekFrom, Write};

use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::color::RGBA256;
use crate::helpers::{read_string, write_string};

bitflags! {
	pub struct Flags: u16 {
		const HasName = 1;
	}
}

#[derive(Debug)]
pub struct PaletteEntry {
	pub flags: Flags,
	pub color: RGBA256,
	pub color_name: Option<String>,
}

#[derive(Debug)]
pub struct PaletteChunk {
	pub new_palette_size: u32,
	pub first_color_index_to_change: u32,
	pub last_color_index_to_change: u32,
	pub palette_entries: Vec<PaletteEntry>,
}

impl PaletteChunk {
	pub fn from_read<R>(read: &mut R) -> io::Result<Self>
	where
		R: Read + Seek,
	{
		let new_palette_size = read.read_u32::<LittleEndian>()?;
		let first_color_index_to_change = read.read_u32::<LittleEndian>()?;
		let last_color_index_to_change = read.read_u32::<LittleEndian>()?;
		read.seek(SeekFrom::Current(8))?;
		let mut palette_entries = Vec::with_capacity(new_palette_size as usize);
		for _ in 0..new_palette_size {
			let flags = Flags::from_bits_truncate(read.read_u16::<LittleEndian>()?);
			let color = RGBA256 {
				r: read.read_u8()?,
				g: read.read_u8()?,
				b: read.read_u8()?,
				a: read.read_u8()?,
			};

			let color_name = if flags.contains(Flags::HasName) {
				Some(read_string(read)?)
			} else {
				None
			};

			palette_entries.push(PaletteEntry {
				flags,
				color,
				color_name,
			});
		}

		Ok(Self {
			new_palette_size,
			first_color_index_to_change,
			last_color_index_to_change,
			palette_entries,
		})
	}

	pub fn write<W>(&self, wtr: &mut W) -> io::Result<()>
	where
		W: Write + Seek,
	{
		wtr.write_u32::<LittleEndian>(self.new_palette_size)?;
		wtr.write_u32::<LittleEndian>(self.first_color_index_to_change)?;
		wtr.write_u32::<LittleEndian>(self.last_color_index_to_change)?;
		wtr.seek(SeekFrom::Current(8))?;
		for pal in &self.palette_entries {
			if pal.flags.contains(Flags::HasName) && pal.color_name.is_none() {
				return Err(io::Error::new(
					io::ErrorKind::InvalidData,
					"Flag HasName is 1 but col_name is None".to_owned()
				));
			}
			wtr.write_u16::<LittleEndian>(pal.flags.bits)?;
			wtr.write_u8(pal.color.r)?;
			wtr.write_u8(pal.color.g)?;
			wtr.write_u8(pal.color.b)?;
			wtr.write_u8(pal.color.a)?;
			if let Some(name) = &pal.color_name {
				write_string(wtr, &name)?
			}
		}
		Ok(())
	}

}
