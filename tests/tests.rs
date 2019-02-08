use ase::*;
use std::fs;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

fn test_read(fname: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::open(fname)?;
    let _ = Aseprite::from_read(&mut file)?;
    Ok(())
}

fn test_rw(fname: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::open(fname)?;
    let mut file_buf = vec![];
    file.seek(SeekFrom::Start(0))?;
    file.read_to_end(&mut file_buf)?;
    file.seek(SeekFrom::Start(0))?;
    let ase = Aseprite::from_read(&mut file)?;

    let my_buf = vec![];
    let mut wtr = Cursor::new(my_buf);
    ase.write(&mut wtr)?;
    let my_buf = wtr.into_inner();
    assert_eq!(my_buf, file_buf);

    Ok(())
}

#[test]
fn read() -> std::io::Result<()> {
    test_read("sample_aseprite_files/simple.aseprite")?;
    test_read("sample_aseprite_files/layered.aseprite")?;
    test_read("sample_aseprite_files/animated.aseprite")?;
    test_read("sample_aseprite_files/sliced.aseprite")?;
    Ok(())
}

#[test]
fn rw() -> std::io::Result<()> {
    test_rw("sample_aseprite_files/simple.aseprite")?;
    test_rw("sample_aseprite_files/layered.aseprite")?;
    test_rw("sample_aseprite_files/animated.aseprite")?;
    test_rw("sample_aseprite_files/sliced.aseprite")?;
    Ok(())
}
