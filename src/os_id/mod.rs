use std::env;
use serde_json::{json, Value};

pub async fn identify_os() -> Value {
    let os: &str = env::consts::OS;
    let os_name = match os {
        "windows" => "Windows",
        "macos" => "macOS",
        "linux" => "Linux",
        "android" => "Android",
        "ios" => "iOS",
        _ => "Unknown",
    };
    json!({"os": os_name})
}
