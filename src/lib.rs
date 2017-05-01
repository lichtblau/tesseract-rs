extern crate tesseract_sys;
extern crate libc;

use tesseract_sys::*;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::ffi::CStr;


pub struct Tesseract {
	raw: *mut TessBaseAPI
}

impl Drop for Tesseract {
	fn drop(&mut self) {
		unsafe { TessBaseAPIDelete(self.raw) }
	}
}

fn cs(string: &str) -> CString {
	// do not call as_ptr yet, since the data will be freed before we return
	CString::new(string).unwrap()
}

impl Tesseract {
	pub fn new() -> Tesseract {
		Tesseract {
			raw: unsafe { TessBaseAPICreate() }
		}
	}
	pub fn set_lang(&self, language: &str) -> i32 {
		let cs_language = cs(language);
		unsafe { TessBaseAPIInit3(self.raw, ptr::null(), cs_language.as_ptr()) }
	}
	pub fn set_image(&self, filename: &str) {
		let cs_filename = cs(filename);
		unsafe {
			let img = pixRead(cs_filename.as_ptr());
			TessBaseAPISetImage2(self.raw, img);
		}
	}
	pub fn set_variable(&self, name: &str, value: &str) -> i32 {
		let cs_name = cs(name);
		let cs_value = cs(value);
		unsafe { TessBaseAPISetVariable(self.raw, cs_name.as_ptr(), cs_value.as_ptr()) }
	}
	pub fn recognize(&self) -> i32 {
		unsafe {
			TessBaseAPIRecognize(self.raw, ptr::null())
		}
	}
	pub fn get_text(&self) -> &str {
		unsafe {
			str::from_utf8(CStr::from_ptr(TessBaseAPIGetUTF8Text(self.raw)).to_bytes()).unwrap()
		}
	}
}

pub fn ocr(filename: &str, language: &str) -> String {
	let cube = Tesseract::new();
	cube.set_lang(language);
	cube.set_image(filename);
	cube.recognize();
	return cube.get_text().to_string()
}


#[test]
fn blah(){
	ocr("img.png", "eng");
}

#[test]
fn it_works() {
	let cube = Tesseract::new();
	cube.set_lang("eng");
	cube.set_image("img.png");
	cube.recognize();
	println!("{:?}", cube.get_text());
}
