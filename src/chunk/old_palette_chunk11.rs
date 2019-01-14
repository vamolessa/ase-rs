use std::io::{self, Read, Write, Seek};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::color::RGB64;

pub struct Packet {
	pub palette_entries_to_skip: u8,
	pub colors: Vec<RGB64>,
}

pub struct OldPaletteChunk11 {
	pub number_of_packets: u16,
	pub packets: Vec<Packet>,
}

impl OldPaletteChunk11 {
	pub fn from_read<R>(read: &mut R) -> io::Result<Self>
	where
		R: Read,
	{
		let number_of_packets = read.read_u16::<LittleEndian>()?;
		let mut packets = Vec::new();

		for _ in 0..number_of_packets {
			let palette_entries_to_skip = read.read_u8()?;

			let mut number_of_colors = read.read_u8()? as usize;
			if number_of_colors == 0 {
				number_of_colors = 256;
			}

			let mut colors = Vec::with_capacity(number_of_colors);

			for _ in 0..number_of_colors {
				colors.push(RGB64 {
					r: read.read_u8()?,
					g: read.read_u8()?,
					b: read.read_u8()?,
				})
			}

			packets.push(Packet {
				palette_entries_to_skip,
				colors,
			});
		}

		Ok(Self {
			number_of_packets,
			packets,
		})
	}

	pub fn write<W>(&self, wtr: &mut W) -> io::Result<()>
	where
		W: Write + Seek,
	{
		wtr.write_u16::<LittleEndian>(self.number_of_packets)?;
		for packet in &self.packets {
			wtr.write_u8(packet.palette_entries_to_skip)?;
			for color in &packet.colors {
				wtr.write_u8(color.r)?;
				wtr.write_u8(color.g)?;
				wtr.write_u8(color.b)?;
			}
		}
		Ok(())
	}

}
