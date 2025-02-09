extern crate winapi;

use serde_json::{json, Value};
use std::collections::HashMap;
use std::ffi::CString;
use winapi::um::fileapi::GetDiskFreeSpaceExA;
use winapi::um::fileapi::{GetDriveTypeA, GetLogicalDrives};
use winapi::um::winbase::DRIVE_FIXED;
use winapi::um::winnt::ULARGE_INTEGER;

// crate imports
use zfs_recovery_rs::drive::bytes::bytes_to_human_readable;
use zfs_recovery_rs::os_id::identify_os;

fn read_all_drives() -> Value {
    let mut drives_info: HashMap<String, Value> = HashMap::new();
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
                let drive_size = bytes_to_human_readable(get_drive_size(&drive_name));
                drives_info.insert(
                    drive_letter.to_string(),
                    json!({"name": drive_name, "size": drive_size}),
                );
            }
        }
    }

    json!({"fixed_drives": drives_info})
}

fn get_drive_size(drive_name: &str) -> u64 {
    let mut free_bytes_available: u64 = 0;
    let mut total_number_of_bytes: u64 = 0;
    let mut total_number_of_free_bytes: u64 = 0;

    let drive_name_cstr: CString = CString::new(drive_name).unwrap();

    let result: i32 = unsafe {
        GetDiskFreeSpaceExA(
            drive_name_cstr.as_ptr(),
            &mut free_bytes_available as *mut u64 as *mut ULARGE_INTEGER,
            &mut total_number_of_bytes as *mut u64 as *mut ULARGE_INTEGER,
            &mut total_number_of_free_bytes as *mut u64 as *mut ULARGE_INTEGER,
        )
    };

    if result == 0 {
        eprintln!("Failed to get drive size for {}", drive_name);
        0
    } else {
        total_number_of_bytes
    }
}

#[tokio::main]
async fn main() {
    let os: Value = identify_os().await;
    let drives_json: Value = read_all_drives();

    let combined_json = json!({
        "os": os["os"],
        "fixed_drives": drives_json["fixed_drives"]
    });

    println!(
        "{}",
        combined_json
            .to_string()
            .replace("{", "{\n    ")
            .replace("}", "\n}")
            .replace(",", ",\n    ")
    );
}
