extern crate winapi;

use std::ffi::CString;
use std::collections::HashMap;
use serde_json::json;
use winapi::um::fileapi::{GetDriveTypeA, GetLogicalDrives};
use winapi::um::winbase::DRIVE_FIXED;

fn read_all_drives() -> serde_json::Value {
    let mut drives_info = HashMap::new();
    let drives: u32 = unsafe { GetLogicalDrives() };
    if drives == 0 {
        eprintln!("Failed to get logical drives.");
        return json!({"error": "Failed to get logical drives."});
    }

    for i in 0..26 {
        if (drives & (1 << i)) != 0 {
            let drive_letter: char = (b'A' + i) as char;
            let drive_name: String = format!("{}:\\", drive_letter);
            let drive_type: u32 =
                unsafe { GetDriveTypeA(CString::new(drive_name.clone()).unwrap().as_ptr()) };

            if drive_type == DRIVE_FIXED {
                drives_info.insert(drive_letter.to_string(), drive_name);
            }
        }
    }

    json!({"fixed_drives": drives_info})
}

fn main() {
    let drives_json = read_all_drives();
    println!("{}", drives_json.to_string());
}
