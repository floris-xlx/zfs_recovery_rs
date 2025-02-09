pub fn bytes_to_human_readable(bytes: u64) -> String {
    const KILOBYTE: u64 = 1024;
    const MEGABYTE: u64 = KILOBYTE * 1024;
    const GIGABYTE: u64 = MEGABYTE * 1024;
    const TERABYTE: u64 = GIGABYTE * 1024;

    if bytes >= TERABYTE {
        format!("{:.2} TB", bytes as f64 / TERABYTE as f64)
    } else if bytes >= GIGABYTE {
        format!("{:.2} GB", bytes as f64 / GIGABYTE as f64)
    } else if bytes >= MEGABYTE {
        format!("{:.2} MB", bytes as f64 / MEGABYTE as f64)
    } else if bytes >= KILOBYTE {
        format!("{:.2} KB", bytes as f64 / KILOBYTE as f64)
    } else {
        format!("{} bytes", bytes)
    }
}
