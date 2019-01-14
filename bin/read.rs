use ase;
use std::{fs, env};

fn main() {
    let fname = env::args().skip(1).next().unwrap();
	let mut file = fs::File::open(fname).unwrap();
    let ase = ase::Aseprite::from_read(&mut file).unwrap();
    println!("{:#?}", ase);
}