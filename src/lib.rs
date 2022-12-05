use std::ffi::{CString, CStr};

pub fn read_dir(path: &str) -> Result<Vec<String>, String> {
	let c_str = CString::new(path).unwrap();
	let mut result = vec![];
	let skip_vec = [".", ".."];
	unsafe {
		let dir = libc::opendir(c_str.as_ptr());
		if dir.is_null() {
			return Err("Failed to open the directory.".to_string());
		}
		loop {
			let dir_pointer = libc::readdir(dir);
			if dir_pointer.is_null() {
				break;
			}
			let name = CStr::from_ptr((*dir_pointer).d_name.as_ptr());
			let str_name_result = name.to_str();
			let str_name = str_name_result.map_err(|err| err.to_string())?;
			if !skip_vec.contains(&str_name) {
				let string_name = str_name.to_string();
				result.push(string_name);
			}
		}
		libc::closedir(dir);
	}
	Ok(result)
}
