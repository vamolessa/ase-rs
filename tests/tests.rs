use ase::*;

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