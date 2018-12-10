use std::io::{Read, Result};

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}

pub fn from_memory<T, R: Read>(mut memory: R) -> Result<T> {
	let num_bytes = ::std::mem::size_of::<T>();
	unsafe {
		let mut s = ::std::mem::uninitialized();
		let mut buffer = slice::from_raw_parts_mut(&mut s as *mut T as *mut u8, num_bytes);
		match read.read_exact(buffer) {
			Ok(()) => Ok(s),
			Err(e) => {
				::std::mem::forget(s);
				Err(e)
			}
		}
	}
}
