use std::io::{self, Read};

use byteorder::{LittleEndian, ReadBytesExt};

pub enum CelType {
	RawCel { width: u16, height: u16,  },
	LinkedCel,
	CompressedImage,
}

pub struct CelChunk {
	pub layer_index: u16,
	pub x_position: i16,
	pub y_position: i16,
	pub opacity_level: u8,
	pub cel_type: u16,
}

impl CelChunk {
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
				colors.push(RGB256 {
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
}
