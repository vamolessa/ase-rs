use std::io::{Cursor, SeekFrom, Seek, Read, Write};
use ase::*;
use std::fs;

#[test]
fn read_simple() -> std::io::Result<()> {
	let mut file = std::fs::File::open("sample_aseprite_files/simple.aseprite")?;
	Aseprite::from_read(&mut file)?;
	Ok(())
}

#[test]
fn read_layered() -> std::io::Result<()> {
	let mut file = std::fs::File::open("sample_aseprite_files/layered.aseprite")?;
	Aseprite::from_read(&mut file)?;
	Ok(())
}

#[test]
fn read_animated() -> std::io::Result<()> {
	let mut file = std::fs::File::open("sample_aseprite_files/animated.aseprite")?;
	Aseprite::from_read(&mut file)?;
	Ok(())
}

#[test]
fn read_sliced() -> std::io::Result<()> {
	let mut file = std::fs::File::open("sample_aseprite_files/sliced.aseprite")?;
	Aseprite::from_read(&mut file)?;
	Ok(())
}

#[test]
fn rw_simple() -> std::io::Result<()> {
	let mut file = std::fs::File::open("sample_aseprite_files/simple.aseprite")?;
	let mut fbuf = vec![];
	file.seek(SeekFrom::Start(0))?;
	file.read_to_end(&mut fbuf)?;
	file.seek(SeekFrom::Start(0))?;
	let ase = Aseprite::from_read(&mut file)?;

	let buf = vec![];
	let mut wtr = Cursor::new(buf);
	ase.write(&mut wtr)?;

	let buf = wtr.into_inner();

	assert_eq!(buf, fbuf);

	let mut out = fs::File::create("simple_out.aseprite")?;
	out.write_all(&buf)?;

	Ok(())
}

// #[test]
// fn rw_layered() -> std::io::Result<()> {
// 	let mut file = std::fs::File::open("sample_aseprite_files/layered.aseprite")?;
// 	Aseprite::from_read(&mut file)?;
// 	Ok(())
// }

// #[test]
// fn rw_animated() -> std::io::Result<()> {
// 	let mut file = std::fs::File::open("sample_aseprite_files/animated.aseprite")?;
// 	Aseprite::from_read(&mut file)?;
// 	Ok(())
// }

// #[test]
// fn rw_sliced() -> std::io::Result<()> {
// 	let mut file = std::fs::File::open("sample_aseprite_files/sliced.aseprite")?;
// 	Aseprite::from_read(&mut file)?;
// 	Ok(())
// }