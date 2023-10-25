extern crate libc;

use libc::c_char;
//CString by rust create
use std::ffi::CString;
//CStr by c create
use std::ffi::CStr;
use std::iter;

#[no_mangle]
pub extern fn add_two_numbers(a: i32, b: i32) -> i32 {
    return a + b*2;
}

#[no_mangle]
pub extern fn text_generate(str: *const c_char,length: u8) -> *mut c_char {
    let c_str: &CStr = unsafe { CStr::from_ptr(str) };

	let mut song: String = String::from("start ");
	song.extend(iter::repeat("ha ").take(length as usize));
	song.push_str("end");
	song.push_str(c_str.to_str().unwrap());

	let c_str_song = CString::new(song).unwrap();
	c_str_song.into_raw()
}

#[no_mangle]
pub extern fn text_free(s: *mut c_char) {
	unsafe {
		if s.is_null() {return}
		let _ = CString::from_raw(s);
	};
}