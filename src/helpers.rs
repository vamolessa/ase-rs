use std::io::{self, Read};

use byteorder::{LittleEndian, ReadBytesExt};

pub fn read_string<R>(read: &mut R) -> io::Result<String>
where
	R: Read,
{
	let length = read.read_u16::<LittleEndian>()? as usize;
	let bytes = Vec::with_capacity(length);
	bytes.resize(length, 0);
	read.read_exact(&mut bytes[..])?;
	String::from_utf8(bytes).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
