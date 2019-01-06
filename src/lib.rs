use std::io::{Read, Result};

mod header;
pub use self::header::*;

mod frame;
pub use self::frame::*;

mod chunk;
pub use self::chunk::*;

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

pub fn read<R: Read>(_read: &mut R) -> Result<()> {
	Ok(())
}
